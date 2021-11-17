use std::sync::Arc;

use auth_vector;

use crate::data::context::DauthContext;
use crate::local;
use crate::remote;
use crate::rpc::d_auth::{AkaVectorReq, AkaVectorResp, AuthVector5G};

/// Attempts to find or possibly generate a new auth vector.
/// Order of checks:
/// 1. Check local database (if found, returns and deletes).
/// 2. Generate if belongs to home network.
/// 3. Query remote if nothing can be done locally.
/// If all checks fail, returns None.
pub fn auth_vector_get(
    context: Arc<DauthContext>,
    av_request: &AkaVectorReq,
) -> Option<AkaVectorResp> {
    tracing::info!("Handling request: {:?}", av_request);

    // Check local database
    if let Some(av_result) = local::database::auth_vector_next(context.clone(), av_request) {
        remote::manager::auth_vector_report_used(context.clone(), &av_result);
        Some(av_result)

    // Generate new
    } else if auth_vector_is_local(context.clone(), av_request) {
        match auth_vector_generate(context.clone(), av_request) {
            Ok(av_result) => Some(av_result),
            Err(e) => {
                tracing::error!("Failed to generate new auth vector: {}", e);
                None
            }
        }

    // Check remote
    } else {
        remote::manager::auth_vector_send_request(context.clone(), &av_request)
    }
}

/// Local handler for used vectors.
/// Called for both local and remote use.
pub fn auth_vector_used(
    context: Arc<DauthContext>,
    av_result: &AkaVectorResp,
) -> Result<(), &'static str> {
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
) -> Result<AkaVectorResp, &'static str> {
    // TODO(nickfh7) integrate with auth-vector crate
    tracing::info!("Attempting to generate for {:?}", av_request.user_id);
    match context.local_context.user_info_database.lock() {
        Ok(mut user_db) => {
            match user_db.get_mut(&av_request.user_id) {
                Some(user_info) => {
                    tracing::info!("User found: {:?}", user_info);
                    let (xres, rand, sqn_xor_ak, mac_a) =
                        auth_vector::generate_vector(user_info.k, user_info.opc, user_info.sqn_max);
                    user_info.increment_sqn(0x21);
                    Ok(AkaVectorResp {
                        error: 0,
                        auth_vector: Some(AuthVector5G {
                            rand: Vec::from(rand),
                            xres_star: Vec::from(xres),
                            // WRONG FIELDS
                            autn: Vec::from(sqn_xor_ak),
                            kausf: Vec::from(mac_a),
                        }),
                        user_id: av_request.user_id.clone(),
                        user_id_type: av_request.user_id_type,
                    })
                }
                None => Err("No user info exists"),
            }
        }
        Err(e) => {
            tracing::error!("Failed to get mutex: {}", e);
            Err("Failed to get mutex")
        }
    }
}
