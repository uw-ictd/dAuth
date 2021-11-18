use std::sync::Arc;

use auth_vector;

use crate::data::{context::DauthContext, error::DauthError};
use crate::local;
use crate::remote;
use crate::rpc::d_auth::{AkaVectorReq, AkaVectorResp, AuthVector5G};

/// Attempts to find or possibly generate a new auth vector.
/// Order of checks:
/// 1. Check local database (if found, returns and deletes).
/// 2. Generate if belongs to home network.
/// 3. Query remote if nothing can be done locally.
/// If all checks fail, returns None.
pub async fn auth_vector_get(
    context: Arc<DauthContext>,
    av_request: &AkaVectorReq,
) -> Result<AkaVectorResp, DauthError> {
    tracing::info!("Handling request: {:?}", av_request);

    match local::database::auth_vector_next(context.clone(), av_request) {
        Ok(av_result) => {
            remote::manager::auth_vector_report_used(context.clone(), &av_result).await;
            Ok(av_result)
        }
        Err(e) => {
            tracing::info!("No auth vector found in database: {}", e);

            if auth_vector_is_local(context.clone(), av_request) {
                auth_vector_generate(context.clone(), av_request)
            } else {
                remote::manager::auth_vector_send_request(context.clone(), &av_request).await
            }
        }
    }
}

/// Local handler for used vectors.
/// Called for both local and remote use.
pub async fn auth_vector_used(
    context: Arc<DauthContext>,
    av_result: &AkaVectorResp,
) -> Result<(), DauthError> {
    tracing::info!("Handling used: {:?}", av_result);
    local::database::auth_vector_delete(context, av_result)
}

/// Returns whether the auth vector belongs to this core
fn auth_vector_is_local(_context: Arc<DauthContext>, _av_request: &AkaVectorReq) -> bool {
    // TODO(nickfh7) Add logic to determine if local
    true
}

/// Generates and returns a new auth vector
/// Will fail if the requested id does not belong to the core
fn auth_vector_generate(
    context: Arc<DauthContext>,
    av_request: &AkaVectorReq,
) -> Result<AkaVectorResp, DauthError> {
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
                auth_vector::generate_vector(user_info.k, user_info.opc, user_info.sqn_max);
            user_info.increment_sqn(0x21);

            let av_response = AkaVectorResp {
                error: 0,
                auth_vector: Some(AuthVector5G {
                    rand: auth_vector_data.rand,
                    xres_star: auth_vector_data.res,
                    // WRONG FIELDS
                    autn: auth_vector_data.sqn_xor_ak,
                    kseaf: auth_vector_data.mac_a,
                }),
                user_id: av_request.user_id.clone(),
                user_id_type: av_request.user_id_type,
            };

            tracing::info!("Auth vector generated: {:?}", av_response);

            Ok(av_response)
        }
        None => {
            tracing::error!("No user info exists for {:?}", av_request);
            Err(DauthError::NotFoundError(format!("No user info exists")))
        }
    }
}
