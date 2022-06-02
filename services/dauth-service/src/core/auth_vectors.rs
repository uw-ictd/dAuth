use std::sync::Arc;

use auth_vector::{self, data::AuthVectorData};
use sqlx::{Sqlite, Transaction};

use crate::core;
use crate::data::{
    context::DauthContext,
    error::DauthError,
    keys,
    state::{AuthSource, AuthState},
    vector::{AuthVectorReq, AuthVectorRes},
};
use crate::database;
use crate::database::utilities::DauthDataUtilities;
use crate::rpc::clients;

/// Attempts to get a vector in the following order of checks:
/// 1. Generate the vector locally if this is the home network
/// 2. Lookup the home network of the user and request a vector
/// 3. Request a vector from all backup networks
/// Stores auth state for 2 and 3.
pub async fn find_vector(
    context: Arc<DauthContext>,
    user_id: &str,
    network_id: &str,
) -> Result<AuthVectorRes, DauthError> {
    tracing::info!("Attempting to find a vector: {}-{}", user_id, network_id);

    if let Ok(vector) = generate_local_vector(
        context.clone(),
        user_id,
        core::users::get_seqnum_slice(context.clone(), user_id, network_id).await?,
    )
    .await
    {
        Ok(vector)
    } else {
        let (home_network_id, backup_network_ids) =
            clients::directory::lookup_user(context.clone(), user_id).await?;

        let (home_address, _) =
            clients::directory::lookup_network(context.clone(), &home_network_id).await?;
        if let Ok(vector) =
            clients::home_network::get_auth_vector(context.clone(), user_id, &home_address).await
        {
            context.backup_context.auth_states.lock().await.insert(
                user_id.to_string(),
                AuthState {
                    rand: vector.rand.clone(),
                    source: AuthSource::HomeNetwork,
                },
            );
            Ok(vector)
        } else {
            for backup_network_id in backup_network_ids {
                let (backup_address, _) =
                    clients::directory::lookup_network(context.clone(), &backup_network_id).await?;
                if let Ok(vector) = clients::backup_network::get_auth_vector(
                    context.clone(),
                    user_id,
                    &backup_address,
                )
                .await
                {
                    context.backup_context.auth_states.lock().await.insert(
                        user_id.to_string(),
                        AuthState {
                            rand: vector.rand.clone(),
                            source: AuthSource::BackupNetwork,
                        },
                    );
                    return Ok(vector);
                }
            }
            tracing::warn!("No auth vector found");
            Err(DauthError::NotFoundError(
                "No auth vector found".to_string(),
            ))
        }
    }
}

/// Generates an auth vector that will be verified locally.
/// Stores the kseaf directly, without key shares.
pub async fn generate_local_vector(
    context: Arc<DauthContext>,
    user_id: &str,
    sqn_slice: i64,
) -> Result<AuthVectorRes, DauthError> {
    tracing::info!("Generating new vector: {}:{}", user_id, sqn_slice);

    let mut transaction = context.local_context.database_pool.begin().await?;

    let (auth_vector_data, seqnum) =
        build_auth_vector(context.clone(), &mut transaction, &user_id, 0).await?;

    let av_response = AuthVectorRes {
        user_id: user_id.to_string(),
        seqnum,
        rand: auth_vector_data.rand,
        autn: auth_vector_data.autn,
        xres_star_hash: auth_vector_data.xres_star_hash,
    };

    database::kseafs::add(
        &mut transaction,
        &auth_vector_data.xres_star,
        &auth_vector_data.kseaf,
    )
    .await?;

    tracing::info!("Auth vector generated: {:?}", av_response);
    transaction.commit().await?;

    Ok(av_response)
}

// Store a new auth vector as a backup.
pub async fn store_backup_auth_vector(
    context: Arc<DauthContext>,
    av_result: &AuthVectorRes,
) -> Result<(), DauthError> {
    tracing::info!("Storing auth vector: {:?}", av_result);

    let mut transaction = context.local_context.database_pool.begin().await?;

    database::auth_vectors::add(
        &mut transaction,
        &av_result.user_id,
        av_result.seqnum,
        &av_result.xres_star_hash,
        &av_result.autn,
        &av_result.rand,
    )
    .await?;

    transaction.commit().await?;
    Ok(())
}

/// Store all auth vectors in the set.
/// Stores all or none on failure.
pub async fn store_backup_auth_vectors(
    context: Arc<DauthContext>,
    av_results: Vec<AuthVectorRes>,
) -> Result<(), DauthError> {
    tracing::info!("Storing auth vectors: {:?}", av_results);

    let mut transaction = context.local_context.database_pool.begin().await?;

    for av_result in av_results {
        database::auth_vectors::add(
            &mut transaction,
            &av_result.user_id,
            av_result.seqnum,
            &av_result.xres_star_hash,
            &av_result.autn,
            &av_result.rand,
        )
        .await?;
    }

    transaction.commit().await?;
    Ok(())
}

