use std::sync::Arc;

use auth_vector::types::ResStar;

use crate::data::context::DauthContext;
use crate::data::error::DauthError;
use crate::data::signing::{self, SignPayloadType};
use crate::data::vector::{AuthVectorReq, AuthVectorRes};
use crate::local;
use crate::rpc::dauth::common::{AuthVector5G, UserIdKind};
use crate::rpc::dauth::local::aka_confirm_resp;
use crate::rpc::dauth::local::local_authentication_server::LocalAuthentication;
use crate::rpc::dauth::local::{AkaConfirmReq, AkaConfirmResp, AkaVectorReq, AkaVectorResp};
use crate::rpc::dauth::remote::*;
use crate::rpc::dauth::remote::{
    backup_network_server::BackupNetwork, home_network_server::HomeNetwork,
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
        tracing::info!("Request: {:?}", request);

        let av_request: AuthVectorReq;
        match AuthVectorReq::from_req(request.into_inner()) {
            Ok(req) => av_request = req,
            Err(e) => return Err(tonic::Status::new(tonic::Code::Aborted, e.to_string())),
        }

        match local::manager::auth_vector_get(self.context.clone(), &av_request).await {
            Ok(av_result) => {
                tracing::info!("Returning result: {:?}", av_result);
                Ok(tonic::Response::new(av_result.to_resp()))
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

        let res_star = request.into_inner().res_star;
        let res_star: auth_vector::types::ResStar =
            res_star.try_into().or_else(|_e: Vec<u8>| {
                Err(tonic::Status::new(
                    tonic::Code::OutOfRange,
                    "Unable to parse res_star",
                ))
            })?;

        let kseaf = local::manager::confirm_auth_vector_used(self.context.clone(), res_star)
            .await
            .or_else(|e| Err(tonic::Status::new(tonic::Code::NotFound, e.to_string())))?;

        let response_payload = AkaConfirmResp {
            error: aka_confirm_resp::ErrorKind::NoError as i32,
            kseaf: kseaf.to_vec(),
        };

        Ok(tonic::Response::new(response_payload))
    }
}

#[tonic::async_trait]
impl HomeNetwork for DauthHandler {
    /// Remote request for a vector
    async fn get_auth_vector(
        &self,
        request: tonic::Request<GetHomeAuthVectorReq>,
    ) -> Result<tonic::Response<GetHomeAuthVectorResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);

        let message = request
            .into_inner()
            .message
            .ok_or_else(|| tonic::Status::new(tonic::Code::NotFound, "No message received"))?;

        let verify_result =
            signing::verify_message(self.context.clone(), &message).or_else(|e| {
                Err(tonic::Status::new(
                    tonic::Code::Unauthenticated,
                    format!("Failed to verify message: {}", e),
                ))
            })?;

        match DauthHandler::get_home_auth_vector_hlp(self.context.clone(), verify_result).await {
            Ok(result) => Ok(result),
            Err(e) => Err(tonic::Status::new(
                tonic::Code::Aborted,
                format!("Error while handling request: {}", e),
            )),
        }
    }

    /// Remote alert that a vector has been used
    async fn get_confirm_key(
        &self,
        request: tonic::Request<GetHomeConfirmKeyReq>,
    ) -> Result<tonic::Response<GetHomeConfirmKeyResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);

        let message = request
            .into_inner()
            .message
            .ok_or_else(|| tonic::Status::new(tonic::Code::NotFound, "No message received"))?;

        let verify_result =
            signing::verify_message(self.context.clone(), &message).or_else(|e| {
                Err(tonic::Status::new(
                    tonic::Code::Unauthenticated,
                    format!("Failed to verify message: {}", e),
                ))
            })?;

        match DauthHandler::get_confirm_key_hlp(self.context.clone(), verify_result).await {
            Ok(result) => Ok(result),
            Err(e) => Err(tonic::Status::new(
                tonic::Code::Aborted,
                format!("Error while handling request: {}", e),
            )),
        }
    }
}

