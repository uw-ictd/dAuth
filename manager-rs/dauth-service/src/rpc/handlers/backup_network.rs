use std::sync::Arc;

use auth_vector::types::HresStar;

use crate::data::context::DauthContext;
use crate::data::error::DauthError;
use crate::data::signing::{self, SignPayloadType};
use crate::data::vector::{AuthVectorReq, AuthVectorRes};
use crate::manager;
use crate::rpc::dauth::common::{AuthVector5G, UserIdKind};
use crate::rpc::dauth::remote::backup_network_server::BackupNetwork;
use crate::rpc::dauth::remote::{delegated_auth_vector5_g, delegated_confirmation_share};
use crate::rpc::dauth::remote::{
    DelegatedAuthVector5G, DelegatedConfirmationShare, EnrollBackupCommitReq,
    EnrollBackupCommitResp, EnrollBackupPrepareReq, EnrollBackupPrepareResp, FloodVectorReq,
    FloodVectorResp, GetBackupAuthVectorReq, GetBackupAuthVectorResp, GetKeyShareReq,
    GetKeyShareResp, ReplaceShareReq, ReplaceShareResp, WithdrawBackupReq, WithdrawBackupResp,
    WithdrawSharesReq, WithdrawSharesResp,
};

pub struct BackupNetworkHandler {
    pub context: Arc<DauthContext>,
}

#[tonic::async_trait]
impl BackupNetwork for BackupNetworkHandler {
    /// Request for this network to become a backup network.
    /// Checks for proper authentication and eligibility.
    /// Sets the provided user as being backed up by this network.
    async fn enroll_backup_prepare(
        &self,
        request: tonic::Request<EnrollBackupPrepareReq>,
    ) -> Result<tonic::Response<EnrollBackupPrepareResp>, tonic::Status> {
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

        match BackupNetworkHandler::enroll_backup_prepare_hlp(self.context.clone(), verify_result)
            .await
        {
            Ok(result) => Ok(result),
            Err(e) => Err(tonic::Status::new(
                tonic::Code::Aborted,
                format!("Error while handling request: {}", e),
            )),
        }
    }

