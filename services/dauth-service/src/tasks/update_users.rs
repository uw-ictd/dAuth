use std::collections::HashMap;
use std::sync::Arc;

use auth_vector::types::Rand;

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
    // T0: Get set of tasks
    let mut transaction = context.local_context.database_pool.begin().await.unwrap();
    let user_ids = database::tasks::update_users::get_user_ids(&mut transaction).await?;
    transaction.commit().await.unwrap(); // T0 end

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

    // T1: Get all backups for the user
    let mut transaction = context.local_context.database_pool.begin().await.unwrap();
    let user_data: Vec<(String, i64)> =
        database::tasks::update_users::get_user_data(&mut transaction, &user_id)
            .await?
            .into_iter()
            .filter(|v| v.0 != context.local_context.id)
            .collect();
    transaction.commit().await?; // T1 end

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
        // T2: Collect existing vectors (one transaction for each backup network)
        let mut transaction = context.local_context.database_pool.begin().await.unwrap();
        let num_existing_vectors =
            database::vector_state::get_all_by_id(&mut transaction, user_id, backup_network_id)
                .await?
                .len() as i64;
        transaction.commit().await?; // T2 end

        if num_existing_vectors > 0 {
            tracing::info!(
                "Found {} existing vector(s) for {} on {}",
                num_existing_vectors,
                user_id,
                backup_network_id
            );
        }

        tracing::info!(
            ?backup_network_id,
            ?user_id,
            "Building auth vectors for backup network"
        );

        // for each vector (up to max), build a new vector and set of key shares
        for _ in 0..std::cmp::max(
            0,
            context.local_context.max_backup_vectors - num_existing_vectors,
        ) {
            // T3: Build auth vector (up to max backups for each backup network)
            let mut transaction = context.local_context.database_pool.begin().await.unwrap();
            let (vector, seqnum) = core::auth_vectors::build_auth_vector(
                context.clone(),
                &mut transaction,
                user_id,
                *sqn_slice,
            )
            .await?;
            transaction.commit().await?; // T3 end

            let (xres_star_hash, xres_hash, rand) = (
                vector.xres_star_hash.clone(),
                vector.xres_hash.clone(),
                vector.rand.clone(),
            );
            let mut rng = rand_0_8::thread_rng();

            let kseaf_shares = keys::create_shares_from_kseaf(
                &vector.kseaf,
                backup_network_ids.len() as u8,
                std::cmp::min(
                    context.backup_context.backup_key_threshold,
                    backup_network_ids.len() as u8,
                ),
                &mut rng,
            )?;

            let kasme_shares = keys::create_shares_from_kasme(
                &vector.kasme,
                backup_network_ids.len() as u8,
                std::cmp::min(
                    context.backup_context.backup_key_threshold,
                    backup_network_ids.len() as u8,
                ),
                &mut rng,
            )?;

            let mut shares: Vec<(keys::CombinedKeyShare, Rand)> = Vec::new();

            for (kseaf_share, kasme_share) in std::iter::zip(kseaf_shares, kasme_shares) {
                shares.push((
                    keys::CombinedKeyShare {
                        xres_star_hash: xres_star_hash,
                        xres_hash: xres_hash,
                        kasme_share: kasme_share,
                        kseaf_share: kseaf_share,
                    }
                    .to_owned(),
                    rand,
                ));
            }

            vectors_map
                .get_mut(backup_network_id)
                .ok_or(DauthError::DataError("Vectors map error".to_string()))?
                .push(AuthVectorRes {
                    user_id: user_id.to_string(),
                    seqnum,
                    rand: vector.rand,
                    autn: vector.autn,
                    xres_star_hash: vector.xres_star_hash,
                    xres_hash: vector.xres_hash,
                });

            for other_id in &backup_network_ids {
                shares_map
                    .get_mut(other_id)
                    .ok_or(DauthError::DataError("Shares map error".to_string()))?
                    .push(shares.pop().ok_or(DauthError::DataError(
                        "Shares list out of shares".to_string(),
                    ))?);
            }

            if !shares.is_empty() {
                tracing::warn!("{} unused share(s) after share generation", shares.len())
            }
        }
    }

    /* enroll backups */
    for (backup_network_id, seqnum_slice) in &user_data {
        let (address, _) = directory::lookup_network(&context, &backup_network_id).await?;

        let vectors = vectors_map
            .get(backup_network_id)
            .ok_or(DauthError::DataError("Failed to get vectors".to_string()))?;
        let shares = shares_map
            .get(backup_network_id)
            .ok_or(DauthError::DataError("Failed to get shares".to_string()))?;

        tracing::info!(?backup_network_id, ?user_id, "Enrolling backup network");
        backup_network::enroll_backup_prepare(
            context.clone(),
            user_id,
            backup_network_id,
            &address,
        )
        .await?;

        // T4: Enroll backup network, then store vector states
        let mut transaction = context.local_context.database_pool.begin().await.unwrap();
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
        for (combined_share, rand) in shares {
            database::key_share_state::add(
                &mut transaction,
                &combined_share.xres_star_hash,
                &combined_share.xres_hash,
                backup_network_id,
                user_id,
                &rand.as_array(),
            )
            .await?;
        }
        transaction.commit().await?; // T4 end

        // drop rand before sending
        // TODO: allow backups to have rand? Would allow them to check res
        let key_shares: Vec<keys::CombinedKeyShare> = shares
            .into_iter()
            .map(|(combined_share, _rand)| (combined_share.to_owned()))
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

    // T5: Task is complete, so remove
    let mut transaction = context.local_context.database_pool.begin().await.unwrap();
    database::tasks::update_users::remove(&mut transaction, &user_id).await?;
    transaction.commit().await?; // TODO: confirm a transaction this long is okay

    Ok(())
}
