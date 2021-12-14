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

    match local::database::auth_vector_next(context.clone(), av_request) {
        Ok(av_result) => {
            tracing::info!("Reporting used {:?}", av_result);
            clients::broadcast_auth_vector_used(context, &av_result).await;
            Ok(av_result)
        }
        Err(e) => {
            tracing::info!("No auth vector found in database: {}", e);

            if auth_vector_is_local(context.clone(), av_request) {
                auth_vector_generate(context.clone(), av_request)
            } else {
                // clients::request_auth_vector_remote(context, av_request).await
                todo!()
            }
        }
    }
}

/// Local handler for used vectors.
/// Called for both local and remote use.
pub async fn auth_vector_used(
    context: Arc<DauthContext>,
    av_result: &AuthVectorRes,
) -> Result<(), DauthError> {
    tracing::info!("Handling used: {:?}", av_result);
    local::database::auth_vector_delete(context, av_result)
}

/// Returns whether the auth vector belongs to this core
fn auth_vector_is_local(context: Arc<DauthContext>, av_request: &AuthVectorReq) -> bool {
    utilities::id_less_or_equal(
        &av_request.user_id,
        &context.local_context.local_user_id_max,
    ) && utilities::id_less_or_equal(
        &context.local_context.local_user_id_min,
        &av_request.user_id,
    )
}

/// Generates and returns a new auth vector
/// Will fail if the requested id does not belong to the core
fn auth_vector_generate(
    context: Arc<DauthContext>,
    av_request: &AuthVectorReq,
) -> Result<AuthVectorRes, DauthError> {
    tracing::info!("Attempting to generate for {:?}", av_request.user_id);

    match context
        .local_context
        .user_info_database
        .lock()
        .unwrap()
        .get_mut(&av_request.user_id)
    {
        Some(user_info) => {
            tracing::info!("User found: {:?}", user_info);

            let auth_vector_data =
                auth_vector::generate_vector(&user_info.k, &user_info.opc, &user_info.sqn_max);
            user_info.increment_sqn(0x21);

            let av_response = AuthVectorRes {
                user_id: av_request.user_id,
                rand: auth_vector_data.rand,
                autn: auth_vector_data.autn,
                xres_star_hash: auth_vector_data.xres_star_hash,
            };

            local::database::kseaf_put(
                context.clone(),
                &auth_vector_data.xres_star,
                &auth_vector_data.kseaf,
            );

            tracing::info!("Auth vector generated: {:?}", av_response);

            Ok(av_response)
        }
        None => {
            tracing::error!("No user info exists for {:?}", av_request);
            Err(DauthError::NotFoundError(format!("No user info exists")))
        }
    }
}
