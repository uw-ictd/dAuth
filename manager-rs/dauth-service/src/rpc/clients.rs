use std::sync::Arc;

use crate::data::context::DauthContext;
use crate::rpc::d_auth::{AkaVectorReq, AkaVectorResp};

/// Send out request to remote core for new auth vector.
pub fn request_auth_vector_remote(
    context: Arc<DauthContext>,
    av_request: &AkaVectorReq,
) -> Option<AkaVectorResp> {
    tracing::info!("Sending remote request: {:?}", av_request);
    Some(AkaVectorResp {
        error: 0,
        auth_vector: None,
    })
}

/// Broadcast to all other cores that an auth vector was used.
pub fn broadcast_auth_vector_used(
    context: Arc<DauthContext>,
    av_result: &AkaVectorResp,
) -> Result<(), &'static str> {
    tracing::info!("Broadcasting usage: {:?}", av_result);
    Ok(())
}
