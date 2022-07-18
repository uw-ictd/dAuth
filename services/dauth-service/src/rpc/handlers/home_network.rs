use std::sync::Arc;

use auth_vector::types::ResStar;

use crate::core;
use crate::data::combined_res::{ResKind, XResHashKind};
use crate::data::context::DauthContext;
use crate::data::error::DauthError;
use crate::data::signing::{self, SignPayloadType};
use crate::rpc::dauth::common::AuthVector5G;
use crate::rpc::dauth::remote::delegated_auth_vector5_g;
use crate::rpc::dauth::remote::home_network_server::HomeNetwork;
use crate::rpc::dauth::remote::{
    DelegatedAuthVector5G, GetHomeAuthVectorReq, GetHomeAuthVectorResp, GetHomeConfirmKeyReq,
    GetHomeConfirmKeyResp, ReportHomeAuthConsumedReq, ReportHomeAuthConsumedResp,
    ReportHomeKeyShareConsumedReq, ReportHomeKeyShareConsumedResp, get_home_confirm_key_req, get_home_confirm_key_resp,
    get_key_share_req
};
use crate::rpc::utilities;

pub struct HomeNetworkHandler {
    pub context: Arc<DauthContext>,
}

#[tonic::async_trait]
impl HomeNetwork for HomeNetworkHandler {
    /// Remote request for a vector that will be generated on this network.
    /// Checks for proper authentication and reputation.
    async fn get_auth_vector(
        &self,
        request: tonic::Request<GetHomeAuthVectorReq>,
    ) -> Result<tonic::Response<GetHomeAuthVectorResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);

        let monitor = tokio_metrics::TaskMonitor::new();

        let res = monitor
            .instrument(async move {
                let message = request.into_inner().message.ok_or_else(|| {
                    tonic::Status::new(tonic::Code::NotFound, "No message received")
                })?;

                let verify_result = signing::verify_message(&self.context, &message)
                    .await
                    .or_else(|e| {
                        Err(tonic::Status::new(
                            tonic::Code::Unauthenticated,
                            format!("Failed to verify message: {}", e),
                        ))
                    })?;

                match HomeNetworkHandler::get_home_auth_vector_hlp(
                    self.context.clone(),
                    verify_result,
                )
                .await
                {
                    Ok(result) => Ok(result),
                    Err(e) => {
                        tracing::warn!("Home network get auth failed: {}", e);

                        Err(tonic::Status::new(
                            tonic::Code::Aborted,
                            format!("Error while handling request: {}", e),
                        ))
                    }
                }
            })
            .await;

        self.context
            .metrics_context
            .record_metrics("home_network::get_auth_vector", monitor)
            .await;
        res
    }

    /// Remote request for to complete auth process for a vector.
    /// Checks for proper authentication.
    async fn get_confirm_key(
        &self,
        request: tonic::Request<GetHomeConfirmKeyReq>,
    ) -> Result<tonic::Response<GetHomeConfirmKeyResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);

        let monitor = tokio_metrics::TaskMonitor::new();

        let res = monitor
            .instrument(async move {
                let message = request.into_inner().message.ok_or_else(|| {
                    tonic::Status::new(tonic::Code::NotFound, "No message received")
                })?;

                let verify_result = signing::verify_message(&self.context, &message)
                    .await
                    .or_else(|e| {
                        Err(tonic::Status::new(
                            tonic::Code::Unauthenticated,
                            format!("Failed to verify message: {}", e),
                        ))
                    })?;

                match HomeNetworkHandler::get_confirm_key_hlp(self.context.clone(), verify_result)
                    .await
                {
                    Ok(result) => Ok(result),
                    Err(e) => Err(tonic::Status::new(
                        tonic::Code::Aborted,
                        format!("Error while handling request: {}", e),
                    )),
                }
            })
            .await;

        self.context
            .metrics_context
            .record_metrics("home_network::get_confirm_key", monitor)
            .await;
        res
    }

    /// Remote request to report an auth vector as used.
    /// Sends a replacement vector in return.
    async fn report_auth_consumed(
        &self,
        request: tonic::Request<ReportHomeAuthConsumedReq>,
    ) -> Result<tonic::Response<ReportHomeAuthConsumedResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);

        let monitor = tokio_metrics::TaskMonitor::new();

        let res = monitor
            .instrument(async move {
                let content = request.into_inner();

                let message = content.backup_auth_vector_req.clone().ok_or_else(|| {
                    tonic::Status::new(tonic::Code::NotFound, "No message received")
                })?;

                let verify_result = signing::verify_message(&self.context, &message)
                    .await
                    .or_else(|e| {
                        Err(tonic::Status::new(
                            tonic::Code::Unauthenticated,
                            format!("Failed to verify message: {}", e),
                        ))
                    })?;

                match HomeNetworkHandler::report_auth_consumed_hlp(
                    self.context.clone(),
                    content,
                    verify_result,
                )
                .await
                {
                    Ok(result) => Ok(result),
                    Err(e) => Err(tonic::Status::new(
                        tonic::Code::Aborted,
                        format!("Error while handling request: {}", e),
                    )),
                }
            })
            .await;

        self.context
            .metrics_context
            .record_metrics("home_network::report_auth_consumed", monitor)
            .await;
        res
    }

    /// Remote request to report a key share as used.
    /// Sends a replacement key share in return.
    async fn report_key_share_consumed(
        &self,
        request: tonic::Request<ReportHomeKeyShareConsumedReq>,
    ) -> Result<tonic::Response<ReportHomeKeyShareConsumedResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);

        let monitor = tokio_metrics::TaskMonitor::new();

        let res = monitor
            .instrument(async move {
                let content = request.into_inner();

                let message = content.get_key_share_req.ok_or_else(|| {
                    tonic::Status::new(tonic::Code::NotFound, "No message received")
                })?;

                let verify_result = signing::verify_message(&self.context, &message)
                    .await
                    .or_else(|e| {
                        Err(tonic::Status::new(
                            tonic::Code::Unauthenticated,
                            format!("Failed to verify message: {}", e),
                        ))
                    })?;

                match HomeNetworkHandler::report_key_share_consumed_hlp(
                    self.context.clone(),
                    &content.backup_network_id,
                    verify_result,
                )
                .await
                {
                    Ok(result) => Ok(result),
                    Err(e) => Err(tonic::Status::new(
                        tonic::Code::Aborted,
                        format!("Error while handling request: {}", e),
                    )),
                }
            })
            .await;

        self.context
            .metrics_context
            .record_metrics("home_network::report_key_share_consumed", monitor)
            .await;
        res
    }
}

