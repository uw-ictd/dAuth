use std::sync::Arc;

use crate::data::context::DauthContext;
use crate::rpc::d_auth::{AkaVectorReq, AkaVectorResp};

pub fn request_auth_vector_remote(
    context: Arc<DauthContext>,
    av_request: &AkaVectorReq,
) -> Option<AkaVectorResp> {
    println!("rpc::clients::request_auth_vector_remote");
    Some(AkaVectorResp {
        error: 0,
        auth_vector: None,
    })
}

pub fn broadcast_auth_vector_used(
    context: Arc<DauthContext>,
    av_result: &AkaVectorResp,
) -> Result<(), &'static str> {
    println!("rpc::clients::broadcast_auth_vector_used");
    Ok(())
}
