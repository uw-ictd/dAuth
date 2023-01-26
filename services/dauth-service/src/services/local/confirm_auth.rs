use std::sync::Arc;

use auth_vector::{
    self,
    types::{Res, ResStar, XResHash, XResStarHash},
};

use crate::data::state::AuthState;
use crate::data::{
    combined_res::ResKind, context::DauthContext, error::DauthError, keys, state::AuthSource,
};
use crate::database;
use crate::database::utilities::DauthDataUtilities;
use crate::rpc::clients;

/// Attempts to find the Kseaf value for the network.
/// Runs the following checks:
/// 1. If generated on this network, get Kseaf from the database.
/// 2. Else, check the home network for a Kseaf value.
/// 3. If not from the home network, get key shares from the backup networks.
#[tracing::instrument(skip(context), name = "local::confirm_auth")]
pub async fn confirm_auth(
    context: Arc<DauthContext>,
    user_id: &str,
    combined_res: ResKind,
) -> Result<keys::KeyKind, DauthError> {
    tracing::info!("Confirming local authentication");

    let (home_network_id, backup_network_ids) =
        clients::directory::lookup_user(&context, user_id).await?;

    if home_network_id == context.local_context.id {
        tracing::debug!(?user_id, "User owned by this network");
        return match combined_res {
            ResKind::ResStar(res_star) => Ok(keys::KeyKind::Kseaf(
                get_confirm_key_res_star(context.clone(), res_star).await?,
            )),
            ResKind::Res(res) => Ok(keys::KeyKind::Kasme(
                get_confirm_key_res(context.clone(), res).await?,
            )),
        };
    } else {
        tracing::debug!(?user_id, ?home_network_id, "User owned by other network");

        let (address, _) = clients::directory::lookup_network(&context, &home_network_id).await?;

        let state;
        {
            let mut map = context.backup_context.auth_states.lock().await;
            state = map.remove(user_id).ok_or(DauthError::NotFoundError(
                "Could not find state for auth transaction".to_string(),
            ))?;
            // drop mutex guard
        }

        let key = match combined_res {
            ResKind::Res(res) => {
                confirm_authentication_eps(&context, &res, &state, &backup_network_ids, &address)
                    .await?
            }
            ResKind::ResStar(res_star) => {
                confirm_authentication_5g(
                    &context,
                    &res_star,
                    &state,
                    &backup_network_ids,
                    &address,
                )
                .await?
            }
        };

        Ok(key)
    }
}

/// Confirm authentication for a 5G request.
/// Checks local state to determine if the auth vector was given
/// by the user's home network or a backup network.
async fn confirm_authentication_5g(
    context: &Arc<DauthContext>,
    res_star: &ResStar,
    state: &AuthState,
    backup_network_ids: &Vec<String>,
    address: &str,
) -> Result<keys::KeyKind, DauthError> {
    tracing::info!("Confirming 5G auth");
    let xres_star_hash = auth_vector::types::gen_xres_star_hash(&state.rand, &res_star);

    let key = match state.source {
        AuthSource::HomeNetwork => {
            tracing::info!("Auth started from home network");
            clients::home_network::get_confirm_key_kseaf(
                context.clone(),
                &res_star,
                &xres_star_hash,
                &address,
            )
            .await?
        }
        AuthSource::BackupNetwork => {
            tracing::info!("Auth started from backup network");

            let mut key_shares = Vec::with_capacity(backup_network_ids.len());
            let mut request_set = tokio::task::JoinSet::new();

            let share_threshold: u8 = std::cmp::min(
                context.backup_context.backup_key_threshold,
                backup_network_ids.len() as u8,
            );

            for backup_network_id in backup_network_ids {
                request_set.spawn(kseaf_key_share_from_network_id(
                    context.clone(),
                    xres_star_hash.clone(),
                    res_star.clone(),
                    backup_network_id.to_string(),
                ));
            }

            while let Some(response_result) = request_set.join_one().await {
                match response_result {
                    Ok(key_share) => match key_share {
                        Ok(share) => {
                            key_shares.push(share);
                            if key_shares.len() >= share_threshold.into() {
                                break;
                            }
                        }
                        Err(error) => tracing::debug!(?error, "Failed to get key share"),
                    },
                    Err(error) => tracing::debug!(?error, "Failed to get key share"),
                }
            }

            if key_shares.len() < share_threshold.into() {
                tracing::error!(
                    "Insufficient valid responses, {} of {} needed to compute kseaf, auth cannot proceed",
                    key_shares.len(),
                    share_threshold
                );
                return Err(DauthError::ShamirShareError());
            }

            keys::recover_kseaf_from_shares(&key_shares, share_threshold)?
        }
    };

    Ok(keys::KeyKind::Kseaf(key))
}

