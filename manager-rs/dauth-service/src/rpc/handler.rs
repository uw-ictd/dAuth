use std::sync::Arc;

use crate::rpc::d_auth::local_authentication_server::LocalAuthentication;
use crate::rpc::d_auth::{AkaConfirmReq, AkaConfirmResp, AkaVectorReq, AkaVectorResp};

use crate::data::context::DauthContext;
use crate::local;
use crate::remote;

/// Handles all RPC calls to the dAuth service.
pub struct DauthHandler {
    pub context: Arc<DauthContext>,
}

impl DauthHandler {
    /// Remote request for a vector
    pub fn auth_vector_get_remote(&self) {
        tracing::info!("Request: {:?}", "remote vector get");
        remote::manager::auth_vector_get_remote(
            self.context.clone(),
            &(AkaVectorReq {
                user_id: vec![0, 1, 2, 3],
                user_id_type: 0,
                resync_info: None,
            }),
        );
    }

    /// Remote alert that a vector has been used
    pub fn auth_vector_used_remote(&self) {
        tracing::info!("Request: {:?}", "remote vector used");
        match remote::manager::auth_vector_used_remote(
            self.context.clone(),
            &(AkaVectorResp {
                error: 0,
                auth_vector: None,
                user_id: vec![0, 1, 2, 3],
                user_id_type: 0,
            }),
        ) {
            Ok(()) => (),
            Err(e) => tracing::error!("Error reporting used: {}", e),
        };
    }
}

#[tonic::async_trait]
impl LocalAuthentication for DauthHandler {
    /// Local (home core) request for a vector
    async fn get_auth_vector(
        &self,
        request: tonic::Request<AkaVectorReq>,
    ) -> Result<tonic::Response<AkaVectorResp>, tonic::Status> {
        let message = request.into_inner();
        local::manager::auth_vector_get(self.context.clone(), &message);
        tracing::info!("Request: {:?}", &message);
        unimplemented!()
    }

    async fn confirm_auth(
        &self,
        request: tonic::Request<AkaConfirmReq>,
    ) -> Result<tonic::Response<AkaConfirmResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);
        unimplemented!()
    }
}
