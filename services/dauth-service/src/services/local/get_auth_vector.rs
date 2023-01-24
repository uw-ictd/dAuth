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
#[tracing::instrument(skip(context), name = "local::get_auth_vector")]
pub async fn get_auth_vector(
    context: Arc<DauthContext>,
    user_id: &str,
    network_id: &str,
    is_resync_attempt: bool,
) -> Result<AuthVectorRes, DauthError> {
    tracing::info!("Getting auth vector for local authentication");

    match common::auth_vectors::generate_local_vector(context.clone(), user_id).await {
        Ok(auth_vector_res) => {
            tracing::debug!(?user_id, "Successfully generated an auth vector locally");
            return Ok(auth_vector_res);
        }
        Err(e) => {
            tracing::debug!(?e, "Failed to generate local vector");
        }
    }

    match attempt_home_network_request(context.clone(), user_id).await {
        Ok(auth_vector_res) => {
            tracing::debug!(
                ?user_id,
                "Successfully requested auth vector from home network"
            );
            return Ok(auth_vector_res);
        }
        Err(e) => {
            tracing::debug!(?e, "Failed to request an auth vector from home network");
        }
    }

    match attempt_backup_network_request(context.clone(), user_id, is_resync_attempt).await {
        Ok(auth_vector_res) => {
            tracing::debug!(
                ?user_id,
                "Successfully requested auth vector from backup networks"
            );
            return Ok(auth_vector_res);
        }
        Err(e) => {
            tracing::debug!(?e, "Failed to request an auth vector from backup networks");
        }
    }

    tracing::error!(?user_id, "Failed to acquire auth vector from any source");
    Err(DauthError::NotFoundError(
        "Failed to acquire auth vector from any source".to_string(),
    ))
}

/// Attempts to ask the user's home network for a vector.
/// If successful, returns resulting vector.
/// Otherwise, returns None.
async fn attempt_home_network_request(
    context: Arc<DauthContext>,
    user_id: &str,
) -> Result<AuthVectorRes, DauthError> {
    tracing::info!(
        ?user_id,
        "Attempting to request auth vector from home network"
    );

    // Attempt to lookup the vector from the home network directly.
    let (home_network_id, _) = clients::directory::lookup_user(&context, user_id).await?;
    let (home_address, _) = clients::directory::lookup_network(&context, &home_network_id).await?;

    match clients::home_network::get_auth_vector(
        context.clone(),
        user_id,
        &home_address,
        Duration::from_millis(100),
    )
    .await
    {
        Ok(vector) => {
            context.backup_context.auth_states.lock().await.insert(
                user_id.to_string(),
                AuthState {
                    rand: vector.rand.clone(),
                    source: AuthSource::HomeNetwork,
                    xres_star_hash: vector.xres_star_hash.clone(),
                },
            );
            Ok(vector)
        }
        Err(e) => Err(e),
    }
}

/// Attempts to ask the user's backup networks for a vector.
/// If successful, returns resulting vector.
/// Otherwise, returns None.
async fn attempt_backup_network_request(
    context: Arc<DauthContext>,
    user_id: &str,
    is_resync_attempt: bool,
) -> Result<AuthVectorRes, DauthError> {
    tracing::info!(
        ?user_id,
        "Attempting to request auth vector from backup networks"
    );

    // Attempt to lookup the vector from the home network directly.
    let (_, backup_network_ids) = clients::directory::lookup_user(&context, user_id).await?;

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
                Ok(auth_vector_result) => {
                    context.backup_context.auth_states.lock().await.insert(
                        user_id.to_string(),
                        AuthState {
                            rand: auth_vector_result.rand.clone(),
                            source: AuthSource::BackupNetwork,
                            xres_star_hash: auth_vector_result.xres_star_hash.clone(),
                        },
                    );
                    return Ok(auth_vector_result);
                }
                Err(e) => tracing::debug!("Failed to get auth from backup: {}", e),
            },
            Err(e) => tracing::debug!("Failed to get auth from backup: {}", e),
        }
    }

    Err(DauthError::NotFoundError(
        "Failed to get auth vector from backups".to_string(),
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
