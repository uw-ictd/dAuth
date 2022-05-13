use std::sync::Arc;

use auth_vector::{self, constants::SQN_LENGTH};

use crate::data::{
    context::DauthContext,
    error::DauthError,
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
pub async fn find_vector(
    context: Arc<DauthContext>,
    user_id: &str,
    network_id: &str,
) -> Result<AuthVectorRes, DauthError> {
    tracing::info!("Attempting to find a vector: {}-{}", user_id, network_id);

    if let Ok(vector) = generate_auth_vector(
        context.clone(),
        user_id,
        get_sqn_slice(context.clone(), user_id, network_id).await?,
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

/// Generates and returns a new auth vector.
pub async fn generate_auth_vector(
    context: Arc<DauthContext>,
    user_id: &str,
    sqn_slice: i64,
) -> Result<AuthVectorRes, DauthError> {
    tracing::info!("Generating new vector: {}:{}", user_id, sqn_slice);

    let mut transaction = context.local_context.database_pool.begin().await?;

    let mut user_info =
        database::user_infos::get(&mut transaction, &user_id.to_string(), sqn_slice)
            .await?
            .to_user_info()?;

    tracing::info!("User found: {:?}", user_info);

    // generate vector, then store new sqn max in the database
    let auth_vector_data = auth_vector::generate_vector(
        &user_info.k,
        &user_info.opc,
        &user_info.sqn.to_be_bytes()[..SQN_LENGTH].try_into()?,
    );

    user_info.sqn += context.local_context.num_sqn_slices;

    database::user_infos::upsert(
        &mut transaction,
        &user_id.to_string(),
        &user_info.k,
        &user_info.opc,
        user_info.sqn,
        sqn_slice,
    )
    .await?;

    let av_response = AuthVectorRes {
        user_id: user_id.to_string(),
        seqnum: user_info.sqn,
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

/// Attempts to find the Kseaf value for the network.
/// Runs the following checks:
/// 1. If generated on this network, get Kseaf from the database.
/// 2. Else, check the home network for a Kseaf value.
/// 3. If not from the home network, get key shares from the backup networks.
pub async fn confirm_auth_vector(
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
                        Ok(key_share) => {
                            key_shares.push(key_share);
                        }
                        Err(e) => {
                            tracing::warn!("Failed to get key share: {}", e)
                        }
                    }
                }

                // need to derive kseaf from key shares, then alert home network
                todo!()
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

// Store a new auth vector as a backup.
pub async fn _store_backup_auth_vector(
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

/// Removes and returns the next backup auth vector.
/// Checks flood vectors first, then auth vector.
/// Returns auth vector with lowest sequence number.
pub async fn next_backup_auth_vector(
    context: Arc<DauthContext>,
    av_request: &AuthVectorReq,
) -> Result<AuthVectorRes, DauthError> {
    tracing::info!("Vector next: {:?}", av_request);

    let mut transaction = context.local_context.database_pool.begin().await?;

    // Check for a flood vector first
    let vector;
    if let Ok(Some(flood_row)) =
        database::flood_vectors::get_first(&mut transaction, &av_request.user_id).await
    {
        vector = flood_row.to_auth_vector()?;
    } else {
        vector = database::auth_vectors::get_first(&mut transaction, &av_request.user_id)
            .await?
            .to_auth_vector()?;
    };

    transaction.commit().await?;

    Ok(vector)
}

/// Sets the provided user id as a being backed up by this network.
pub async fn set_backup_user(
    context: Arc<DauthContext>,
    user_id: &str,
    home_network_id: &str,
) -> Result<(), DauthError> {
    tracing::info!("Setting backup user: {:?} - {:?}", user_id, home_network_id);

    let mut transaction = context.local_context.database_pool.begin().await?;
    database::backup_users::add(&mut transaction, user_id, home_network_id).await?;
    transaction.commit().await?;
    Ok(())
}

/// Gets the home network of the provided user id.
/// Fails if the user is not being backed up by this network.
pub async fn get_backup_user(
    context: Arc<DauthContext>,
    user_id: &str,
) -> Result<String, DauthError> {
    tracing::info!("Getting backup user: {:?}", user_id);

    let mut transaction = context.local_context.database_pool.begin().await?;
    let res = database::backup_users::get(&mut transaction, user_id)
        .await?
        .to_backup_user_home_network_id()?;
    transaction.commit().await?;
    Ok(res)
}

/// Removes the user from being backup up on this network.
/// Also removes all related auth vectors.
pub async fn remove_backup_user(
    context: Arc<DauthContext>,
    user_id: &str,
    home_network_id: &str,
) -> Result<(), DauthError> {
    tracing::info!("Getting backup user: {:?}", user_id);

    let mut transaction = context.local_context.database_pool.begin().await?;
    database::backup_users::remove(&mut transaction, user_id, home_network_id).await?;
    database::auth_vectors::remove_all(&mut transaction, user_id).await?;
    transaction.commit().await?;
    Ok(())
}

// Stores a collection of key shares.
pub async fn store_key_shares(
    context: Arc<DauthContext>,
    key_shares: Vec<(auth_vector::types::HresStar, auth_vector::types::Kseaf)>,
) -> Result<(), DauthError> {
    tracing::info!("Handling multiple key store: {:?}", key_shares);

    let mut transaction = context.local_context.database_pool.begin().await?;

    for (xres_star_hash, key_share) in key_shares {
        database::key_shares::add(&mut transaction, &xres_star_hash, &key_share).await?;
    }
    transaction.commit().await?;
    Ok(())
}

/// Removes and returns a key share value.
pub async fn get_key_share(
    context: Arc<DauthContext>,
    res_star: &auth_vector::types::ResStar,
    xres_star_hash: &auth_vector::types::HresStar,
) -> Result<auth_vector::types::Kseaf, DauthError> {
    tracing::info!(
        "Handling key share get: {:?} - {:?}",
        res_star,
        xres_star_hash,
    );

    // TODO: Alert home network

    let mut transaction = context.local_context.database_pool.begin().await?;
    let key_share = database::key_shares::get(&mut transaction, xres_star_hash)
        .await?
        .to_key_share()?;

    if let Ok(row) = database::flood_vectors::get_by_hash(&mut transaction, xres_star_hash).await {
        let vector = row.to_auth_vector()?;
        validate_xres_star_hash(xres_star_hash, res_star, &vector.rand)?;
        database::flood_vectors::remove(&mut transaction, &vector.user_id, vector.seqnum).await?;
    } else if let Ok(row) =
        database::auth_vectors::get_by_hash(&mut transaction, xres_star_hash).await
    {
        let vector = row.to_auth_vector()?;
        validate_xres_star_hash(xres_star_hash, res_star, &vector.rand)?;
        database::auth_vectors::remove(&mut transaction, &vector.user_id, vector.seqnum).await?;
    } else {
        tracing::info!("Vector not found on this network: {:?}", xres_star_hash);
    }

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

pub async fn get_sqn_slice(
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
