use std::sync::Arc;

use auth_vector::types::ResStar;

use crate::data::context::DauthContext;
use crate::data::signing;
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

        match verify_result {
            signing::SignPayloadType::GetHomeAuthVectorReq(payload) => {
                // verify contents and fulfill request
                match payload.user_id_type() {
                    UserIdKind::Supi => {
                        let user_id = std::str::from_utf8(payload.user_id.as_slice())
                            .or_else(|e| {
                                Err(tonic::Status::new(
                                    tonic::Code::InvalidArgument,
                                    format!("Bad user id: {}", e),
                                ))
                            })?
                            .to_string();
                        let av_result = local::manager::auth_vector_get(
                            self.context.clone(),
                            &AuthVectorReq { user_id },
                        )
                        .await
                        .or_else(|e| {
                            Err(tonic::Status::new(
                                tonic::Code::Aborted,
                                format!("Failed to process request: {}", e),
                            ))
                        })?;

                        let payload = delegated_auth_vector5_g::Payload {
                            serving_network_id: self.context.local_context.id.clone(),
                            v: Some(AuthVector5G {
                                rand: av_result.rand.to_vec(),
                                xres_star_hash: av_result.xres_star_hash.to_vec(),
                                autn: av_result.autn.to_vec(),
                                seqnum: av_result.seqnum,
                            }),
                        };

                        let vector = DelegatedAuthVector5G {
                            message: Some(signing::sign_message(
                                self.context.clone(),
                                signing::SignPayloadType::DelegatedAuthVector5G(payload),
                            )),
                        };

                        Ok(tonic::Response::new(GetHomeAuthVectorResp {
                            vector: Some(vector),
                        }))
                    }
                    _ => Err(tonic::Status::new(
                        tonic::Code::InvalidArgument,
                        format!("Unsupported user type: {:?}", payload.user_id_type()),
                    )),
                }
            }
            _ => {
                tracing::error!("Incorrect message type: {:?}", verify_result);
                Err(tonic::Status::new(
                    tonic::Code::InvalidArgument,
                    format!("Incorrect message type"),
                ))
            }
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

        match verify_result {
            signing::SignPayloadType::GetHomeConfirmKeyReq(payload) => {
                let res_star: ResStar = payload.res_star.as_slice().try_into().or_else(|e| {
                    Err(tonic::Status::new(
                        tonic::Code::InvalidArgument,
                        format!("res star is invalid: {}", e),
                    ))
                })?;

                let kseaf =
                    local::manager::confirm_auth_vector_used(self.context.clone(), res_star)
                        .await
                        .or_else(|e| {
                            Err(tonic::Status::new(
                                tonic::Code::NotFound,
                                format!("Failed to get kseaf: {}", e),
                            ))
                        })?;

                Ok(tonic::Response::new(GetHomeConfirmKeyResp {
                    kseaf: kseaf.to_vec(),
                }))
            }
            _ => {
                tracing::error!("Incorrect message type: {:?}", verify_result);
                Err(tonic::Status::new(
                    tonic::Code::InvalidArgument,
                    format!("Incorrect message type"),
                ))
            }
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
        // TODO (nickfh7) add more logic to define cases acceptance

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

        match verify_result {
            signing::SignPayloadType::EnrollBackupPrepareReq(payload) => {
                // verify contents and fulfill request
                match payload.user_id_kind() {
                    UserIdKind::Supi => {
                        let user_id = std::str::from_utf8(payload.user_id.as_slice())
                            .or_else(|e| {
                                Err(tonic::Status::new(
                                    tonic::Code::InvalidArgument,
                                    format!("Bad user id: {}", e),
                                ))
                            })?
                            .to_string();

                        if self.context.local_context.id != payload.backup_network_id {
                            Err(tonic::Status::new(
                                tonic::Code::InvalidArgument,
                                format!(
                                    "Wrong intended backup network: {:?}",
                                    payload.backup_network_id
                                ),
                            ))
                        } else {
                            // TODO (nickfh7) check originating id?
                            self.context
                                .remote_context
                                .pending_backups
                                .lock()
                                .await
                                .insert(user_id, payload.home_network_id.clone());

                            Ok(tonic::Response::new(EnrollBackupPrepareResp {
                                message: Some(signing::sign_message(
                                    self.context.clone(),
                                    signing::SignPayloadType::EnrollBackupPrepareReq(payload),
                                )),
                            }))
                        }
                    }
                    _ => Err(tonic::Status::new(
                        tonic::Code::InvalidArgument,
                        format!("Unsupported user type: {:?}", payload.user_id_kind()),
                    )),
                }
            }
            _ => {
                tracing::error!("Incorrect message type: {:?}", verify_result);
                Err(tonic::Status::new(
                    tonic::Code::InvalidArgument,
                    format!("Incorrect message type"),
                ))
            }
        }
    }

    async fn enroll_backup_commit(
        &self,
        request: tonic::Request<EnrollBackupCommitReq>,
    ) -> Result<tonic::Response<EnrollBackupCommitResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);

        let content = request.into_inner();

        match content.user_id_kind() {
            UserIdKind::Supi => {
                let user_id = std::str::from_utf8(content.user_id.as_slice())
                    .or_else(|e| {
                        Err(tonic::Status::new(
                            tonic::Code::InvalidArgument,
                            format!("Bad user id: {}", e),
                        ))
                    })?
                    .to_string();

                let pending_backups = self.context.remote_context.pending_backups.lock().await;
                let home_network_id = pending_backups.get(&user_id.clone()).ok_or_else(|| {
                    tonic::Status::new(
                        tonic::Code::InvalidArgument,
                        format!("Pending backup request not found"),
                    )
                })?;

                for vector_request in content.vectors {
                    if let Some(message) = vector_request.message {
                        match signing::verify_message(self.context.clone(), &message) {
                            Ok(verify_result) => match verify_result {
                                signing::SignPayloadType::DelegatedAuthVector5G(payload) => {
                                    if let Some(vector) = payload.v {
                                        match AuthVectorRes::from_av5_g(&user_id, vector) {
                                            Ok(av_request) => local::manager::auth_vector_store(
                                                self.context.clone(),
                                                &av_request,
                                            )
                                            .await
                                            .unwrap_or_else(|e| {
                                                tracing::warn!("Failed to store vector: {}", e)
                                            }),
                                            Err(e) => {
                                                tracing::warn!("Failed to process vector: {}", e)
                                            }
                                        }
                                    } else {
                                        tracing::warn!("Empty vector sent")
                                    }
                                }
                                _ => {
                                    tracing::warn!("Bad vector type: {:?}", verify_result)
                                }
                            },
                            Err(e) => {
                                tracing::warn!("Failed to verify auth vector: {}", e)
                            }
                        }
                    } else {
                        tracing::warn!("Recieved empty auth vector");
                    }
                }

                for share_request in content.shares {
                    if let Some(message) = share_request.message {
                        // verify that the home network is the original requester
                        if &message.signer_id == home_network_id {
                            match signing::verify_message(self.context.clone(), &message) {
                                Ok(_payload) => {
                                    todo!() // Todo: Add share requests to database
                                }
                                Err(e) => {
                                    tracing::warn!("Failed to verify auth vector: {}", e)
                                }
                            }
                        } else {
                            tracing::warn!("Signer and home network id mismatch")
                        }
                    } else {
                        tracing::warn!("Recieved empty share request");
                    }
                }

                Ok(tonic::Response::new(EnrollBackupCommitResp {}))
            }
            _ => Err(tonic::Status::new(
                tonic::Code::InvalidArgument,
                format!("Unsupported user type: {:?}", content.user_id_kind()),
            )),
        }
    }

    async fn get_auth_vector(
        &self,
        request: tonic::Request<GetBackupAuthVectorReq>,
    ) -> Result<tonic::Response<GetBackupAuthVectorResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);

        todo!();
    }

    async fn get_key_share(
        &self,
        request: tonic::Request<GetKeyShareReq>,
    ) -> Result<tonic::Response<GetKeyShareResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);

        todo!();
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

        todo!();
    }
}
