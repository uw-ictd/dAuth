use std::sync::Arc;

use crate::data::context::DauthContext;
use crate::manager;
use crate::rpc::dauth::local::aka_confirm_resp;
use crate::rpc::dauth::local::local_authentication_server::LocalAuthentication;
use crate::rpc::dauth::local::{AkaConfirmReq, AkaConfirmResp, AkaVectorReq, AkaVectorResp};

pub struct LocalAuthenticationHandler {
    pub context: Arc<DauthContext>,
}

#[tonic::async_trait]
impl LocalAuthentication for LocalAuthenticationHandler {
    /// Local request for a vector that will be used on this network.
    /// No authentication is done.
    async fn get_auth_vector(
        &self,
        request: tonic::Request<AkaVectorReq>,
    ) -> Result<tonic::Response<AkaVectorResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);

        let content = request.into_inner();
        let user_id: String;
        match std::str::from_utf8(content.user_id.as_slice()) {
            Ok(res) => user_id = res.to_string(),
            Err(e) => return Err(tonic::Status::new(tonic::Code::Aborted, e.to_string())),
        }

        match manager::find_vector(
            self.context.clone(),
            &user_id,
            &self.context.local_context.id,
        )
        .await
        {
            Ok(av_result) => {
                tracing::info!("Returning result: {:?}", av_result);
                Ok(tonic::Response::new(av_result.to_resp()))
            }
            Err(e) => {
                tracing::error!("Error while handling request: {}", e);
                Err(tonic::Status::new(tonic::Code::Aborted, e.to_string()))
            }
        }
    }

    /// Local request for to complete auth process for a vector.
    /// No authentication is done.
    async fn confirm_auth(
        &self,
        request: tonic::Request<AkaConfirmReq>,
    ) -> Result<tonic::Response<AkaConfirmResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);

        let res_star = request.into_inner().res_star;
        let res_star: auth_vector::types::ResStar =
            res_star.try_into().or_else(|_e: Vec<u8>| {
                Err(tonic::Status::new(
                    tonic::Code::OutOfRange,
                    "Unable to parse res_star",
                ))
            })?;

        let kseaf = manager::confirm_auth_vector(self.context.clone(), res_star)
            .await
            .or_else(|e| Err(tonic::Status::new(tonic::Code::NotFound, e.to_string())))?;

        let response_payload = AkaConfirmResp {
            error: aka_confirm_resp::ErrorKind::NoError as i32,
            kseaf: kseaf.to_vec(),
        };

        Ok(tonic::Response::new(response_payload))
    }
}
