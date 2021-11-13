use std::sync::Arc;

use crate::data::context::DauthContext;
use crate::local;
use crate::rpc::clients;
use crate::rpc::d_auth::{AkaVectorReq, AkaVectorResp};

/// Handles a request from a remote core.
pub fn auth_vector_get_remote(
    context: Arc<DauthContext>,
    av_request: &AkaVectorReq,
) -> Option<AkaVectorResp> {
    tracing::info!("Handling remote request {:?}", av_request);
    local::manager::auth_vector_get(context, av_request)
}

/// Handles a use from a remote core.
pub fn auth_vector_used_remote(
    context: Arc<DauthContext>,
    av_result: &AkaVectorResp,
) -> Result<(), &'static str> {
    tracing::info!("Handling remote used {:?}", av_result);
    local::manager::auth_vector_used(context, av_result)
}

/// Requests a vector from a remote core.
pub fn auth_vector_send_request(
    context: Arc<DauthContext>,
    av_request: &AkaVectorReq,
) -> Option<AkaVectorResp> {
    tracing::info!("Requesting from remote {:?}", av_request);
    clients::request_auth_vector_remote(context, av_request)
}

/// Reports a local use to all other remote cores.
pub fn auth_vector_report_used(context: Arc<DauthContext>, av_result: &AkaVectorResp) {
    tracing::info!("Reporting used {:?}", av_result);
    clients::broadcast_auth_vector_used(context, av_result)
}