impl HomeNetworkHandler {
    async fn get_home_auth_vector_hlp(
        context: Arc<DauthContext>,
        verify_result: SignPayloadType,
    ) -> Result<tonic::Response<GetHomeAuthVectorResp>, DauthError> {
        if let SignPayloadType::GetHomeAuthVectorReq(payload) = verify_result {
            let user_id = std::str::from_utf8(payload.user_id.as_slice())?.to_string();

            // TODO: Handle reputation

            let av_result =
                core::auth_vectors::generate_local_vector(context.clone(), &user_id, 0).await?;

            let payload = delegated_auth_vector5_g::Payload {
                serving_network_id: context.local_context.id.clone(),
                v: Some(AuthVector5G {
                    rand: av_result.rand.to_vec(),
                    xres_star_hash: av_result.xres_star_hash.to_vec(),
                    autn: av_result.autn.to_vec(),
                    seqnum: av_result.seqnum,
                    xres_hash: av_result.xres_hash.to_vec(),
                }),
            };

            Ok(tonic::Response::new(GetHomeAuthVectorResp {
                vector: Some(DelegatedAuthVector5G {
                    message: Some(signing::sign_message(
                        context,
                        signing::SignPayloadType::DelegatedAuthVector5G(payload),
                    )),
                }),
            }))
        } else {
            Err(DauthError::InvalidMessageError(format!(
                "Incorrect message type: {:?}",
                verify_result
            )))
        }
    }

