use std::sync::Arc;
use tracing::*;

use auth_vector::{
    self,
    types::{XResStarHash, ResStar, Res, XResHash},
};

use crate::data::state::AuthState;
use crate::data::{context::DauthContext, error::DauthError, keys, state::AuthSource, combined_res::ResKind, combined_res::XResHashKind};
use crate::database;
use crate::database::utilities::DauthDataUtilities;
use crate::rpc::clients;

/// Attempts to find the Kseaf value for the network.
/// Runs the following checks:
/// 1. If generated on this network, get Kseaf from the database.
/// 2. Else, check the home network for a Kseaf value.
/// 3. If not from the home network, get key shares from the backup networks.
#[instrument(level = "debug")]
pub async fn confirm_authentication(
    context: Arc<DauthContext>,
    user_id: &str,
    combined_res: ResKind,
) -> Result<keys::KeyKind, DauthError> {
    tracing::info!("Confirming auth with res: {:?}", combined_res);

    let (home_network_id, backup_network_ids) =
        clients::directory::lookup_user(context.clone(), user_id).await?;

    if home_network_id == context.local_context.id {
        tracing::info!("User owned by this network");
        return match combined_res {
            ResKind::ResStar(res_star) => Ok(keys::KeyKind::Kseaf(get_confirm_key_res_star(context.clone(), res_star).await?)),
            ResKind::Res(res) => Ok(keys::KeyKind::Kasme(get_confirm_key_res(context.clone(), res).await?)),
        };
    }

    let (address, _) =
        clients::directory::lookup_network(context.clone(), &home_network_id).await?;

    let state;

    {
        let mut map = context.backup_context.auth_states.lock().await;
        state = map.remove(user_id).ok_or(DauthError::NotFoundError(
            "Could not find state for auth transaction".to_string(),
        ))?;
        // drop mutex guard
    }

    let key = match combined_res {
        ResKind::Res(r) => {
            confirm_authentication_eps(&context, user_id, &r, &state, &backup_network_ids, &address).await?
        },
        ResKind::ResStar(r) => {
            confirm_authentication_5g(&context, user_id, &r, &state, &backup_network_ids, &address).await?
        }
    };

    Ok(key)


}

