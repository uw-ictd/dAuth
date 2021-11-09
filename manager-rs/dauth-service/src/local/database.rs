use std::sync::Arc;

use crate::data::context::DauthContext;
use crate::rpc::d_auth::{AkaVectorReq, AkaVectorResp};

pub fn auth_vector_lookup(
    context: Arc<DauthContext>,
    av_request: &AkaVectorReq,
) -> Option<AkaVectorResp> {
    tracing::info!("Database lookup: {:?}", av_request);
    Some(AkaVectorResp {
        error: 0,
        auth_vector: None,
    })
}

pub fn auth_vector_delete(
    context: Arc<DauthContext>,
    av_result: &AkaVectorResp,
) -> Result<(), &'static str> {
    tracing::info!("Database delete: {:?}", av_result);
    Ok(())
}
