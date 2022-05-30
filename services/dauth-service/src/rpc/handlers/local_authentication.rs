use std::sync::Arc;

use auth_vector::types::Kseaf;

use crate::core;
use crate::data::context::DauthContext;
use crate::data::error::DauthError;
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

        match core::auth_vectors::find_vector(
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

        match self.confirm_auth_hlp(request.into_inner()).await {
            Ok(kseaf) => {
                let response_payload = AkaConfirmResp {
                    error: aka_confirm_resp::ErrorKind::NoError as i32,
                    kseaf: kseaf.to_vec(),
                };

                Ok(tonic::Response::new(response_payload))
            }
            Err(e) => Err(tonic::Status::new(tonic::Code::NotFound, e.to_string())),
        }
    }
}

impl LocalAuthenticationHandler {
    async fn confirm_auth_hlp(&self, payload: AkaConfirmReq) -> Result<Kseaf, DauthError> {
        let user_id = std::str::from_utf8(payload.user_id.as_slice())?.to_string();

        let res_star: auth_vector::types::ResStar = payload.res_star[..].try_into()?;

        Ok(
            core::confirm_keys::confirm_authentication(self.context.clone(), &user_id, res_star)
                .await?,
        )
    }
}
