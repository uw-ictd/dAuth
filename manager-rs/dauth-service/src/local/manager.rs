use std::sync::Arc;

use auth_vector;

use crate::data::{
    context::DauthContext,
    error::DauthError,
    utilities,
    vector::{AuthVectorReq, AuthVectorRes},
};
use crate::local;
use crate::rpc::clients;

/// Attempts to find or possibly generate a new auth vector.
/// Order of checks:
/// 1. Check local database (if found, returns and deletes).
/// 2. Generate if belongs to home network.
/// 3. Query remote if nothing can be done locally.
/// If all checks fail, returns None.
pub async fn auth_vector_get(
    context: Arc<DauthContext>,
    av_request: &AuthVectorReq,
) -> Result<AuthVectorRes, DauthError> {
    tracing::info!("Handling request: {:?}", av_request);

    match local::database::auth_vector_next(context.clone(), av_request).await {
        Ok(av_result) => {
            tracing::info!("Reporting used {:?}", av_result);
            clients::broadcast_auth_vector_used(context, &av_result).await;
            Ok(av_result)
        }
        Err(e) => {
            tracing::info!("No auth vector found in database: {}", e);

            if auth_vector_is_local(context.clone(), av_request) {
                auth_vector_generate(context.clone(), av_request).await
            } else {
                // clients::request_auth_vector_remote(context, av_request).await
                todo!()
            }
        }
    }
}

// Store a new auth vector, likely as a backup
pub async fn auth_vector_store(
    context: Arc<DauthContext>,
    av_result: &AuthVectorRes,
) -> Result<(), DauthError> {
    tracing::info!("Handling request: {:?}", av_result);

    local::database::auth_vector_put(context.clone(), av_result).await
}

// Store a new auth vector, likely as a backup
pub async fn flood_vector_store(
    context: Arc<DauthContext>,
    av_result: &AuthVectorRes,
) -> Result<(), DauthError> {
    tracing::info!("Handling request: {:?}", av_result);

    local::database::flood_vector_put(context.clone(), av_result).await
}

// Store a new auth vector, likely as a backup
pub async fn key_share_store(
    context: Arc<DauthContext>,
    xres_star_hash: &auth_vector::types::HresStar,
    key_share: &auth_vector::types::Kseaf,
) -> Result<(), DauthError> {
    tracing::info!("Handling key store: {:?} - {:?}", xres_star_hash, key_share);

    local::database::key_share_put(context.clone(), xres_star_hash, key_share).await
}

pub async fn key_share_get(
    context: Arc<DauthContext>,
    res_star: &auth_vector::types::ResStar,
    xres_star_hash: &auth_vector::types::HresStar,
) -> Result<auth_vector::types::Kseaf, DauthError> {
    tracing::info!(
        "Handling key share get: {:?} - {:?}",
        res_star,
        xres_star_hash,
    );

    // TODO: Validate res_star!

    local::database::key_share_get(context, xres_star_hash).await
}

/// Local handler for used vectors.
/// Called for both local and remote use.
pub async fn confirm_auth_vector_used(
    context: Arc<DauthContext>,
    res_star: auth_vector::types::ResStar,
) -> Result<auth_vector::types::Kseaf, DauthError> {
    tracing::info!("Handling confirm: {:?}", res_star);

    local::database::kseaf_get(context.clone(), &res_star).await
}

/// Returns whether the auth vector belongs to this core
fn auth_vector_is_local(context: Arc<DauthContext>, av_request: &AuthVectorReq) -> bool {
    (av_request.user_id <= context.local_context.local_user_id_max)
        && (av_request.user_id >= context.local_context.local_user_id_min)
}

/// Generates and returns a new auth vector
/// Will fail if the requested id does not belong to the core
async fn auth_vector_generate(
    context: Arc<DauthContext>,
    av_request: &AuthVectorReq,
) -> Result<AuthVectorRes, DauthError> {
    tracing::info!("Generating new vector for {:?}", av_request.user_id);

    let mut user_info =
        local::database::user_info_get(context.clone(), &av_request.user_id).await?;

    tracing::info!("User found: {:?}", user_info);

    // generate vector, then store new sqn max in the database
    let auth_vector_data =
        auth_vector::generate_vector(&user_info.k, &user_info.opc, &user_info.sqn_max);
    user_info.increment_sqn(0x21);
    local::database::user_info_add(context.clone(), &av_request.user_id, &user_info).await?;

    let seqnum = utilities::convert_sqn_bytes_to_int(&user_info.sqn_max)?;

    let av_response = AuthVectorRes {
        user_id: av_request.user_id.clone(),
        seqnum,
        rand: auth_vector_data.rand,
        autn: auth_vector_data.autn,
        xres_star_hash: auth_vector_data.xres_star_hash,
    };

    local::database::kseaf_put(
        context.clone(),
        &auth_vector_data.xres_star,
        &auth_vector_data.kseaf,
    )
    .await?;

    tracing::info!("Auth vector generated: {:?}", av_response);

    Ok(av_response)
}
