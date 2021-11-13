use std::sync::Arc;

use crate::rpc::d_auth::local_authentication_server::LocalAuthentication;
use crate::rpc::d_auth::remote_authentication_server::RemoteAuthentication;
use crate::rpc::d_auth::{
    AkaConfirmReq, AkaConfirmResp, AkaVectorReq, AkaVectorResp, AkaVectorUsedResp,
};
use crate::data::context::DauthContext;
use crate::local;
use crate::remote;

/// Handles all RPC calls to the dAuth service.
pub struct DauthHandler {
    pub context: Arc<DauthContext>,
}

#[tonic::async_trait]
impl LocalAuthentication for DauthHandler {
    /// Local (home core) request for a vector
    async fn get_auth_vector(
        &self,
        request: tonic::Request<AkaVectorReq>,
    ) -> Result<tonic::Response<AkaVectorResp>, tonic::Status> {
        let av_request = request.into_inner();
        tracing::info!("Request: {:?}", &av_request);

        match local::manager::auth_vector_get(self.context.clone(), &av_request) {
            Some(av_result) => {
                tracing::info!("Returning auth vector: {:?}", av_result);
                Ok(tonic::Response::new(av_result))
            }
            None => {
                tracing::info!("No auth vector found {:?}", av_request);
                Ok(tonic::Response::new(AkaVectorResp {
                    error: 1, // ErrorKind::NotFound,  Why doesn't this work?
                    auth_vector: None,
                    user_id: av_request.user_id.clone(),
                    user_id_type: av_request.user_id_type.clone(),
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

#[tonic::async_trait]
impl RemoteAuthentication for DauthHandler {
    /// Remote request for a vector
    async fn get_auth_vector_remote(
        &self,
        request: tonic::Request<AkaVectorReq>,
    ) -> Result<tonic::Response<AkaVectorResp>, tonic::Status> {
        let av_request = request.into_inner();
        tracing::info!("Remote request: {:?}", av_request);

        match remote::manager::auth_vector_get_remote(self.context.clone(), &av_request) {
            Some(av_result) => {
                tracing::info!("Returning auth vector: {:?}", av_result);
                Ok(tonic::Response::new(av_result))
            }
            None => {
                tracing::info!("No auth vector found {:?}", av_request);
                Ok(tonic::Response::new(AkaVectorResp {
                    error: 1, // ErrorKind::NotFound,  (nickfh7) Why doesn't this work?
                    auth_vector: None,
                    user_id: av_request.user_id.clone(),
                    user_id_type: av_request.user_id_type.clone(),
                }))
            }
        }
    }

    /// Remote alert that a vector has been used
    async fn report_used_auth_vector(
        &self,
        request: tonic::Request<AkaVectorResp>,
    ) -> Result<tonic::Response<AkaVectorUsedResp>, tonic::Status> {
        let av_result = request.into_inner();
        tracing::info!("Remote used: {:?}", av_result);

        match remote::manager::auth_vector_used_remote(self.context.clone(), &av_result) {
            Ok(()) => tracing::info!("Successfuly reported used: {:?}", av_result),
            Err(e) => tracing::error!("Error reporting used: {}", e),
        };
        Ok(tonic::Response::new(AkaVectorUsedResp {}))
    }
}