// Store a new flood vector as a backup.
// Will be used before any normal auth vectors.
pub async fn store_backup_flood_vector(
    context: Arc<DauthContext>,
    av_result: &AuthVectorRes,
) -> Result<(), DauthError> {
    tracing::info!("Storing flood vector: {:?}", av_result);

    let mut transaction = context.local_context.database_pool.begin().await?;

    database::flood_vectors::add(
        &mut transaction,
        &av_result.user_id,
        av_result.seqnum,
        &av_result.xres_star_hash,
        &av_result.autn,
        &av_result.rand,
    )
    .await?;

    transaction.commit().await?;
    Ok(())
}

/// Returns the next backup auth vector.
/// Checks flood vectors first, then auth vector.
/// Returns auth vector with lowest sequence number.
pub async fn next_backup_auth_vector(
    context: Arc<DauthContext>,
    av_request: &AuthVectorReq,
    signed_request_bytes: &Vec<u8>,
) -> Result<AuthVectorRes, DauthError> {
    tracing::info!("Vector next: {:?}", av_request);

    let mut transaction = context.local_context.database_pool.begin().await?;

    // Check for a flood vector first
    let vector;
    if let Ok(Some(flood_row)) =
        database::flood_vectors::get_first(&mut transaction, &av_request.user_id).await
    {
        vector = flood_row.to_auth_vector()?;

        database::flood_vectors::remove(&mut transaction, &vector.user_id, vector.seqnum).await?;

        tracing::info!("Flood vector found: {:?}", vector);
    } else {
        vector = database::auth_vectors::get_first(&mut transaction, &av_request.user_id)
            .await?
            .to_auth_vector()?;
        
        database::auth_vectors::remove(&mut transaction, &vector.user_id, vector.seqnum).await?;

        tracing::info!("Backup vector found: {:?}", vector);
    };

    database::tasks::report_auth_vectors::add(
        &mut transaction,
        &vector.xres_star_hash,
        &vector.user_id,
        signed_request_bytes,
    )
    .await?;

    transaction.commit().await?;

    Ok(vector)
}

/// Report that a vector given to a backup network has been used.
/// Returns a new vector to replace the used vector.
/// Sends new key shares to all other backup networks for the same user.
pub async fn backup_auth_vector_used(
    context: Arc<DauthContext>,
    backup_network_id: &str,
    xres_star_hash: &auth_vector::types::HresStar,
) -> Result<AuthVectorRes, DauthError> {
    tracing::info!(
        "Auth vector used on {:?}: {:?}",
        backup_network_id,
        xres_star_hash
    );

    let mut transaction = context.local_context.database_pool.begin().await?;
    let (owning_network_id, user_id) =
        database::vector_state::get(&mut transaction, xres_star_hash).await?;

    if owning_network_id != backup_network_id {
        return Err(DauthError::DataError("Not the owning network".to_string()));
    }

    database::vector_state::remove(&mut transaction, xres_star_hash).await?;

    let seqnum_slice =
        database::backup_networks::get_slice(&mut transaction, &user_id, backup_network_id).await?;

    let (auth_vector_data, seqnum) =
        build_auth_vector(context.clone(), &mut transaction, &user_id, seqnum_slice).await?;

    database::vector_state::add(
        &mut transaction,
        &auth_vector_data.xres_star_hash,
        &user_id,
        backup_network_id,
    )
    .await?;

    let (_, mut backup_networks) =
        clients::directory::lookup_user(context.clone(), &user_id).await?;
    backup_networks.retain(|id| id != backup_network_id);

    let mut key_shares = keys::create_shares_from_kseaf(
        &auth_vector_data.kseaf,
        backup_networks.len() as u8,
        std::cmp::min(keys::TEMPORARY_CONSTANT_THRESHOLD, backup_networks.len() as u8),
        &mut rand_0_8::thread_rng(),
    )?;

    for id in backup_networks {
        let key_share = key_shares.pop().ok_or(DauthError::DataError(
            "Failed to generate all key shares".to_string(),
        ))?;
        database::tasks::replace_key_shares::add(
            &mut transaction,
            &id,
            &auth_vector_data.xres_star_hash,
            xres_star_hash,
            &key_share,
        )
        .await?;
    }

    transaction.commit().await?;

    Ok(AuthVectorRes {
        user_id: user_id.to_string(),
        seqnum,
        rand: auth_vector_data.rand,
        autn: auth_vector_data.autn,
        xres_star_hash: auth_vector_data.xres_star_hash,
    })
}

/// Builds an auth vector and updates the user state.
/// Returns the auth vector and seqnum values.
pub async fn build_auth_vector(
    context: Arc<DauthContext>,
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &str,
    sqn_slice: i64,
) -> Result<(AuthVectorData, i64), DauthError> {
    let mut user_info = database::user_infos::get(transaction, &user_id.to_string(), sqn_slice)
        .await?
        .to_user_info()?;

    let auth_vector_data = auth_vector::generate_vector(
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
