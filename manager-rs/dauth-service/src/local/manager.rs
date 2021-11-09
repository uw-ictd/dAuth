use std::sync::Arc;

use crate::data::context::DauthContext;
use crate::local;
use crate::remote;
use crate::rpc::d_auth::{AkaVectorReq, AkaVectorResp};

/// Attempts to find or possibly generate a new auth vector
/// Order of checks:
/// 1. Check local database (if found, returns and deletes)
/// 2. Generate if belongs to home network
/// 3. Query remote if nothing can be done locally
pub fn auth_vector_get(
    context: Arc<DauthContext>,
    av_request: &AkaVectorReq,
) -> Option<AkaVectorResp> {
    println!("local::manager::auth_vector_get");
    if let Some(av_result) =
        local::database::auth_vector_lookup(context.clone(), (&av_request).clone())
    {
        match auth_vector_used(context.clone(), &av_result) {
            _ => (),
        }
        match remote::manager::auth_vector_report_used(context.clone(), &av_result) {
            _ => (),
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

pub fn auth_vector_used(
    context: Arc<DauthContext>,
    av_result: &AkaVectorResp,
) -> Result<(), &'static str> {
    println!("local::manager::auth_vector_used");
    local::database::auth_vector_delete(context, av_result)
}
