use std::sync::Arc;

use crate::data::{context::DauthContext, error::DauthError};
use crate::local;
use crate::rpc::clients;
use crate::rpc::dauth::common::{AkaVectorReq, AkaVectorResp};

/// Handles a request from a remote core.
pub async fn auth_vector_get_remote(
    context: Arc<DauthContext>,
    av_request: &AkaVectorReq,
) -> Result<AkaVectorResp, DauthError> {
    tracing::info!("Handling remote request {:?}", av_request);
    local::manager::auth_vector_get(context, av_request).await
}

/// Handles a use from a remote core.
pub async fn auth_vector_used_remote(
    context: Arc<DauthContext>,
    av_result: &AkaVectorResp,
) -> Result<(), DauthError> {
    tracing::info!("Handling remote used {:?}", av_result);
    local::manager::auth_vector_used(context, av_result).await
}

/// Requests a vector from a remote core.
pub async fn auth_vector_send_request(
    context: Arc<DauthContext>,
    av_request: &AkaVectorReq,
) -> Result<AkaVectorResp, DauthError> {
    tracing::info!("Requesting from remote {:?}", av_request);
    clients::request_auth_vector_remote(context, av_request).await
}

/// Reports a local use to all other remote cores.
pub async fn auth_vector_report_used(context: Arc<DauthContext>, av_result: &AkaVectorResp) {
    tracing::info!("Reporting used {:?}", av_result);
    clients::broadcast_auth_vector_used(context, av_result).await
}
