use std::sync::Arc;

use crate::data::context::DauthContext;
use crate::data::vector::AuthVectorReq;
use crate::local;
use crate::rpc::dauth::local::aka_confirm_resp;
use crate::rpc::dauth::local::local_authentication_server::LocalAuthentication;
use crate::rpc::dauth::local::{AkaConfirmReq, AkaConfirmResp, AkaVectorReq, AkaVectorResp};
use crate::rpc::dauth::remote::{home_network_server::HomeNetwork, backup_network_server::BackupNetwork};
use crate::rpc::dauth::remote::*;

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
        todo!();

        // match remote::manager::auth_vector_get_remote(self.context.clone(), &av_request).await {
        //     Ok(av_result) => {
        //         tracing::info!("Returning result: {:?}", av_result);
        //         Ok(tonic::Response::new(av_result))
        //     }
        //     Err(e) => {
        //         tracing::error!("Error while handling request for {:?}: {}", av_request, e);
        //         Err(tonic::Status::new(tonic::Code::Aborted, e.to_string()))
        //     }
        // }
    }

    /// Remote alert that a vector has been used
    async fn get_confirm_key(
        &self,
        request: tonic::Request<GetHomeConfirmKeyReq>,
    ) -> Result<tonic::Response<GetHomeConfirmKeyResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);

        todo!();

        // match remote::manager::auth_vector_used_remote(context.clone(), &av_result).await {
        //     Ok(()) => {
        //         tracing::info!("Successfuly reported used: {:?}", av_result);
        //         Ok(tonic::Response::new(AkaVectorUsedResp {}))
        //     }
        //     Err(e) => {
        //         tracing::error!("Error reporting used: {}", e);
        //         Err(tonic::Status::new(tonic::Code::Aborted, e.to_string()))
        //     }
        // }
    }
}

#[tonic::async_trait]
impl BackupNetwork for DauthHandler {
    async fn enroll_backup_prepare(
        &self,
        request: tonic::Request<EnrollBackupPrepareReq>
    ) -> Result<tonic::Response<EnrollBackupPrepareResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);

        todo!();
    }

    async fn enroll_backup_commit(
        &self,
        request: tonic::Request<EnrollBackupCommitReq>
    ) -> Result<tonic::Response<EnrollBackupCommitResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);

        todo!();
    }

    async fn get_auth_vector(
        &self,
        request: tonic::Request<GetBackupAuthVectorReq>
    ) -> Result<tonic::Response<GetBackupAuthVectorResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);

        todo!();
    }

    async fn get_key_share(
        &self,
        request: tonic::Request<GetKeyShareReq>
    ) -> Result<tonic::Response<GetKeyShareResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);

        todo!();
    }

    async fn withdraw_backup(
        &self,
        request: tonic::Request<WithdrawBackupReq>
    ) -> Result<tonic::Response<WithdrawBackupResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);

        todo!();
    }

    async fn withdraw_shares(
        &self,
        request: tonic::Request<WithdrawSharesReq>
    ) -> Result<tonic::Response<WithdrawSharesResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);

        todo!();
    }

    async fn flood_vector(
        &self,
        request: tonic::Request<FloodVectorReq>
    ) -> Result<tonic::Response<FloodVectorResp>, tonic::Status> {
        tracing::info!("Request: {:?}", request);

        todo!();
    }
}