/// Confirm authentication for a 4G/EPS request.
/// Checks local state to determine if the auth vector was given
/// by the user's home network or a backup network.
async fn confirm_authentication_eps(
    context: &Arc<DauthContext>,
    res: &Res,
    state: &AuthState,
    backup_network_ids: &Vec<String>,
    address: &str,
) -> Result<keys::KeyKind, DauthError> {
    tracing::info!("Confirming 4G/EPS auth");
    let xres_hash = auth_vector::types::gen_xres_hash(&state.rand, &res);

    let key = match state.source {
        AuthSource::HomeNetwork => {
            tracing::info!("Auth started from home network");
            clients::home_network::get_confirm_key_kasme(
                context.clone(),
                &res,
                &xres_hash,
                &address,
            )
            .await?
        }
        AuthSource::BackupNetwork => {
            tracing::info!("Auth started from backup network");

            let mut key_shares = Vec::with_capacity(backup_network_ids.len());
            let mut request_set = tokio::task::JoinSet::new();

            let share_threshold: u8 = std::cmp::min(
                context.backup_context.backup_key_threshold,
                backup_network_ids.len() as u8,
            );

            for backup_network_id in backup_network_ids {
                request_set.spawn(kasme_key_share_from_network_id(
                    context.clone(),
                    xres_hash.clone(),
                    res.clone(),
                    backup_network_id.to_string(),
                ));
            }

            while let Some(response_result) = request_set.join_one().await {
                match response_result {
                    Ok(key_share) => match key_share {
                        Ok(share) => {
                            key_shares.push(share);
                            if key_shares.len() >= share_threshold.into() {
                                break;
                            }
                        }
                        Err(error) => tracing::debug!(?error, "Failed to get key share"),
                    },
                    Err(error) => tracing::debug!(?error, "Failed to get key share"),
                }
            }

            if key_shares.len() < share_threshold.into() {
                tracing::error!(
                    "Insufficient valid responses, {} of {} needed to compute kasme, auth cannot proceed",
                    key_shares.len(),
                    share_threshold
                );
                return Err(DauthError::ShamirShareError());
            }

            keys::recover_kasme_from_shares(&key_shares, share_threshold)?
        }
    };

    Ok(keys::KeyKind::Kasme(key))
}

/// Gets the Kseaf value for the auth vector from this network.
/// Auth vector must have been generated by this network.
pub async fn get_confirm_key_res_star(
    context: Arc<DauthContext>,
    res_star: auth_vector::types::ResStar,
) -> Result<auth_vector::types::Kseaf, DauthError> {
    tracing::info!(?res_star, "Getting confirm key for res_star");

    let mut transaction = context.local_context.database_pool.begin().await?;

    let kseaf = database::kseafs::get(&mut transaction, &res_star)
        .await?
        .to_kseaf()?;
    database::kseafs::remove(&mut transaction, &res_star).await?;

    transaction.commit().await?;
    Ok(kseaf)
}

/// Gets the Kasmf value for the auth vector from this network.
/// Auth vector must have been generated by this network.
pub async fn get_confirm_key_res(
    context: Arc<DauthContext>,
    res: auth_vector::types::Res,
) -> Result<auth_vector::types::Kasme, DauthError> {
    tracing::info!(?res, "Getting confirm key for res");

    let mut transaction = context.local_context.database_pool.begin().await?;

    let kasme = database::kasmes::get(&mut transaction, &res)
        .await?
        .to_kasme()?;
    database::kasmes::remove(&mut transaction, &res).await?;

    transaction.commit().await?;
    Ok(kasme)
}

async fn kseaf_key_share_from_network_id(
    context: Arc<DauthContext>,
    xres_star_hash: XResStarHash,
    res_star: ResStar,
    backup_network_id: String,
) -> Result<keys::KseafShare, DauthError> {
    let (backup_address, _) =
        clients::directory::lookup_network(&context, &backup_network_id).await?;

    clients::backup_network::get_kseaf_key_share(
        context.clone(),
        xres_star_hash.clone(),
        res_star.clone(),
        backup_address.to_string(),
    )
    .await
}

async fn kasme_key_share_from_network_id(
    context: Arc<DauthContext>,
    xres_hash: XResHash,
    res: Res,
    backup_network_id: String,
) -> Result<keys::KasmeShare, DauthError> {
    let (backup_address, _) =
        clients::directory::lookup_network(&context, &backup_network_id).await?;

    clients::backup_network::get_kasme_key_share(context, xres_hash, res, backup_address).await
}
