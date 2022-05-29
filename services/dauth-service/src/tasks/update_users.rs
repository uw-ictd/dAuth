use std::collections::HashMap;
use std::sync::Arc;

use auth_vector::types::{HresStar, Rand};

use crate::core;
use crate::data::context::DauthContext;
use crate::data::error::DauthError;
use crate::data::keys;
use crate::data::vector::AuthVectorRes;
use crate::database;
use crate::rpc::clients::{backup_network, directory};

/// Runs the update user task.
/// Iterates through user in the user update table.
/// First registers each user with the directory service,
/// then enrolls all of the user's backup networks.
pub async fn run_task(context: Arc<DauthContext>) -> Result<(), DauthError> {
    let mut transaction = context.local_context.database_pool.begin().await.unwrap();
    let user_ids = database::tasks::update_users::get_user_ids(&mut transaction).await?;
    transaction.commit().await.unwrap();

    if user_ids.is_empty() {
        tracing::debug!("Nothing to do for update user task");
    } else {
        tracing::info!("Found {} user update(s) pending", user_ids.len());

        let mut tasks = Vec::new();

        for user_id in user_ids {
            tasks.push(tokio::spawn(handle_user_update(context.clone(), user_id)));
        }

        for task in tasks {
            match task.await {
                Ok(task_res) => {
                    if let Err(e) = task_res {
                        tracing::warn!("Failed to handle user update: {}", e);
                    }
                }
                Err(je) => {
                    tracing::warn!("Error while joining: {}", je)
                }
            }
        }
    }
    Ok(())
}

/// Adds the user and its backup networks to the directory service.
/// Then, enrolls each of the backup networks.
async fn handle_user_update(context: Arc<DauthContext>, user_id: String) -> Result<(), DauthError> {
    let user_id = &user_id;

    let mut transaction = context.local_context.database_pool.begin().await.unwrap();
    let user_data =
        database::tasks::update_users::get_user_data(&mut transaction, &user_id).await?;

    let mut backup_network_ids = Vec::new();
    let mut vectors_map = HashMap::new();
    let mut shares_map = HashMap::new();

    for (backup_network_id, _) in &user_data {
        backup_network_ids.push(backup_network_id.clone());
        vectors_map.insert(backup_network_id.clone(), Vec::new());
        shares_map.insert(backup_network_id.clone(), Vec::new());
    }

    directory::upsert_user(context.clone(), &user_id, backup_network_ids.clone()).await?;

    /* create vectors and shares */
    for (backup_network_id, sqn_slice) in &user_data {
        let num_existing_vectors =
            database::vector_state::get_all_by_id(&mut transaction, user_id, backup_network_id)
                .await?
                .len() as i64;

        if num_existing_vectors > 0 {
            tracing::info!(
                "Found {} existing vector(s) for {} on {}",
                num_existing_vectors,
                user_id,
                backup_network_id
            );
        }

        // for each vector (up to max), build a new vector and set of key shares
        for _ in 0..std::cmp::max(
            0,
            context.local_context.max_backup_vectors - num_existing_vectors,
        ) {
            let (vector, seqnum) = core::auth_vectors::build_auth_vector(
                context.clone(),
                &mut transaction,
                user_id,
                *sqn_slice,
            )
            .await?;

            let (xres_star_hash, rand) = (vector.xres_star_hash.clone(), vector.rand.clone());
            let mut rng = rand_0_8::thread_rng();
            let mut shares: Vec<(HresStar, keys::KseafShare, Rand)> =
                keys::create_shares_from_kseaf(
                    &vector.kseaf,
                    backup_network_ids.len() as u8,
                    std::cmp::min(keys::TEMPORARY_CONSTANT_THRESHOLD, backup_network_ids.len() as u8),
                    &mut rng,
                )?
                .into_iter()
                .map(|key_share| (xres_star_hash, key_share, rand))
                .collect();

            vectors_map
                .get_mut(backup_network_id)
                .ok_or(DauthError::DataError("Vectors map error".to_string()))?
                .push(AuthVectorRes {
                    user_id: user_id.to_string(),
                    seqnum,
                    rand: vector.rand,
                    autn: vector.autn,
                    xres_star_hash: vector.xres_star_hash,
                });

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

    /* enroll backups */
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

        // drop rand before sending
        // TODO: allow backups to have rand? Would allow them to check res
        let key_shares = shares
            .into_iter()
            .map(|(xres_star_hash, kseaf, _rand)| (xres_star_hash.clone(), kseaf.to_owned()))
            .collect();

        backup_network::enroll_backup_commit(
            context.clone(),
            backup_network_id,
            user_id,
            &vectors,
            &key_shares,
            &address,
        )
        .await?;
    }

    database::tasks::update_users::remove(&mut transaction, &user_id).await?;
    transaction.commit().await?; // TODO: confirm a transaction this long is okay

    Ok(())
}
