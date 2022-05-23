use std::sync::Arc;
use tracing::*;

use auth_vector::{
    self,
    types::{HresStar, ResStar},
};

use crate::data::{context::DauthContext, error::DauthError, keys, state::AuthSource};
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
    res_star: auth_vector::types::ResStar,
) -> Result<auth_vector::types::Kseaf, DauthError> {
    tracing::info!("Confirming auth with res_star: {:?}", res_star);

    let (home_network_id, backup_network_ids) =
        clients::directory::lookup_user(context.clone(), user_id).await?;

    if home_network_id == context.local_context.id {
        tracing::info!("User owned by this network");
        get_confirm_key(context.clone(), res_star).await
    } else {
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

        let xres_star_hash = auth_vector::gen_xres_star_hash(&state.rand, &res_star);

        match state.source {
            AuthSource::HomeNetwork => {
                tracing::info!("Auth started from home network");
                Ok(clients::home_network::get_confirm_key(
                    context.clone(),
                    &res_star,
                    &xres_star_hash,
                    &address,
                )
                .await?)
            }
            AuthSource::BackupNetwork => {
                tracing::info!("Auth started from backup network");

                let mut key_shares = Vec::with_capacity(backup_network_ids.len());
                let mut responses = Vec::with_capacity(backup_network_ids.len());

                for backup_network_id in backup_network_ids {
                    let (backup_address, _) =
                        clients::directory::lookup_network(context.clone(), &backup_network_id)
                            .await?;

                    responses.push(tokio::spawn(clients::backup_network::get_key_share(
                        context.clone(),
                        xres_star_hash.clone(),
                        res_star.clone(),
                        backup_address.to_string(),
                    )));
                }

                for resp in responses {
                    match resp.await {
                        Ok(key_share) => match key_share {
                            Ok(share) => key_shares.push(share),
                            Err(e) => tracing::warn!("Failed to get key share: {}", e),
                        },
                        Err(e) => {
                            tracing::warn!("Failed to get key share: {}", e)
                        }
                    }
                }

                if key_shares.len() < keys::TEMPORARY_CONSTANT_THRESHOLD.into() {
                    tracing::warn!("Insufficient valid responses to compute the kseaf");
                    return Err(DauthError::ShamirShareError());
                }

                let kseaf = keys::recover_kseaf_from_shares(
                    &key_shares,
                    keys::TEMPORARY_CONSTANT_THRESHOLD,
                )?;

                Ok(kseaf)
            }
        }
    }
}

/// Gets the Kseaf value for the auth vector from this network.
/// Auth vector must have been generated by this network.
pub async fn get_confirm_key(
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

/// Stores a collection of key shares.
pub async fn store_key_shares(
    context: Arc<DauthContext>,
    user_id: &str,
    key_shares: Vec<(auth_vector::types::HresStar, auth_vector::types::Kseaf)>,
) -> Result<(), DauthError> {
    tracing::info!("Handling multiple key store: {:?}", key_shares);

    let mut transaction = context.local_context.database_pool.begin().await?;

    for (xres_star_hash, key_share) in key_shares {
        database::key_shares::add(&mut transaction, &xres_star_hash, user_id, &key_share).await?;
    }
    transaction.commit().await?;
    Ok(())
}

/// Replace the old key share if found.
/// Adds the new key share.
pub async fn replace_key_share(
    context: Arc<DauthContext>,
    old_xres_star_hash: &auth_vector::types::HresStar,
    new_xres_star_hash: &auth_vector::types::HresStar,
    new_key_share: &auth_vector::types::Kseaf,
) -> Result<(), DauthError> {
    tracing::info!(
        "Replacing key share: {:?} => {:?}",
        old_xres_star_hash,
        new_xres_star_hash
    );

    let mut transaction = context.local_context.database_pool.begin().await?;

    let user_id = database::key_shares::get_user_id(&mut transaction, old_xres_star_hash).await?;
    database::key_shares::remove(&mut transaction, old_xres_star_hash).await?;
    database::key_shares::add(
        &mut transaction,
        new_xres_star_hash,
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
    xres_star_hash: &auth_vector::types::HresStar,
    signed_request_bytes: &Vec<u8>,
) -> Result<auth_vector::types::Kseaf, DauthError> {
    tracing::info!("Handling key share get: {:?}", xres_star_hash,);

    let mut transaction = context.local_context.database_pool.begin().await?;

    let key_share = database::key_shares::get(&mut transaction, xres_star_hash)
        .await?
        .to_key_share()?;

    let user_id = database::key_shares::get_user_id(&mut transaction, xres_star_hash).await?;

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
    xres_star_hashs: Vec<&auth_vector::types::HresStar>,
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
    res_star: &ResStar,
    xres_star_hash: &HresStar,
    backup_network_id: &str,
) -> Result<(), DauthError> {
    let mut transaction = context.local_context.database_pool.begin().await?;

    let (user_id, rand) =
        database::key_share_state::get(&mut transaction, xres_star_hash, backup_network_id).await?;

    tracing::info!(
        "Key share reported used by {} for {}",
        backup_network_id,
        user_id
    );

    validate_xres_star_hash(xres_star_hash, res_star, &rand)?;

    database::key_share_state::remove(&mut transaction, xres_star_hash, backup_network_id).await?;

    transaction.commit().await?;

    Ok(())
}

/// Confirms res* is a valid preimage of xres* hash.
fn validate_xres_star_hash(
    xres_star_hash: &auth_vector::types::HresStar,
    res_star: &auth_vector::types::ResStar,
    rand: &auth_vector::types::Rand,
) -> Result<(), DauthError> {
    if xres_star_hash != &auth_vector::gen_xres_star_hash(rand, res_star) {
        Err(DauthError::DataError(
            "Provided res* does not hash to provided xres* hash".to_string(),
        ))
    } else {
        Ok(())
    }
}
