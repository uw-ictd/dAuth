use std::sync::Arc;

use auth_vector;

use crate::data::{
    context::DauthContext,
    error::DauthError,
    utilities,
    vector::{AuthVectorReq, AuthVectorRes},
};
use crate::database;
use crate::database::utilities::DauthDataUtilities;

/// Generates and returns a new auth vector
/// Will fail if the requested id does not belong to the core
pub async fn generate_auth_vector(
    context: Arc<DauthContext>,
    av_request: &AuthVectorReq,
) -> Result<AuthVectorRes, DauthError> {
    tracing::info!("Generating new vector for {:?}", av_request.user_id);

    let mut transaction = context.local_context.database_pool.begin().await?;

    let mut user_info = database::user_infos::get(&mut transaction, &av_request.user_id)
        .await?
        .to_user_info()?;

    tracing::info!("User found: {:?}", user_info);

    // generate vector, then store new sqn max in the database
    let auth_vector_data =
        auth_vector::generate_vector(&user_info.k, &user_info.opc, &user_info.sqn_max);
    user_info.increment_sqn(32);
    database::user_infos::upsert(
        &mut transaction,
        &av_request.user_id,
        &user_info.k,
        &user_info.opc,
        &user_info.sqn_max,
    )
    .await?;

    let seqnum = utilities::convert_sqn_bytes_to_int(&user_info.sqn_max)?;

    let av_response = AuthVectorRes {
        user_id: av_request.user_id.clone(),
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

/// Local handler for used vectors.
/// Called for both local and remote use.
pub async fn confirm_auth_vector(
    context: Arc<DauthContext>,
    res_star: auth_vector::types::ResStar,
) -> Result<auth_vector::types::Kseaf, DauthError> {
    tracing::info!("Handling confirm: {:?}", res_star);

    let mut transaction = context.local_context.database_pool.begin().await?;

    let kseaf = database::kseafs::get(&mut transaction, &res_star)
        .await?
        .to_kseaf()?;

    transaction.commit().await?;
    Ok(kseaf)
}

// Store a new auth vector, likely as a backup
pub async fn store_auth_vector(
    context: Arc<DauthContext>,
    av_result: &AuthVectorRes,
) -> Result<(), DauthError> {
    tracing::info!("Handling request: {:?}", av_result);

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

// Store a new auth vector, likely as a backup
pub async fn store_flood_vector(
    context: Arc<DauthContext>,
    av_result: &AuthVectorRes,
) -> Result<(), DauthError> {
    tracing::info!("Handling request: {:?}", av_result);

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

/// Removes and returns vector if at least one exists.
pub async fn next_auth_vector(
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
        database::flood_vectors::remove(&mut transaction, &vector.user_id, vector.seqnum).await?;
    } else {
        vector = database::auth_vectors::get_first(&mut transaction, &av_request.user_id)
            .await?
            .to_auth_vector()?;

        database::auth_vectors::remove(&mut transaction, &vector.user_id, vector.seqnum).await?;
    };

    transaction.commit().await?;

    Ok(vector)
}

// Store a new auth vector, likely as a backup
pub async fn store_key_share(
    context: Arc<DauthContext>,
    xres_star_hash: &auth_vector::types::HresStar,
    key_share: &auth_vector::types::Kseaf,
) -> Result<(), DauthError> {
    tracing::info!("Handling key store: {:?} - {:?}", xres_star_hash, key_share);

    let mut transaction = context.local_context.database_pool.begin().await?;

    database::key_shares::add(&mut transaction, xres_star_hash, key_share).await?;

    transaction.commit().await?;
    Ok(())
}

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

    // TODO: Validate res_star, send back to home network

    let mut transaction = context.local_context.database_pool.begin().await?;

    let key_share = database::key_shares::get(&mut transaction, xres_star_hash)
        .await?
        .to_key_share()?;

    transaction.commit().await?;
    Ok(key_share)
}

/// Returns whether the auth vector belongs to this core
fn _auth_vector_is_local(context: Arc<DauthContext>, av_request: &AuthVectorReq) -> bool {
    (av_request.user_id <= context.local_context.local_user_id_max)
        && (av_request.user_id >= context.local_context.local_user_id_min)
}
