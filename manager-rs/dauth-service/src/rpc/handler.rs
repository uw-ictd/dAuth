use std::rc::Rc;
use std::sync::Arc;

use crate::rpc::d_auth::local_authentication_server::LocalAuthentication;
use crate::rpc::d_auth::{AkaVectorReq, AkaVectorResp, AkaConfirmReq, AkaConfirmResp};

use crate::data::auth_vector::{AuthVectorRequest, AuthVectorResult};
use crate::data::context::DauthContext;
use crate::local;
use crate::remote;

/// Handles all RPC calls to the dAuth service.
pub struct DauthHandler {
    pub context: Arc<DauthContext>,
}

impl DauthHandler {
    /// Remote core request for a vector
    pub fn auth_vector_get_remote(&self) {
        println!("--- RPC call for remote vector request");
        println!("rpc::DauthHandler::auth_vector_generate_remote");
        remote::manager::auth_vector_get_remote(
            self.context.clone(),
            Rc::new(AuthVectorRequest {}),
        );
        println!("--- RPC call complete");
    }

    /// Remote alert that a vector has been used
    pub fn auth_vector_used_remote(&self) {
        println!("--- RPC call for remote vector used");
        println!("rpc::DauthHandler::auth_vector_used_remote");
        match remote::manager::auth_vector_used_remote(
            self.context.clone(),
            Rc::new(AuthVectorResult {}),
        ) {
            Ok(()) => (),
            Err(e) => println!("Error reporting used vector: {}", e),
        };
        println!("--- RPC call complete");
    }
}

#[tonic::async_trait]
impl LocalAuthentication for DauthHandler {
    /// Local (home) core request for a vector
    async fn get_auth_vector(&self, request: tonic::Request<AkaVectorReq>) -> Result<tonic::Response<AkaVectorResp>, tonic::Status> {
        local::manager::auth_vector_get(self.context.clone(), Rc::new(AuthVectorRequest {}));
        tracing::info!("Request: {:?}", request);
        unimplemented!()
    }

    async fn confirm_auth(&self, request: tonic::Request<AkaConfirmReq>) -> Result<tonic::Response<AkaConfirmResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);
        unimplemented!()
    }
}