#[tonic::async_trait]
impl BackupNetwork for DauthHandler {
    async fn enroll_backup_prepare(
        &self,
        request: tonic::Request<EnrollBackupPrepareReq>,
    ) -> Result<tonic::Response<EnrollBackupPrepareResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);

        // For now, always accept
        // TODO (nickfh7) add more logic to define acceptance cases

        let message = request
            .into_inner()
            .message
            .ok_or_else(|| tonic::Status::new(tonic::Code::NotFound, "No message received"))?;

        let verify_result =
            signing::verify_message(self.context.clone(), &message).or_else(|e| {
                Err(tonic::Status::new(
                    tonic::Code::Unauthenticated,
                    format!("Failed to verify message: {}", e),
                ))
            })?;

        match DauthHandler::enroll_backup_prepare_hlp(self.context.clone(), verify_result).await {
            Ok(result) => Ok(result),
            Err(e) => Err(tonic::Status::new(
                tonic::Code::Aborted,
                format!("Error while handling request: {}", e),
            )),
        }
    }

    async fn enroll_backup_commit(
        &self,
        request: tonic::Request<EnrollBackupCommitReq>,
    ) -> Result<tonic::Response<EnrollBackupCommitResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);

        let content = request.into_inner();
        match DauthHandler::enroll_backup_commit_hlp(self.context.clone(), content).await {
            Ok(result) => Ok(result),
            Err(e) => Err(tonic::Status::new(
                tonic::Code::Aborted,
                format!("Error while handling request: {}", e),
            )),
        }
    }

    async fn get_auth_vector(
        &self,
        request: tonic::Request<GetBackupAuthVectorReq>,
    ) -> Result<tonic::Response<GetBackupAuthVectorResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);

        let message = request
            .into_inner()
            .message
            .ok_or_else(|| tonic::Status::new(tonic::Code::NotFound, "No message received"))?;

        let verify_result =
            signing::verify_message(self.context.clone(), &message).or_else(|e| {
                Err(tonic::Status::new(
                    tonic::Code::Unauthenticated,
                    format!("Failed to verify message: {}", e),
                ))
            })?;

        match DauthHandler::get_backup_auth_vector_hlp(self.context.clone(), verify_result).await {
            Ok(result) => Ok(result),
            Err(e) => Err(tonic::Status::new(
                tonic::Code::Aborted,
                format!("Error while handling request: {}", e),
            )),
        }
    }

    async fn get_key_share(
        &self,
        request: tonic::Request<GetKeyShareReq>,
    ) -> Result<tonic::Response<GetKeyShareResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);

        let message = request
            .into_inner()
            .message
            .ok_or_else(|| tonic::Status::new(tonic::Code::NotFound, "No message received"))?;

        let verify_result =
            signing::verify_message(self.context.clone(), &message).or_else(|e| {
                Err(tonic::Status::new(
                    tonic::Code::Unauthenticated,
                    format!("Failed to verify message: {}", e),
                ))
            })?;

        match DauthHandler::get_key_share_hlp(self.context.clone(), verify_result).await {
            Ok(result) => Ok(result),
            Err(e) => Err(tonic::Status::new(
                tonic::Code::Aborted,
                format!("Error while handling request: {}", e),
            )),
        }
    }

    async fn withdraw_backup(
        &self,
        request: tonic::Request<WithdrawBackupReq>,
    ) -> Result<tonic::Response<WithdrawBackupResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);

        todo!();
    }

    async fn withdraw_shares(
        &self,
        request: tonic::Request<WithdrawSharesReq>,
    ) -> Result<tonic::Response<WithdrawSharesResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);

        todo!();
    }

    async fn flood_vector(
        &self,
        request: tonic::Request<FloodVectorReq>,
    ) -> Result<tonic::Response<FloodVectorResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);

        let message = request
            .into_inner()
            .message
            .ok_or_else(|| tonic::Status::new(tonic::Code::NotFound, "No message received"))?;

        let verify_result =
            signing::verify_message(self.context.clone(), &message).or_else(|e| {
                Err(tonic::Status::new(
                    tonic::Code::Unauthenticated,
                    format!("Failed to verify message: {}", e),
                ))
            })?;

        match DauthHandler::flood_vector_hlp(self.context.clone(), verify_result).await {
            Ok(result) => Ok(result),
            Err(e) => Err(tonic::Status::new(
                tonic::Code::Aborted,
                format!("Error while handling request: {}", e),
            )),
        }
    }
}

