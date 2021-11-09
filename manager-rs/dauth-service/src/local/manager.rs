use std::sync::Arc;

use crate::data::context::DauthContext;
use crate::local;
use crate::remote;
use crate::rpc::d_auth::{AkaVectorReq, AkaVectorResp};

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
    if let Some(av_result) =
        local::database::auth_vector_lookup(context.clone(), (&av_request).clone())
    {
        match auth_vector_used(context.clone(), &av_result) {
            Ok(()) => (),
            Err(e) => tracing::error!("Failed to remove used: {}", e),
        }
        match remote::manager::auth_vector_report_used(context.clone(), &av_result) {
            Ok(()) => (),
            Err(e) => tracing::error!("Failed to report used: {}", e),
        }
        Some(av_result)
    } else if let Some(av_result) =
        remote::manager::auth_vector_send_request(context.clone(), &av_request)
    {
        Some(av_result)
    } else {
        None
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