    /// Finishes the process of enrolling this network as a backup.
    /// Stores all provided auth vectors and key shares.
    /// Fails if the user is not being backed up by this network.
    async fn enroll_backup_commit(
        &self,
        request: tonic::Request<EnrollBackupCommitReq>,
    ) -> Result<tonic::Response<EnrollBackupCommitResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);

        let content = request.into_inner();
        match BackupNetworkHandler::enroll_backup_commit_hlp(self.context.clone(), content).await {
            Ok(result) => Ok(result),
            Err(e) => Err(tonic::Status::new(
                tonic::Code::Aborted,
                format!("Error while handling request: {}", e),
            )),
        }
    }

    /// Retrieves an auth vector backup that has been stored by this network.
    async fn get_auth_vector(
        &self,
        request: tonic::Request<GetBackupAuthVectorReq>,
    ) -> Result<tonic::Response<GetBackupAuthVectorResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);

        // TODO: Handle retry case? Auth vector is removed from database

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

        match BackupNetworkHandler::get_backup_auth_vector_hlp(self.context.clone(), verify_result)
            .await
        {
            Ok(result) => Ok(result),
            Err(e) => Err(tonic::Status::new(
                tonic::Code::Aborted,
                format!("Error while handling request: {}", e),
            )),
        }
    }

    /// Retrieves a key share that has been stored by this network.
    async fn get_key_share(
        &self,
        request: tonic::Request<GetKeyShareReq>,
    ) -> Result<tonic::Response<GetKeyShareResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);

        // TODO: Need to alert home network
        // TODO: Handle retry case? Key share is removed from database

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

        match BackupNetworkHandler::get_key_share_hlp(self.context.clone(), verify_result).await {
            Ok(result) => Ok(result),
            Err(e) => Err(tonic::Status::new(
                tonic::Code::Aborted,
                format!("Error while handling request: {}", e),
            )),
        }
    }

    async fn replace_key_share(
        &self,
        request: tonic::Request<ReplaceShareReq>,
    ) -> Result<tonic::Response<ReplaceShareResp>, tonic::Status> {
        match BackupNetworkHandler::replace_key_share_hlp(
            self.context.clone(),
            request.into_inner(),
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

    /// Removes the requested user id as a backup on this network.
    /// Deletes all related auth vectors (excludes flood vectors).
    async fn withdraw_backup(
        &self,
        request: tonic::Request<WithdrawBackupReq>,
    ) -> Result<tonic::Response<WithdrawBackupResp>, tonic::Status> {
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

        match BackupNetworkHandler::withdraw_backup_hlp(self.context.clone(), verify_result).await {
            Ok(result) => Ok(result),
            Err(e) => Err(tonic::Status::new(
                tonic::Code::Aborted,
                format!("Error while handling request: {}", e),
            )),
        }
    }

    async fn withdraw_shares(
        &self,
        request: tonic::Request<WithdrawSharesReq>,
    ) -> Result<tonic::Response<WithdrawSharesResp>, tonic::Status> {
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

        match BackupNetworkHandler::withdraw_shares_hlp(self.context.clone(), verify_result).await {
            Ok(result) => Ok(result),
            Err(e) => Err(tonic::Status::new(
                tonic::Code::Aborted,
                format!("Error while handling request: {}", e),
            )),
        }
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

        let verify_result = signing::verify_message(self.context.clone(), &message)
            .await
            .or_else(|e| {
                Err(tonic::Status::new(
                    tonic::Code::Unauthenticated,
                    format!("Failed to verify message: {}", e),
                ))
            })?;

        match BackupNetworkHandler::flood_vector_hlp(self.context.clone(), verify_result).await {
            Ok(result) => Ok(result),
            Err(e) => Err(tonic::Status::new(
                tonic::Code::Aborted,
                format!("Error while handling request: {}", e),
            )),
        }
    }
}

/// Implementation of all helper functions to reuse/condense error logic
impl BackupNetworkHandler {
    /* General helpers */
    async fn handle_delegated_vector(
        context: Arc<DauthContext>,
        dvector: DelegatedAuthVector5G,
        user_id: &str,
    ) -> Result<AuthVectorRes, DauthError> {
        let verify_result = signing::verify_message(
            context,
            &dvector.message.ok_or(DauthError::InvalidMessageError(
                "Missing content".to_string(),
            ))?,
        )
        .await?;

        if let SignPayloadType::DelegatedAuthVector5G(payload) = verify_result {
            Ok(AuthVectorRes::from_av5_g(
                user_id,
                payload.v.ok_or(DauthError::InvalidMessageError(
                    "Missing content".to_string(),
                ))?,
            )?)
        } else {
            Err(DauthError::InvalidMessageError(format!(
                "Incorrect message type: {:?}",
                verify_result
            )))
        }
    }

    async fn handle_key_share(
        context: Arc<DauthContext>,
        dshare: DelegatedConfirmationShare,
    ) -> Result<(auth_vector::types::HresStar, auth_vector::types::Kseaf), DauthError> {
        let verify_result = signing::verify_message(
            context,
            &dshare.message.ok_or(DauthError::InvalidMessageError(
                "Missing content".to_string(),
            ))?,
        )
        .await?;

        if let SignPayloadType::DelegatedConfirmationShare(payload) = verify_result {
            Ok((
                payload.xres_star_hash[..].try_into()?,
                payload.confirmation_share[..].try_into()?,
            ))
        } else {
            Err(DauthError::InvalidMessageError(format!(
                "Incorrect message type: {:?}",
                verify_result
            )))
        }
    }

    /* Specific helpers */

    async fn enroll_backup_prepare_hlp(
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
                        manager::set_backup_user(
                            context.clone(),
                            &user_id,
                            &payload.home_network_id,
                        )
                        .await?;

                        Ok(tonic::Response::new(EnrollBackupPrepareResp {
                            message: Some(signing::sign_message(
                                context,
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

    async fn enroll_backup_commit_hlp(
        context: Arc<DauthContext>,
        content: EnrollBackupCommitReq,
    ) -> Result<tonic::Response<EnrollBackupCommitResp>, DauthError> {
        match content.user_id_kind() {
            UserIdKind::Supi => {
                let user_id = std::str::from_utf8(content.user_id.as_slice())?.to_string();

                // TODO: possibly use home network id?
                let _home_network_id = manager::get_backup_user(context.clone(), &user_id).await?;

                // collect all properly formated delegated vectors
                // log and skip on error
                let mut processed_vectors = Vec::new();
                for dvector in content.vectors {
                    processed_vectors.push(
                        BackupNetworkHandler::handle_delegated_vector(
                            context.clone(),
                            dvector,
                            &user_id,
                        )
                        .await,
                    );
                }
                manager::store_backup_auth_vectors(
                    context.clone(),
                    processed_vectors
                        .into_iter()
                        .flat_map(|vector| {
                            vector.or_else(|e| {
                                tracing::warn!("Failed to process vector: {}", e);
                                Err(e)
                            })
                        })
                        .collect(),
                )
                .await?;

                // collect all properly formated delegated shares
                // log and skip on error
                let mut processed_shares = Vec::new();
                for dshare in content.shares {
                    processed_shares.push(
                        BackupNetworkHandler::handle_key_share(context.clone(), dshare).await,
                    );
                }
                manager::store_key_shares(
                    context.clone(),
                    processed_shares
                        .into_iter()
                        .flat_map(|share| {
                            share.or_else(|e| {
                                tracing::warn!("Failed to process key share: {}", e);
                                Err(e)
                            })
                        })
                        .collect(),
                )
                .await?;

                Ok(tonic::Response::new(EnrollBackupCommitResp {}))
            }
            _ => Err(DauthError::InvalidMessageError(format!(
                "Unsupported user type: {:?}",
                content.user_id_kind()
            ))),
        }
    }

    async fn get_backup_auth_vector_hlp(
        context: Arc<DauthContext>,
        verify_result: SignPayloadType,
    ) -> Result<tonic::Response<GetBackupAuthVectorResp>, DauthError> {
        if let SignPayloadType::GetBackupAuthVectorReq(payload) = verify_result {
            let user_id = std::str::from_utf8(payload.user_id.as_slice())?.to_string();

            let av_result = manager::next_backup_auth_vector(
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

            Ok(tonic::Response::new(GetBackupAuthVectorResp {
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

    async fn get_key_share_hlp(
        context: Arc<DauthContext>,
        verify_result: SignPayloadType,
    ) -> Result<tonic::Response<GetKeyShareResp>, DauthError> {
        if let SignPayloadType::GetKeyShareReq(payload) = verify_result {
            let key_share = manager::get_key_share(
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
                    context,
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

    async fn replace_key_share_hlp(
        context: Arc<DauthContext>,
        request: ReplaceShareReq,
    ) -> Result<tonic::Response<ReplaceShareResp>, DauthError> {
        let dshare = request
            .new_share
            .ok_or(DauthError::DataError("No new share received".to_string()))?;

        let old_xres_star_hash: HresStar = request.replaced_share_xres_star_hash[..].try_into()?;

        let (new_xres_star_hash, new_key_share) =
            BackupNetworkHandler::handle_key_share(context.clone(), dshare).await?;

        manager::replace_key_shares(
            context,
            &old_xres_star_hash,
            &new_xres_star_hash,
            &new_key_share,
        )
        .await?;

        Ok(tonic::Response::new(ReplaceShareResp {}))
    }

    async fn withdraw_backup_hlp(
        context: Arc<DauthContext>,
        verify_result: SignPayloadType,
    ) -> Result<tonic::Response<WithdrawBackupResp>, DauthError> {
        if let SignPayloadType::WithdrawBackupReq(payload) = verify_result {
            let user_id = std::str::from_utf8(payload.user_id.as_slice())?.to_string();

            if context.local_context.id != payload.backup_network_id {
                Err(DauthError::InvalidMessageError(format!(
                    "Wrong intended network: {:?}",
                    payload.backup_network_id
                )))
            } else {
                if manager::get_backup_user(context.clone(), &user_id).await?
                    != payload.home_network_id
                {
                    Err(DauthError::InvalidMessageError(format!(
                        "Not the correct home network",
                    )))
                } else {
                    manager::remove_backup_user(context, &user_id, &payload.home_network_id)
                        .await?;
                    Ok(tonic::Response::new(WithdrawBackupResp {}))
                }
            }
        } else {
            Err(DauthError::InvalidMessageError(format!(
                "Incorrect message type: {:?}",
                verify_result
            )))
        }
    }

    async fn withdraw_shares_hlp(
        context: Arc<DauthContext>,
        verify_result: SignPayloadType,
    ) -> Result<tonic::Response<WithdrawSharesResp>, DauthError> {
        if let SignPayloadType::WithdrawSharesReq(payload) = verify_result {
            manager::remove_key_shares(
                context,
                payload
                    .xres_star_hash
                    .iter()
                    .flat_map(|xres_star_hash_vec| xres_star_hash_vec[..].try_into())
                    .collect(),
            )
            .await?;
            Ok(tonic::Response::new(WithdrawSharesResp {}))
        } else {
            Err(DauthError::InvalidMessageError(format!(
                "Incorrect message type: {:?}",
                verify_result
            )))
        }
    }

    async fn flood_vector_hlp(
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

                    manager::store_backup_flood_vector(
                        context.clone(),
                        &BackupNetworkHandler::handle_delegated_vector(context, dvector, &user_id)
                            .await?,
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