    async fn get_confirm_key_hlp(
        context: Arc<DauthContext>,
        verify_result: SignPayloadType,
    ) -> Result<tonic::Response<GetHomeConfirmKeyResp>, DauthError> {
        if let SignPayloadType::GetHomeConfirmKeyReq(payload) = verify_result {
            use get_home_confirm_key_req::payload::Preimage;
            let key = match payload.preimage.ok_or(DauthError::InvalidMessageError("missing preimage".to_string()))? {
                Preimage::ResStar(res_star) => {
                    let kseaf = core::confirm_keys::get_confirm_key_res_star(context, res_star.as_slice().try_into()?).await?;
                    get_home_confirm_key_resp::Key::Kseaf(kseaf.to_vec())
                },
                Preimage::Res(res) => {
                    let kasme = core::confirm_keys::get_confirm_key_res(context, res.as_slice().try_into()?).await?;
                    get_home_confirm_key_resp::Key::Kasme(kasme.to_vec())
                }
            };

            Ok(tonic::Response::new(GetHomeConfirmKeyResp {
                key: Some(key),
            }))
        } else {
            Err(DauthError::InvalidMessageError(format!(
                "Incorrect message type: {:?}",
                verify_result
            )))
        }
    }

    async fn report_auth_consumed_hlp(
        context: Arc<DauthContext>,
        content: ReportHomeAuthConsumedReq,
        verify_result: SignPayloadType,
    ) -> Result<tonic::Response<ReportHomeAuthConsumedResp>, DauthError> {
        // TODO: Check the payload further
        if let SignPayloadType::GetBackupAuthVectorReq(_payload) = verify_result {
            let core_response = core::auth_vectors::backup_auth_vector_used(
                context.clone(),
                &content.backup_network_id,
                content.xres_star_hash[..].try_into()?,
            ).await?;

            match core_response {
                Some(response) => {
                    Ok(tonic::Response::new(ReportHomeAuthConsumedResp {
                        vector: Some(utilities::build_delegated_vector(
                            context,
                            &response,
                            &content.backup_network_id,
                        )),
                    }))
                },
                None => {
                    Ok(tonic::Response::new(ReportHomeAuthConsumedResp {
                        vector: None,
                    }))
                }
            }


        } else {
            Err(DauthError::InvalidMessageError(format!(
                "Incorrect message type: {:?}",
                verify_result
            )))
        }
    }

    async fn report_key_share_consumed_hlp(
        context: Arc<DauthContext>,
        backup_network_id: &str,
        verify_result: SignPayloadType,
    ) -> Result<tonic::Response<ReportHomeKeyShareConsumedResp>, DauthError> {
        // TODO: Check the payload further
        if let SignPayloadType::GetKeyShareReq(payload) = verify_result {
            let hash = match payload.hash.ok_or(DauthError::InvalidMessageError("Missing hash".to_string()))? {
                get_key_share_req::payload::Hash::XresHash(h) => XResHashKind::XResHash(h.as_slice().try_into()?),
                get_key_share_req::payload::Hash::XresStarHash(h) => XResHashKind::XResStarHash(h.as_slice().try_into()?),
            };

            let preimage = match payload.preimage.ok_or(DauthError::InvalidMessageError("Missing preimage".to_string()))? {
                get_key_share_req::payload::Preimage::Res(r) => ResKind::Res(r.as_slice().try_into()?),
                get_key_share_req::payload::Preimage::ResStar(r) => ResKind::ResStar(r.as_slice().try_into()?),
            };

            core::confirm_keys::key_share_used(
                context,
                &preimage,
                &hash,
                backup_network_id,
            )
            .await?;
            Ok(tonic::Response::new(ReportHomeKeyShareConsumedResp {
                share: None, // TODO: requires extra state and generation cases
            }))
        } else {
            Err(DauthError::InvalidMessageError(format!(
                "Incorrect message type: {:?}",
                verify_result
            )))
        }
    }
}
