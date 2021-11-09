use std::sync::Arc;

use crate::data::context::DauthContext;
use crate::local;
use crate::rpc::clients;
use crate::rpc::d_auth::{AkaVectorReq, AkaVectorResp};

pub fn auth_vector_get_remote(
    context: Arc<DauthContext>,
    av_request: &AkaVectorReq,
) -> Option<AkaVectorResp> {
    tracing::info!("Getting auth vector for remote {:?}", av_request);
    local::manager::auth_vector_get(context, av_request)
}

pub fn auth_vector_used_remote(
    context: Arc<DauthContext>,
    av_result: &AkaVectorResp,
) -> Result<(), &'static str> {
    tracing::info!("Handling remote auth vector used {:?}", av_result);
    local::manager::auth_vector_used(context, av_result)
}

pub fn auth_vector_send_request(
    context: Arc<DauthContext>,
    av_request: &AkaVectorReq,
) -> Option<AkaVectorResp> {
    tracing::info!("Sending request for remote vector {:?}", av_request);
    clients::request_auth_vector_remote(context, av_request)
}

pub fn auth_vector_report_used(
    context: Arc<DauthContext>,
    av_result: &AkaVectorResp,
) -> Result<(), &'static str> {
    tracing::info!("Reporting auth vector used {:?}", av_result);
    clients::broadcast_auth_vector_used(context, av_result)
}
