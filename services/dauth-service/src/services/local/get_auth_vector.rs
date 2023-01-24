use std::sync::Arc;
use std::time::Duration;

use auth_vector::{self, types::XResStarHash};

use crate::common;
use crate::data::{
    context::DauthContext,
    error::DauthError,
    state::{AuthSource, AuthState},
    vector::AuthVectorRes,
};
use crate::rpc::clients;

/// Attempts to get a vector in the following order of checks:
/// 1. Generate the vector locally if this is the home network
/// 2. Lookup the home network of the user and request a vector
/// 3. Request a vector from all backup networks
/// Stores auth state for 2 and 3.
#[tracing::instrument(skip(context))]
pub async fn get_auth_vector(
    context: Arc<DauthContext>,
    user_id: &str,
    network_id: &str,
    is_resync_attempt: bool,
) -> Result<AuthVectorRes, DauthError> {
    // First see if this node has key material to generate the vector itself.
    let res = common::auth_vectors::generate_local_vector(context.clone(), user_id).await;

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
