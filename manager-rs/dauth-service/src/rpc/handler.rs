use std::sync::Arc;

use prost::encoding::message;

use crate::rpc::d_auth::aka_vector_resp::ErrorKind;
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
        tracing::info!("Request: {:?}", &message);

        match local::manager::auth_vector_get(self.context.clone(), &message) {
            Some(av_result) => {
                tracing::info!("Returning auth vector: {:?}", av_result);
                Ok(tonic::Response::new(av_result))
            }
            None => {
                tracing::info!("No auth vector found {:?}", message);
                Ok(tonic::Response::new(AkaVectorResp {
                    error: 1, // ErrorKind::NotFound,  Why doesn't this work?
                    auth_vector: None,
                    user_id: message.user_id.clone(),
                    user_id_type: message.user_id_type.clone(),
                }))
            }
        }
    }

    async fn confirm_auth(
        &self,
        request: tonic::Request<AkaConfirmReq>,
    ) -> Result<tonic::Response<AkaConfirmResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);
        unimplemented!()
    }
}