/// Implementation of all helper functions to reuse/condense error logic
impl DauthHandler {
    /* General helpers */
    pub async fn handle_delegated_vector_store(
        context: Arc<DauthContext>,
        dvector: DelegatedAuthVector5G,
        user_id: &str,
        is_flood: bool,
    ) -> Result<(), DauthError> {
        let verify_result = signing::verify_message(
            context.clone(),
            &dvector.message.ok_or(DauthError::InvalidMessageError(
                "Missing content".to_string(),
            ))?,
        )?;

        if let SignPayloadType::DelegatedAuthVector5G(payload) = verify_result {
            if is_flood {
                local::manager::flood_vector_store(
                    context.clone(),
                    &AuthVectorRes::from_av5_g(
                        &user_id,
                        payload.v.ok_or(DauthError::InvalidMessageError(
                            "Missing content".to_string(),
                        ))?,
                    )?,
                )
                .await
            } else {
                local::manager::auth_vector_store(
                    context.clone(),
                    &AuthVectorRes::from_av5_g(
                        user_id,
                        payload.v.ok_or(DauthError::InvalidMessageError(
                            "Missing content".to_string(),
                        ))?,
                    )?,
                )
                .await
            }
        } else {
            Err(DauthError::InvalidMessageError(format!(
                "Incorrect message type: {:?}",
                verify_result
            )))
        }
    }

    pub async fn handle_key_share_store(
        context: Arc<DauthContext>,
        dshare: DelegatedConfirmationShare,
    ) -> Result<(), DauthError> {
        let verify_result = signing::verify_message(
            context.clone(),
            &dshare.message.ok_or(DauthError::InvalidMessageError(
                "Missing content".to_string(),
            ))?,
        )?;

        if let SignPayloadType::DelegatedConfirmationShare(payload) = verify_result {
            local::manager::key_share_store(
                context.clone(),
                &payload.xres_star_hash[..].try_into()?,
                &payload.confirmation_share[..].try_into()?,
            )
            .await
        } else {
            Err(DauthError::InvalidMessageError(format!(
                "Incorrect message type: {:?}",
                verify_result
            )))
        }
    }

