use std::sync::Arc;

use auth_vector::types::ResStar;

use crate::data::context::DauthContext;
use crate::data::error::DauthError;
use crate::data::signing::{self, SignPayloadType};
use crate::data::vector::AuthVectorReq;
use crate::manager;
use crate::rpc::dauth::common::AuthVector5G;
use crate::rpc::dauth::remote::delegated_auth_vector5_g;
use crate::rpc::dauth::remote::home_network_server::HomeNetwork;
use crate::rpc::dauth::remote::{
    DelegatedAuthVector5G, GetHomeAuthVectorReq, GetHomeAuthVectorResp, GetHomeConfirmKeyReq,
    GetHomeConfirmKeyResp,
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
                &AuthVectorReq {
                    user_id: user_id.to_string(),
                },
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

            let kseaf = manager::confirm_auth_vector(context, res_star).await?;

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
}
