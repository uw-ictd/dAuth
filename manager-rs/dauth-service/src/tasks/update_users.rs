use std::collections::HashMap;
use std::sync::Arc;

use auth_vector::constants::KSEAF_LENGTH;
use auth_vector::types::{HresStar, Kseaf, Rand};

use crate::data::context::DauthContext;
use crate::data::error::DauthError;
use crate::data::vector::AuthVectorRes;
use crate::rpc::clients::{backup_network, directory};
use crate::{database, manager};

/// Runs the update user task.
/// Iterates through user in the user update table.
/// First registers each user with the directory service,
/// then enrolls all of the user's backup networks.
pub async fn run_task(context: Arc<DauthContext>) -> Result<(), DauthError> {
    let mut transaction = context.local_context.database_pool.begin().await.unwrap();
    let user_ids = database::tasks::update_users::get_user_ids(&mut transaction).await?;
    transaction.commit().await.unwrap();

    if user_ids.is_empty() {
        tracing::info!("Nothing to do for update user task");
    } else {
        tracing::info!("Found {} user update(s) pending", user_ids.len());
        for user_id in user_ids {
            if let Err(e) = handle_user_update(context.clone(), &user_id).await {
                tracing::warn!("Failed to handle user update: {}", e);
                // move on to next user id
            }
        }
    }
    Ok(())
}

/// Adds the user and its backup networks to the directory service.
/// Then, enrolls each of the backup networks.
async fn handle_user_update(context: Arc<DauthContext>, user_id: &str) -> Result<(), DauthError> {
    let mut transaction = context.local_context.database_pool.begin().await.unwrap();
    let user_data =
        database::tasks::update_users::get_user_data(&mut transaction, &user_id).await?;
    transaction.commit().await.unwrap();

    let mut backup_network_ids = Vec::new();
    let mut vectors_map = HashMap::new();
    let mut shares_map = HashMap::new();

    for (backup_network_id, _) in &user_data {
        backup_network_ids.push(backup_network_id.clone());
        vectors_map.insert(backup_network_id.clone(), Vec::new());
        shares_map.insert(backup_network_id.clone(), Vec::new());
    }

    directory::upsert_user(context.clone(), &user_id, backup_network_ids.clone()).await?;

    for (backup_network_id, sqn_slice) in &user_data {
        let mut transaction = context.local_context.database_pool.begin().await.unwrap();
        let num_existing_vectors =
            database::vector_state::get_all_by_id(&mut transaction, user_id, backup_network_id)
                .await?
                .len() as i64;
        transaction.commit().await.unwrap();

        if num_existing_vectors > 0 {
            tracing::info!(
                "Found {} existing vector(s) for {} on {}",
                num_existing_vectors,
                user_id,
                backup_network_id
            );
        }

        for _ in 0..std::cmp::max(
            0,
            context.local_context.max_backup_vectors - num_existing_vectors,
        ) {
            let vector =
                manager::generate_auth_vector(context.clone(), user_id, *sqn_slice).await?;
            let mut shares =
                generate_key_shares(context.clone(), &vector, backup_network_ids.len() - 1).await?;

            vectors_map
                .get_mut(backup_network_id)
                .ok_or(DauthError::DataError("Vectors map error".to_string()))?
                .push(vector);

            for other_id in &backup_network_ids {
                if other_id != backup_network_id {
                    shares_map
                        .get_mut(other_id)
                        .ok_or(DauthError::DataError("Shares map error".to_string()))?
                        .push(shares.pop().ok_or(DauthError::DataError(
                            "Shares list out of shares".to_string(),
                        ))?);
                }
            }

            if !shares.is_empty() {
                tracing::warn!("{} unused share(s) after share generation", shares.len())
            }
        }
    }

    // TODO: Handle cleanup after failure
    for (backup_network_id, seqnum_slice) in &user_data {
        let (address, _) = directory::lookup_network(context.clone(), &backup_network_id).await?;

        let vectors = vectors_map
            .get(backup_network_id)
            .ok_or(DauthError::DataError("Failed to get vectors".to_string()))?;
        let shares = shares_map
            .get(backup_network_id)
            .ok_or(DauthError::DataError("Failed to get shares".to_string()))?;

        backup_network::enroll_backup_prepare(
            context.clone(),
            user_id,
            backup_network_id,
            &address,
        )
        .await?;

        let mut transaction = context.local_context.database_pool.begin().await?;
        database::backup_networks::upsert(
            &mut transaction,
            user_id,
            backup_network_id,
            *seqnum_slice,
        )
        .await?;
        for vector in vectors {
            database::vector_state::add(
                &mut transaction,
                &vector.xres_star_hash,
                user_id,
                backup_network_id,
            )
            .await?;
        }
        for (xres_star_hash, _, rand) in shares {
            database::key_share_state::add(
                &mut transaction,
                xres_star_hash,
                backup_network_id,
                user_id,
                rand,
            )
            .await?;
        }
        transaction.commit().await?;

        // drop rand before sending
        // TODO: allow backups to have rand? Would allow them to check res
        let key_shares = shares.into_iter().map(|(xres_star_hash, kseaf, _rand)| { (xres_star_hash.clone(), kseaf.clone())}).collect();

        backup_network::enroll_backup_commit(
            context.clone(),
            backup_network_id,
            user_id,
            vectors,
            &key_shares,
            &address,
        )
        .await?;
    }

    let mut transaction = context.local_context.database_pool.begin().await?;
    database::tasks::update_users::remove(&mut transaction, &user_id).await?;
    transaction.commit().await?;

    Ok(())
}

/// Placeholder function for generating key shares
async fn generate_key_shares(
    _context: Arc<DauthContext>,
    vector: &AuthVectorRes,
    num_slices: usize,
) -> Result<Vec<(HresStar, Kseaf, Rand)>, DauthError> {
    let mut slices = Vec::new();

    for _ in 0..num_slices {
        slices.push((vector.xres_star_hash.clone(), [0u8; KSEAF_LENGTH], vector.rand.clone()))
    }

    Ok(slices)
}
