use std::sync::Arc;

use crate::core;
use crate::data::combined_res::ResKind;
use crate::data::context::DauthContext;
use crate::data::error::DauthError;
use crate::data::keys::KeyKind;
use crate::rpc::dauth::local::local_authentication_server::LocalAuthentication;
use crate::rpc::dauth::local::{aka_confirm_req, aka_confirm_resp};
use crate::rpc::dauth::local::{AkaConfirmReq, AkaConfirmResp, AkaVectorReq, AkaVectorResp};

pub struct LocalAuthenticationHandler {
    pub context: Arc<DauthContext>,
}

#[tonic::async_trait]
impl LocalAuthentication for LocalAuthenticationHandler {
    /// Local request for a vector that will be used on this network.
    /// No authentication is done.
    #[tracing::instrument(skip_all)]
    async fn get_auth_vector(
        &self,
        request: tonic::Request<AkaVectorReq>,
    ) -> Result<tonic::Response<AkaVectorResp>, tonic::Status> {
        tracing::debug!(?request, "Request received");

        let monitor = tokio_metrics::TaskMonitor::new();

        let res = monitor
            .instrument(async move {
                let content = request.into_inner();
                let user_id: String;
                match std::str::from_utf8(content.user_id.as_slice()) {
                    Ok(res) => user_id = res.to_string(),
                    Err(e) => {
                        tracing::error!("Error while handling request: {}", e);
                        return Err(tonic::Status::new(tonic::Code::Aborted, e.to_string()));
                    }
                }

                match core::auth_vectors::find_vector(
                    self.context.clone(),
                    &user_id,
                    &self.context.local_context.id,
                    content.resync_info.is_some(),
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
            })
            .await;

        self.context
            .metrics_context
            .record_metrics("local_authentication::get_auth_vector", monitor)
            .await;
        res
    }

    /// Local request for to complete auth process for a vector.
    /// No authentication is done.
    #[tracing::instrument(skip_all)]
    async fn confirm_auth(
        &self,
        request: tonic::Request<AkaConfirmReq>,
    ) -> Result<tonic::Response<AkaConfirmResp>, tonic::Status> {
        tracing::debug!(?request, "Request received");

        let monitor = tokio_metrics::TaskMonitor::new();

        let res = monitor
            .instrument(async move {
                match self.confirm_auth_hlp(request.into_inner()).await {
                    Ok(key) => {
                        let response_payload = AkaConfirmResp {
                            error: aka_confirm_resp::ErrorKind::NoError as i32,
                            key: Some(key),
                        };

                        tracing::info!("Returning result: {:?}", response_payload);
                        Ok(tonic::Response::new(response_payload))
                    }
                    Err(e) => {
                        tracing::error!("Error while handling request: {}", e);
                        Err(tonic::Status::new(tonic::Code::NotFound, e.to_string()))
                    }
                }
            })
            .await;

        self.context
            .metrics_context
            .record_metrics("local_authentication::confirm_auth", monitor)
            .await;
        res
    }
}

impl LocalAuthenticationHandler {
    async fn confirm_auth_hlp(
        &self,
        payload: AkaConfirmReq,
    ) -> Result<aka_confirm_resp::Key, DauthError> {
        let user_id = std::str::from_utf8(payload.user_id.as_slice())?.to_string();

        let res = match payload.response.ok_or(DauthError::InvalidMessageError(
            "Missing required UE response".to_string(),
        ))? {
            aka_confirm_req::Response::Res(r) => {
                tracing::debug!(?r, "Received Res");
                ResKind::Res(
                    r.try_into()
                        .or(Err(DauthError::DataError("brokenRes".to_string())))?,
                )
            }
            aka_confirm_req::Response::ResStar(r) => {
                tracing::debug!(?r, "Received Res*");
                ResKind::ResStar(
                    r.try_into()
                        .or(Err(DauthError::DataError("brokenResStar".to_string())))?,
                )
            }
        };

        let key =
            match core::confirm_keys::confirm_authentication(self.context.clone(), &user_id, res)
                .await?
            {
                KeyKind::Kasme(k) => aka_confirm_resp::Key::Kasme(k.to_vec()),
                KeyKind::Kseaf(k) => aka_confirm_resp::Key::Kseaf(k.to_vec()),
            };
        Ok(key)
    }
}
