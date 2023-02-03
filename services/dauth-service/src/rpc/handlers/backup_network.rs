use std::sync::Arc;

use prost::Message;

use auth_vector::types::XResStarHash;

use crate::data::context::DauthContext;
use crate::data::error::DauthError;
use crate::data::signing::{self, SignPayloadType};
use crate::data::vector::AuthVectorReq;
use crate::rpc::dauth::common::{AuthVector5G, UserIdKind};
use crate::rpc::dauth::remote::backup_network_server::BackupNetwork;
use crate::rpc::dauth::remote::{
    delegated_auth_vector5_g, delegated_confirmation_share, SignedMessage,
};
use crate::rpc::dauth::remote::{
    get_key_share_req, DelegatedAuthVector5G, DelegatedConfirmationShare, EnrollBackupCommitReq,
    EnrollBackupCommitResp, EnrollBackupPrepareReq, EnrollBackupPrepareResp, FloodVectorReq,
    FloodVectorResp, GetBackupAuthVectorReq, GetBackupAuthVectorResp, GetKeyShareReq,
    GetKeyShareResp, ReplaceShareReq, ReplaceShareResp, WithdrawBackupReq, WithdrawBackupResp,
    WithdrawSharesReq, WithdrawSharesResp,
};
use crate::rpc::utilities;
use crate::services::backup;

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

                match BackupNetworkHandler::enroll_backup_prepare_hlp(
                    self.context.clone(),
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
            .record_metrics("backup_network::enroll_backup_prepare", monitor)
            .await;
        res
    }

    /// Finishes the process of enrolling this network as a backup.
    /// Stores all provided auth vectors and key shares.
    /// Fails if the user is not being backed up by this network.
    async fn enroll_backup_commit(
        &self,
        request: tonic::Request<EnrollBackupCommitReq>,
    ) -> Result<tonic::Response<EnrollBackupCommitResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);

        let monitor = tokio_metrics::TaskMonitor::new();

        let res = monitor
            .instrument(async move {
                let content = request.into_inner();
                match BackupNetworkHandler::enroll_backup_commit_hlp(self.context.clone(), content)
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
            .record_metrics("backup_network::enroll_backup_commit", monitor)
            .await;
        res
    }

    /// Retrieves an auth vector backup that has been stored by this network.
    async fn get_auth_vector(
        &self,
        request: tonic::Request<GetBackupAuthVectorReq>,
    ) -> Result<tonic::Response<GetBackupAuthVectorResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);

        let monitor = tokio_metrics::TaskMonitor::new();

        let res = monitor
            .instrument(async move {
                let message = request.into_inner().message.ok_or_else(|| {
                    tonic::Status::new(tonic::Code::NotFound, "No message received")
                })?;

                let mut signed_request_bytes = Vec::new();
                message.encode(&mut signed_request_bytes).or_else(|e| {
                    Err(tonic::Status::new(
                        tonic::Code::Aborted,
                        format!("Failed to encode message: {}", e),
                    ))
                })?;

                let verify_result = signing::verify_message(&self.context, &message)
                    .await
                    .or_else(|e| {
                        Err(tonic::Status::new(
                            tonic::Code::Unauthenticated,
                            format!("Failed to verify message: {}", e),
                        ))
                    })?;

                match BackupNetworkHandler::get_backup_auth_vector_hlp(
                    self.context.clone(),
                    verify_result,
                    &signed_request_bytes,
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
            .record_metrics("backup_network::get_auth_vector", monitor)
            .await;
        res
    }

    /// Retrieves a key share that has been stored by this network.
    async fn get_key_share(
        &self,
        request: tonic::Request<GetKeyShareReq>,
    ) -> Result<tonic::Response<GetKeyShareResp>, tonic::Status> {
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

                match BackupNetworkHandler::get_key_share_hlp(
                    self.context.clone(),
                    verify_result,
                    message,
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
            .record_metrics("backup_network::get_key_share", monitor)
            .await;
        res
    }

    async fn replace_key_share(
        &self,
        request: tonic::Request<ReplaceShareReq>,
    ) -> Result<tonic::Response<ReplaceShareResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);

        let monitor = tokio_metrics::TaskMonitor::new();

        let res = monitor
            .instrument(async move {
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
            })
            .await;

        self.context
            .metrics_context
            .record_metrics("backup_network::replace_key_share", monitor)
            .await;
        res
    }

    /// Removes the requested user id as a backup on this network.
    /// Deletes all related auth vectors (excludes flood vectors).
    async fn withdraw_backup(
        &self,
        request: tonic::Request<WithdrawBackupReq>,
    ) -> Result<tonic::Response<WithdrawBackupResp>, tonic::Status> {
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

                match BackupNetworkHandler::withdraw_backup_hlp(self.context.clone(), verify_result)
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
            .record_metrics("backup_network::withdraw_backup", monitor)
            .await;
        res
    }

    async fn withdraw_shares(
        &self,
        request: tonic::Request<WithdrawSharesReq>,
    ) -> Result<tonic::Response<WithdrawSharesResp>, tonic::Status> {
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

                match BackupNetworkHandler::withdraw_shares_hlp(self.context.clone(), verify_result)
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
            .record_metrics("backup_network::withdraw_shares", monitor)
            .await;
        res
    }

    async fn flood_vector(
        &self,
        request: tonic::Request<FloodVectorReq>,
    ) -> Result<tonic::Response<FloodVectorResp>, tonic::Status> {
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

                match BackupNetworkHandler::flood_vector_hlp(self.context.clone(), verify_result)
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
            .record_metrics("backup_network::flood_vector", monitor)
            .await;
        res
    }
}

