use std::sync::Arc;
use std::time::Duration;

use auth_vector::{self, data::AuthVectorData, types::XResStarHash};
use sqlx::{Sqlite, Transaction};

use crate::data::{
    context::DauthContext,
    error::DauthError,
    state::{AuthSource, AuthState},
    vector::AuthVectorRes,
};
use crate::database;
use crate::database::utilities::DauthDataUtilities;
use crate::rpc::clients;

/// Attempts to get a vector in the following order of checks:
/// 1. Generate the vector locally if this is the home network
/// 2. Lookup the home network of the user and request a vector
/// 3. Request a vector from all backup networks
/// Stores auth state for 2 and 3.
#[tracing::instrument(skip(context))]
pub async fn find_vector(
    context: Arc<DauthContext>,
    user_id: &str,
    network_id: &str,
    is_resync_attempt: bool,
) -> Result<AuthVectorRes, DauthError> {
    // First see if this node has key material to generate the vector itself.
    let res = generate_local_vector(
        context.clone(),
        user_id,
        get_seqnum_slice(context.clone(), user_id, network_id).await?,
    )
    .await;

    if let Ok(vector) = res {
        return Ok(vector);
    }

    tracing::debug!(?res, "Unable to generate local vector");
    tracing::info!("Unable to generate vector locally, attempting to fall back to home network");

    // Attempt to lookup the vector from the home network directly.
    let (home_network_id, backup_network_ids) =
        clients::directory::lookup_user(&context, user_id).await?;

    let (home_address, _) = clients::directory::lookup_network(&context, &home_network_id).await?;

    let res = clients::home_network::get_auth_vector(
        context.clone(),
        user_id,
        &home_address,
        Duration::from_millis(100),
    )
    .await;

    if let Ok(vector) = res {
        context.backup_context.auth_states.lock().await.insert(
            user_id.to_string(),
            AuthState {
                rand: vector.rand.clone(),
                source: AuthSource::HomeNetwork,
                xres_star_hash: vector.xres_star_hash.clone(),
            },
        );
        return Ok(vector);
    }

    tracing::debug!(?res, "Unable to get vector from home network.");
    tracing::info!("Unable to get vector from home network, attempting backup networks");

    // Lookup our authentication state for this user to see if we have
    // previously sent a tuple.
    let mut resync_xres_star_hash: Option<XResStarHash> = None;

    // Only attempt to look up the prior used vector if this is a resync
    if is_resync_attempt {
        resync_xres_star_hash = context
            .backup_context
            .auth_states
            .lock()
            .await
            .get(user_id)
            .and_then(|state| Some(state.xres_star_hash));
    }

    // Attempt to lookup an auth vector from the backup networks in parallel.
    let mut request_set = tokio::task::JoinSet::new();

    for backup_network_id in backup_network_ids {
        request_set.spawn(get_auth_vector_from_network_id(
            context.clone(),
            user_id.to_string(),
            backup_network_id.to_string(),
            resync_xres_star_hash,
        ));
    }

    while let Some(response_result) = request_set.join_one().await {
        match response_result {
            Ok(response) => match response {
                Ok(vector) => {
                    context.backup_context.auth_states.lock().await.insert(
                        user_id.to_string(),
                        AuthState {
                            rand: vector.rand.clone(),
                            source: AuthSource::BackupNetwork,
                            xres_star_hash: vector.xres_star_hash.clone(),
                        },
                    );
                    return Ok(vector);
                }
                Err(e) => tracing::debug!("Failed to get auth from single backup: {}", e),
            },
            Err(e) => tracing::debug!("Failed to get auth from single backup: {}", e),
        }
    }

    tracing::error!(
        ?user_id,
        "No auth vector found, authentication cannot proceed"
    );
    Err(DauthError::NotFoundError(
        "No auth vector found".to_string(),
    ))
}

async fn get_auth_vector_from_network_id(
    context: Arc<DauthContext>,
    user_id: String,
    backup_network_id: String,
    resync_xres_star_hash: Option<XResStarHash>,
) -> Result<AuthVectorRes, DauthError> {
    let (backup_address, _) =
        clients::directory::lookup_network(&context, &backup_network_id).await?;

    clients::backup_network::get_auth_vector(
        context.clone(),
        &user_id,
        &backup_address,
        resync_xres_star_hash,
    )
    .await
}

/// Generates an auth vector that will be verified locally.
/// Stores the kseaf directly, without key shares.
async fn generate_local_vector(
    context: Arc<DauthContext>,
    user_id: &str,
    sqn_slice: i64,
) -> Result<AuthVectorRes, DauthError> {
    tracing::info!("Attempting to generate new vector locally");

    let mut transaction = context.local_context.database_pool.begin().await?;

    let (auth_vector_data, seqnum) =
        build_auth_vector(context.clone(), &mut transaction, &user_id, 0).await?;

    let av_response = AuthVectorRes {
        user_id: user_id.to_string(),
        seqnum,
        rand: auth_vector_data.rand,
        autn: auth_vector_data.autn,
        xres_star_hash: auth_vector_data.xres_star_hash,
        xres_hash: auth_vector_data.xres_hash,
    };

    database::kseafs::add(
        &mut transaction,
        &auth_vector_data.xres_star,
        &auth_vector_data.kseaf,
    )
    .await?;

    database::kasmes::add(
        &mut transaction,
        &auth_vector_data.xres,
        &auth_vector_data.kasme,
    )
    .await?;

    tracing::info!("Auth vector generated: {:?}", av_response);
    transaction.commit().await?;

    Ok(av_response)
}

/// Builds an auth vector and updates the user state.
/// Returns the auth vector and seqnum values.
async fn build_auth_vector(
    context: Arc<DauthContext>,
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &str,
    sqn_slice: i64,
) -> Result<(AuthVectorData, i64), DauthError> {
    let mut user_info = database::user_infos::get(transaction, &user_id.to_string(), sqn_slice)
        .await?
        .to_user_info()?;

    tracing::info!(?user_id, ?sqn_slice, "sqn"=?user_info.sqn, "Generating Vector for user");

    let auth_vector_data = auth_vector::generate_vector(
        &context.local_context.mcc,
        &context.local_context.mnc,
        &user_info.k,
        &user_info.opc,
        &user_info.sqn.try_into()?,
    );

    user_info.sqn += context.local_context.num_sqn_slices;

    database::user_infos::upsert(
        transaction,
        &user_id.to_string(),
        &user_info.k,
        &user_info.opc,
        user_info.sqn,
        sqn_slice,
    )
    .await?;

    Ok((auth_vector_data, user_info.sqn))
}

/// Gets the seqnum slice assigned to a backup network for a user.
async fn get_seqnum_slice(
    context: Arc<DauthContext>,
    user_id: &str,
    network_id: &str,
) -> Result<i64, DauthError> {
    if network_id == context.local_context.id {
        Ok(0)
    } else {
        let mut transaction = context.local_context.database_pool.begin().await?;
        let slice =
            database::backup_networks::get_slice(&mut transaction, user_id, network_id).await?;
        transaction.commit().await?;
        Ok(slice)
    }
}