    pub async fn handle_get_auth_vector(
        context: Arc<DauthContext>,
        user_id: &str,
    ) -> Result<DelegatedAuthVector5G, DauthError> {
        let av_result = local::manager::auth_vector_get(
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

        Ok(DelegatedAuthVector5G {
            message: Some(signing::sign_message(
                context.clone(),
                signing::SignPayloadType::DelegatedAuthVector5G(payload),
            )),
        })
    }

    /* Specific helpers */
    pub async fn get_home_auth_vector_hlp(
        context: Arc<DauthContext>,
        verify_result: SignPayloadType,
    ) -> Result<tonic::Response<GetHomeAuthVectorResp>, DauthError> {
        if let SignPayloadType::GetHomeAuthVectorReq(payload) = verify_result {
            let user_id = std::str::from_utf8(payload.user_id.as_slice())?.to_string();

            Ok(tonic::Response::new(GetHomeAuthVectorResp {
                vector: Some(
                    DauthHandler::handle_get_auth_vector(context.clone(), &user_id).await?,
                ),
            }))
        } else {
            Err(DauthError::InvalidMessageError(format!(
                "Incorrect message type: {:?}",
                verify_result
            )))
        }
    }

    pub async fn get_confirm_key_hlp(
        context: Arc<DauthContext>,
        verify_result: SignPayloadType,
    ) -> Result<tonic::Response<GetHomeConfirmKeyResp>, DauthError> {
        if let SignPayloadType::GetHomeConfirmKeyReq(payload) = verify_result {
            let res_star: ResStar = payload.res_star.as_slice().try_into()?;

            let kseaf = local::manager::confirm_auth_vector_used(context.clone(), res_star).await?;

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

    pub async fn enroll_backup_prepare_hlp(
        context: Arc<DauthContext>,
        verify_result: SignPayloadType,
    ) -> Result<tonic::Response<EnrollBackupPrepareResp>, DauthError> {
        if let signing::SignPayloadType::EnrollBackupPrepareReq(payload) = verify_result {
            match payload.user_id_kind() {
                UserIdKind::Supi => {
                    let user_id = std::str::from_utf8(payload.user_id.as_slice())?.to_string();

                    if context.local_context.id != payload.backup_network_id {
                        Err(DauthError::InvalidMessageError(format!(
                            "Wrong intended network: {:?}",
                            payload.backup_network_id
                        )))
                    } else {
                        // TODO (nickfh7) check originating id?
                        context
                            .remote_context
                            .pending_backups
                            .lock()
                            .await
                            .insert(user_id, payload.home_network_id.clone());

                        Ok(tonic::Response::new(EnrollBackupPrepareResp {
                            message: Some(signing::sign_message(
                                context.clone(),
                                signing::SignPayloadType::EnrollBackupPrepareReq(payload),
                            )),
                        }))
                    }
                }
                _ => Err(DauthError::InvalidMessageError(format!(
                    "Unsupported user type: {:?}",
                    payload.user_id_kind()
                ))),
            }
        } else {
            Err(DauthError::InvalidMessageError(format!(
                "Incorrect message type: {:?}",
                verify_result
            )))
        }
    }

    pub async fn enroll_backup_commit_hlp(
        context: Arc<DauthContext>,
        content: EnrollBackupCommitReq,
    ) -> Result<tonic::Response<EnrollBackupCommitResp>, DauthError> {
        match content.user_id_kind() {
            UserIdKind::Supi => {
                let user_id = std::str::from_utf8(content.user_id.as_slice())?.to_string();

                let pending_backups = context.remote_context.pending_backups.lock().await;

                // TODO: possibly use home network id?
                let _home_network_id = pending_backups.get(&user_id.clone()).ok_or(
                    DauthError::InvalidMessageError("Enroll backup request not found".to_string()),
                )?;

                for dvector in content.vectors {
                    DauthHandler::handle_delegated_vector_store(
                        context.clone(),
                        dvector,
                        &user_id,
                        false,
                    )
                    .await
                    .or_else(|e| {
                        tracing::warn!("Failed to store vector: {}", e);
                        Ok::<(), DauthError>(()) // proceed through errors for now
                    })?
                }

                for dshare in content.shares {
                    DauthHandler::handle_key_share_store(context.clone(), dshare)
                        .await
                        .or_else(|e| {
                            tracing::warn!("Failed to store key share: {}", e);
                            Ok::<(), DauthError>(()) // proceed through errors for now
                        })?
                }

                Ok(tonic::Response::new(EnrollBackupCommitResp {}))
            }
            _ => Err(DauthError::InvalidMessageError(format!(
                "Unsupported user type: {:?}",
                content.user_id_kind()
            ))),
        }
    }

    pub async fn get_backup_auth_vector_hlp(
        context: Arc<DauthContext>,
        verify_result: SignPayloadType,
    ) -> Result<tonic::Response<GetBackupAuthVectorResp>, DauthError> {
        if let SignPayloadType::GetBackupAuthVectorReq(payload) = verify_result {
            let user_id = std::str::from_utf8(payload.user_id.as_slice())?.to_string();

            Ok(tonic::Response::new(GetBackupAuthVectorResp {
                vector: Some(
                    DauthHandler::handle_get_auth_vector(context.clone(), &user_id).await?,
                ),
            }))
        } else {
            Err(DauthError::InvalidMessageError(format!(
                "Incorrect message type: {:?}",
                verify_result
            )))
        }
    }

    pub async fn get_key_share_hlp(
        context: Arc<DauthContext>,
        verify_result: SignPayloadType,
    ) -> Result<tonic::Response<GetKeyShareResp>, DauthError> {
        if let SignPayloadType::GetKeyShareReq(payload) = verify_result {
            let key_share = local::manager::key_share_get(
                context.clone(),
                payload.res_star[..].try_into()?,
                payload.hash_xres_star[..].try_into()?,
            )
            .await?;

            let payload = delegated_confirmation_share::Payload {
                xres_star_hash: payload.hash_xres_star,
                confirmation_share: key_share.to_vec(),
            };

            let dshare = DelegatedConfirmationShare {
                message: Some(signing::sign_message(
                    context.clone(),
                    signing::SignPayloadType::DelegatedConfirmationShare(payload),
                )),
            };

            Ok(tonic::Response::new(GetKeyShareResp {
                share: Some(dshare),
            }))
        } else {
            Err(DauthError::InvalidMessageError(format!(
                "Incorrect message type: {:?}",
                verify_result
            )))
        }
    }

    pub async fn flood_vector_hlp(
        context: Arc<DauthContext>,
        verify_result: SignPayloadType,
    ) -> Result<tonic::Response<FloodVectorResp>, DauthError> {
        if let SignPayloadType::FloodVectorReq(payload) = verify_result {
            match payload.user_id_kind() {
                UserIdKind::Supi => {
                    let user_id = std::str::from_utf8(payload.user_id.as_slice())?.to_string();
                    let dvector = payload.vector.ok_or(DauthError::InvalidMessageError(
                        "Missing content".to_string(),
                    ))?;

                    DauthHandler::handle_delegated_vector_store(
                        context.clone(),
                        dvector,
                        &user_id,
                        true,
                    )
                    .await?;

                    Ok(tonic::Response::new(FloodVectorResp {}))
                }
                _ => Err(DauthError::InvalidMessageError(format!(
                    "Unsupported user type: {:?}",
                    payload.user_id_kind()
                ))),
            }
        } else {
            Err(DauthError::InvalidMessageError(format!(
                "Incorrect message type: {:?}",
                verify_result
            )))
        }
    }
}