/// Implementation of all helper functions to reuse/condense error logic
impl BackupNetworkHandler {
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
                        backup::enroll_backup_prepare(
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

                // collect all properly formated delegated vectors
                // log and skip on error
                let mut processed_vectors = Vec::new();
                for dvector in content.vectors {
                    processed_vectors.push(
                        utilities::handle_delegated_vector(&context, dvector, &user_id).await,
                    );
                }

                // collect all properly formated delegated shares
                // log and skip on error
                let mut processed_shares = Vec::new();
                for dshare in content.shares {
                    processed_shares
                        .push(utilities::handle_key_share(context.clone(), dshare).await);
                }

                backup::enroll_backup_commit(
                    context.clone(),
                    &user_id,
                    processed_vectors
                        .into_iter()
                        .flat_map(|vector| {
                            vector.or_else(|e| {
                                tracing::warn!("Failed to process vector: {}", e);
                                Err(e)
                            })
                        })
                        .collect(),
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
        signed_request_bytes: &Vec<u8>,
    ) -> Result<tonic::Response<GetBackupAuthVectorResp>, DauthError> {
        if let SignPayloadType::GetBackupAuthVectorReq(payload) = verify_result {
            let user_id = std::str::from_utf8(payload.user_id.as_slice())?.to_string();

            // If we get any information about stale sequence numbers in our
            // auth list, remove them before proceeding.
            if let Some(resync_xres_star_hash) = payload.xres_star_hash_resync {
                let mut transaction = context.local_context.database_pool.begin().await?;
                // Don't remove flood vectors though until receiving a
                // confirmation, since we need to be sure they are used.
                // crate::database::flood_vectors::remove(&mut transaction, &user_id, &resync_xres_star_hash).await?;
                crate::database::auth_vectors::remove(
                    &mut transaction,
                    &user_id,
                    &resync_xres_star_hash,
                )
                .await?;
                transaction.commit().await?;
            }

            let av_result = backup::get_auth_vector(
                context.clone(),
                &AuthVectorReq {
                    user_id: user_id.to_string(),
                },
                signed_request_bytes,
            )
            .await?;

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
        message: SignedMessage,
    ) -> Result<tonic::Response<GetKeyShareResp>, DauthError> {
        if let SignPayloadType::GetKeyShareReq(payload) = verify_result {
            let mut signed_request_bytes = Vec::new();
            message.encode(&mut signed_request_bytes)?;

            let request_hash = payload.hash.ok_or(DauthError::InvalidMessageError(
                "Missing res(star) hash".to_string(),
            ))?;
            let _request_preimage = payload.preimage.ok_or(DauthError::InvalidMessageError(
                "Missing res(star) hash".to_string(),
            ))?;

            // TODO(matt9j) This was supposed to be the actual signed share from
            // the host, not constructed on demand in the backup network's
            // signing key as done below : /
            let key_share = match request_hash {
                get_key_share_req::payload::Hash::XresStarHash(xres_star_hash) => {
                    backup::get_key_share_5g(
                        context.clone(),
                        xres_star_hash[..].try_into()?,
                        &signed_request_bytes,
                    )
                    .await?
                }
                get_key_share_req::payload::Hash::XresHash(xres_hash) => {
                    backup::get_key_share_eps(
                        context.clone(),
                        xres_hash.as_slice().try_into()?,
                        &signed_request_bytes,
                    )
                    .await?
                }
            };

            let payload = delegated_confirmation_share::Payload {
                xres_star_hash: key_share.xres_star_hash.to_vec(),
                xres_hash: key_share.xres_hash.to_vec(),
                kseaf_confirmation_share: key_share.kseaf_share.to_vec(),
                kasme_confirmation_share: key_share.kasme_share.to_vec(),
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

        let old_xres_star_hash: XResStarHash =
            request.replaced_share_xres_star_hash[..].try_into()?;

        let new_key_share = utilities::handle_key_share(context.clone(), dshare).await?;

        backup::replace_key_share(context, &old_xres_star_hash, &new_key_share).await?;

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
                backup::withdraw_backup(context, &user_id, &payload.home_network_id).await?;
                Ok(tonic::Response::new(WithdrawBackupResp {}))
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
            backup::withdraw_shares(
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

                    backup::flood_vector(
                        context.clone(),
                        &utilities::handle_delegated_vector(&context, dvector, &user_id).await?,
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
