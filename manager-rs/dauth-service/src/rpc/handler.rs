use std::sync::Arc;

use crate::data::context::DauthContext;
use crate::local;
use crate::remote;
use crate::rpc::dauth::common::local_authentication_server::LocalAuthentication;
use crate::rpc::dauth::common::home_network_server::HomeNetwork;
use crate::rpc::dauth::common::{
    GetHomeAuthVectorReq, GetHomeAuthVectorResp, GetHomeConfirmKeyReq, GetHomeConfirmKeyResp
};
use crate::rpc::dauth::common::{
    AkaConfirmReq, AkaConfirmResp, AkaVectorReq, AkaVectorResp,
};

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
        tracing::info!("Local request: {:?}", &av_request);

        match local::manager::auth_vector_get(self.context.clone(), &av_request).await {
            Ok(av_result) => {
                tracing::info!("Returning result: {:?}", av_result);
                Ok(tonic::Response::new(av_result))
            }
            Err(e) => {
                tracing::error!("Error while handling request for {:?}: {}", av_request, e);
                Err(tonic::Status::new(tonic::Code::Aborted, e.to_string()))
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
impl HomeNetwork for DauthHandler {
    /// Remote request for a vector
    async fn get_auth_vector(
        &self,
        request: tonic::Request<GetHomeAuthVectorReq>,
    ) -> Result<tonic::Response<GetHomeAuthVectorResp>, tonic::Status> {
        let av_request = request.into_inner();
        tracing::info!("Remote request: {:?}", av_request);
        todo!();

        // match remote::manager::auth_vector_get_remote(self.context.clone(), &av_request).await {
        //     Ok(av_result) => {
        //         tracing::info!("Returning result: {:?}", av_result);
        //         Ok(tonic::Response::new(av_result))
        //     }
        //     Err(e) => {
        //         tracing::error!("Error while handling request for {:?}: {}", av_request, e);
        //         Err(tonic::Status::new(tonic::Code::Aborted, e.to_string()))
        //     }
        // }
    }

    /// Remote alert that a vector has been used
    async fn get_confirm_key(
        &self,
        request: tonic::Request<GetHomeConfirmKeyReq>,
    ) -> Result<tonic::Response<GetHomeConfirmKeyResp>, tonic::Status> {
        let av_result = request.into_inner();
        tracing::info!("Remote used: {:?}", av_result);
        let context = self.context.clone();

        todo!();

        // match remote::manager::auth_vector_used_remote(context.clone(), &av_result).await {
        //     Ok(()) => {
        //         tracing::info!("Successfuly reported used: {:?}", av_result);
        //         Ok(tonic::Response::new(AkaVectorUsedResp {}))
        //     }
        //     Err(e) => {
        //         tracing::error!("Error reporting used: {}", e);
        //         Err(tonic::Status::new(tonic::Code::Aborted, e.to_string()))
        //     }
        // }
    }
}
