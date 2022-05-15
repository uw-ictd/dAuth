use std::sync::Arc;

use auth_vector::types::ResStar;

use crate::data::context::DauthContext;
use crate::data::error::DauthError;
use crate::data::signing::{self, SignPayloadType};
use crate::manager;
use crate::rpc::dauth::common::AuthVector5G;
use crate::rpc::dauth::remote::delegated_auth_vector5_g;
use crate::rpc::dauth::remote::home_network_server::HomeNetwork;
use crate::rpc::dauth::remote::{
    DelegatedAuthVector5G, GetHomeAuthVectorReq, GetHomeAuthVectorResp, GetHomeConfirmKeyReq,
    GetHomeConfirmKeyResp, ReportHomeAuthConsumedReq, ReportHomeAuthConsumedResp,
    ReportHomeKeyShareConsumedReq, ReportHomeKeyShareConsumedResp,
};

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

        let message = request
            .into_inner()
            .message
            .ok_or_else(|| tonic::Status::new(tonic::Code::NotFound, "No message received"))?;

        let verify_result = signing::verify_message(self.context.clone(), &message)
            .await
            .or_else(|e| {
                Err(tonic::Status::new(
                    tonic::Code::Unauthenticated,
                    format!("Failed to verify message: {}", e),
                ))
            })?;

        match HomeNetworkHandler::get_home_auth_vector_hlp(self.context.clone(), verify_result)
            .await
        {
            Ok(result) => Ok(result),
            Err(e) => Err(tonic::Status::new(
                tonic::Code::Aborted,
                format!("Error while handling request: {}", e),
            )),
        }
    }

    /// Remote request for to complete auth process for a vector.
    /// Checks for proper authentication.
    async fn get_confirm_key(
        &self,
        request: tonic::Request<GetHomeConfirmKeyReq>,
    ) -> Result<tonic::Response<GetHomeConfirmKeyResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);

        let message = request
            .into_inner()
            .message
            .ok_or_else(|| tonic::Status::new(tonic::Code::NotFound, "No message received"))?;

        let verify_result = signing::verify_message(self.context.clone(), &message)
            .await
            .or_else(|e| {
                Err(tonic::Status::new(
                    tonic::Code::Unauthenticated,
                    format!("Failed to verify message: {}", e),
                ))
            })?;

        match HomeNetworkHandler::get_confirm_key_hlp(self.context.clone(), verify_result).await {
            Ok(result) => Ok(result),
            Err(e) => Err(tonic::Status::new(
                tonic::Code::Aborted,
                format!("Error while handling request: {}", e),
            )),
        }
    }

    /// Remote request to report an auth vector as used.
    /// Sends a replacement vector in return.
    async fn report_auth_consumed(
        &self,
        request: tonic::Request<ReportHomeAuthConsumedReq>,
    ) -> Result<tonic::Response<ReportHomeAuthConsumedResp>, tonic::Status> {
        todo!()
    }

    /// Remote request to report a key share as used.
    /// Sends a replacement key share in return.
    async fn report_key_share_consumed(
        &self,
        request: tonic::Request<ReportHomeKeyShareConsumedReq>,
    ) -> Result<tonic::Response<ReportHomeKeyShareConsumedResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);

        let content = request.into_inner();

        let message = content
            .get_key_share_req
            .ok_or_else(|| tonic::Status::new(tonic::Code::NotFound, "No message received"))?;

        let verify_result = signing::verify_message(self.context.clone(), &message)
            .await
            .or_else(|e| {
                Err(tonic::Status::new(
                    tonic::Code::Unauthenticated,
                    format!("Failed to verify message: {}", e),
                ))
            })?;

        // TODO: Verify the message further
        // The above will verify that it has been signed appropriately, but nothing else
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

            let av_result = manager::generate_auth_vector(
                context.clone(),
                &user_id,
                manager::get_sqn_slice(context.clone(), &user_id, &payload.serving_network_id)
                    .await?,
            )
            .await?;

            let payload = delegated_auth_vector5_g::Payload {
                serving_network_id: context.local_context.id.clone(),
                v: Some(AuthVector5G {
                    rand: av_result.rand.to_vec(),
                    xres_star_hash: av_result.xres_star_hash.to_vec(),
                    autn: av_result.autn.to_vec(),
                    seqnum: av_result.seqnum,
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
            let res_star: ResStar = payload.res_star.as_slice().try_into()?;

            let kseaf = manager::get_confirm_key(context, res_star).await?;

            Ok(tonic::Response::new(GetHomeConfirmKeyResp {
                kseaf: kseaf.to_vec(),
            }))
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
        if let SignPayloadType::GetKeyShareReq(payload) = verify_result {
            manager::key_share_used(
                context,
                &payload.res_star[..].try_into()?,
                &payload.hash_xres_star[..].try_into()?,
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