async fn confirm_authentication_5g(
    context: &Arc<DauthContext>,
    user_id: &str,
    res_star: &ResStar,
    state: &AuthState,
    backup_network_ids: &Vec<String>,
    address: &str,
) -> Result<keys::KeyKind, DauthError> {
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
                        Err(e) => tracing::warn!("Failed to get key share: {}", e),
                    },
                    Err(e) => {
                        tracing::warn!("Failed to get key share: {}", e)
                    }
                }
            }

            if key_shares.len() < share_threshold.into() {
                tracing::warn!(
                    "Insufficient valid responses {} of {} needed to compute the kseaf",
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

async fn confirm_authentication_eps(
    context: &Arc<DauthContext>,
    user_id: &str,
    res: &Res,
    state: &AuthState,
    backup_network_ids: &Vec<String>,
    address: &str,
) -> Result<keys::KeyKind, DauthError> {
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
                        Err(e) => tracing::warn!("Failed to get key share: {}", e),
                    },
                    Err(e) => {
                        tracing::warn!("Failed to get key share: {}", e)
                    }
                }
            }

            if key_shares.len() < share_threshold.into() {
                tracing::warn!(
                    "Insufficient valid responses {} of {} needed to compute the kseaf",
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



async fn kseaf_key_share_from_network_id(
    context: Arc<DauthContext>,
    xres_star_hash: XResStarHash,
    res_star: ResStar,
    backup_network_id: String,
) -> Result<keys::KseafShare, DauthError> {
    let (backup_address, _) =
        clients::directory::lookup_network(context.clone(), &backup_network_id).await?;

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
        clients::directory::lookup_network(context.clone(), &backup_network_id).await?;

    clients::backup_network::get_kasme_key_share(
        context,
        xres_hash,
        res,
        backup_address,
    )
    .await
}

/// Gets the Kseaf value for the auth vector from this network.
/// Auth vector must have been generated by this network.
pub async fn get_confirm_key_res_star(
    context: Arc<DauthContext>,
    res_star: auth_vector::types::ResStar,
) -> Result<auth_vector::types::Kseaf, DauthError> {
    tracing::info!("Getting confirm key for res_star: {:?}", res_star);

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
    tracing::info!("Getting confirm key for res: {:?}", res);

    let mut transaction = context.local_context.database_pool.begin().await?;

    let kasme = database::kasmes::get(&mut transaction, &res)
        .await?
        .to_kasme()?;
    database::kasmes::remove(&mut transaction, &res).await?;

    transaction.commit().await?;
    Ok(kasme)
}

/// Stores a collection of key shares.
pub async fn store_key_shares(
    context: Arc<DauthContext>,
    user_id: &str,
    key_shares: Vec<keys::CombinedKeyShare>,
) -> Result<(), DauthError> {
    tracing::info!("Handling multiple key store: {:?}", key_shares);

    let mut transaction = context.local_context.database_pool.begin().await?;

    for share in key_shares {
        database::key_shares::add(
            &mut transaction,
            user_id,
            &share,
        )
        .await?;
    }
    transaction.commit().await?;
    Ok(())
}

/// Replace the old key share if found.
/// Adds the new key share.
pub async fn replace_key_share(
    context: Arc<DauthContext>,
    old_xres_star_hash: &auth_vector::types::XResStarHash,
    new_key_share: &keys::CombinedKeyShare,
) -> Result<(), DauthError> {
    tracing::info!(
        "Replacing key share: {:?} => {:?}",
        old_xres_star_hash,
        new_key_share,
    );

    let mut transaction = context.local_context.database_pool.begin().await?;

    let user_id = database::key_shares::get_user_id(&mut transaction, old_xres_star_hash).await?;
    database::key_shares::remove(&mut transaction, old_xres_star_hash).await?;
    database::key_shares::add(
        &mut transaction,
        &user_id,
        new_key_share,
    )
    .await?;

    transaction.commit().await?;

    Ok(())
}

/// Returns a key share value corresponding to the xres* hash.
pub async fn get_key_share(
    context: Arc<DauthContext>,
    xres_star_hash: &auth_vector::types::XResStarHash,
    signed_request_bytes: &Vec<u8>,
) -> Result<keys::CombinedKeyShare, DauthError> {
    tracing::info!("Handling key share get: {:?}", xres_star_hash,);

    let mut transaction = context.local_context.database_pool.begin().await?;

    let key_share = database::key_shares::get_from_xres_star_hash(&mut transaction, xres_star_hash)
        .await?;

    let user_id = database::key_shares::get_user_id(&mut transaction, xres_star_hash).await?;

    // Remove the auth vectors at the point we have confirmed they were used.
    database::flood_vectors::remove(&mut transaction, &user_id, xres_star_hash).await?;
    database::auth_vectors::remove(&mut transaction, &user_id, xres_star_hash).await?;

    database::tasks::report_key_shares::add(
        &mut transaction,
        xres_star_hash,
        &user_id,
        signed_request_bytes,
    )
    .await?;

    transaction.commit().await?;

    Ok(key_share)
}

/// Removes all key shares.
/// On failure, removes none.
pub async fn remove_key_shares(
    context: Arc<DauthContext>,
    xres_star_hashs: Vec<&auth_vector::types::XResStarHash>,
) -> Result<(), DauthError> {
    tracing::info!("Handling key shares remove: {:?}", xres_star_hashs,);

    let mut transaction = context.local_context.database_pool.begin().await?;
    for xres_star_hash in xres_star_hashs {
        database::key_shares::remove(&mut transaction, xres_star_hash).await?;
    }
    transaction.commit().await?;
    Ok(())
}

/// Handles a key share that was generated by this network and used
/// by a backup network.
pub async fn key_share_used(
    context: Arc<DauthContext>,
    response: &ResKind,
    xresponse_hash: &XResHashKind,
    backup_network_id: &str,
) -> Result<(), DauthError> {
    tracing::info!("Key share reported used by {}", backup_network_id);

    let mut transaction = context.local_context.database_pool.begin().await?;

    let state = match xresponse_hash {
        XResHashKind::XResStarHash(xres_star_hash) => {
            database::key_share_state::get_by_xres_star_hash(&mut transaction, xres_star_hash, backup_network_id).await?
        },
        XResHashKind::XResHash(xres_hash) => {
            database::key_share_state::get_by_xres_hash(&mut transaction, xres_hash, backup_network_id).await?
        }
    };

    if state.is_none() {
        tracing::warn!("Key share use reported with no corresponding share state");
        return Ok(());
    }

    let state = state.unwrap();
    tracing::info!(
        "Key share reported used by {} mapped to user {}",
        backup_network_id,
        state.user_id
    );

    match xresponse_hash {
        XResHashKind::XResStarHash(xres_star_hash) => {
            if let ResKind::ResStar(res_star) = response {
                validate_xres_star_hash(xres_star_hash, res_star, &state.rand)?;
                database::key_share_state::remove_by_xres_star_hash(&mut transaction, xres_star_hash, backup_network_id).await?;
            } else {
                return Err(DauthError::DataError(
                    "Provided xres* with no res*".to_string(),
                ));
            }
        },
        XResHashKind::XResHash(xres_hash) => {
            if let ResKind::Res(res) = response {
                validate_xres_hash(xres_hash, res, &state.rand)?;
                database::key_share_state::remove_by_xres_hash(&mut transaction, xres_hash, backup_network_id).await?;
            } else {
                return Err(DauthError::DataError(
                    "Provided xres with no res".to_string(),
                ));
            }
        }
    };



    transaction.commit().await?;

    Ok(())
}

/// Confirms res* is a valid preimage of xres* hash.
fn validate_xres_star_hash(
    xres_star_hash: &auth_vector::types::XResStarHash,
    res_star: &auth_vector::types::ResStar,
    rand: &auth_vector::types::Rand,
) -> Result<(), DauthError> {
    if xres_star_hash != &auth_vector::types::gen_xres_star_hash(rand, res_star) {
        Err(DauthError::DataError(
            "Provided res* does not hash to provided xres* hash".to_string(),
        ))
    } else {
        Ok(())
    }
}

/// Confirms res* is a valid preimage of xres* hash.
fn validate_xres_hash(
    xres_hash: &auth_vector::types::XResHash,
    res: &auth_vector::types::Res,
    rand: &auth_vector::types::Rand,
) -> Result<(), DauthError> {
    if xres_hash != &auth_vector::types::gen_xres_hash(rand, res) {
        Err(DauthError::DataError(
            "Provided res* does not hash to provided xres* hash".to_string(),
        ))
    } else {
        Ok(())
    }
}
