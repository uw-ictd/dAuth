use std::sync::Arc;

use crate::data::config::{BackupConfig, UserInfoConfig};
use crate::data::context::DauthContext;
use crate::data::error::DauthError;
use crate::management;
use crate::rpc::dauth::management::management_server::Management;
use crate::rpc::dauth::management::{AddUserReq, CommandResp, RemoveUserReq};

pub struct ManagementHandler {
    pub context: Arc<DauthContext>,
}

#[tonic::async_trait]
impl Management for ManagementHandler {
    #[tracing::instrument(skip_all)]
    async fn add_user(
        &self,
        request: tonic::Request<AddUserReq>,
    ) -> Result<tonic::Response<CommandResp>, tonic::Status> {
        tracing::info!(?request, "Add user request");

        match self
            .add_user_hlp(self.context.clone(), request.into_inner())
            .await
        {
            Ok(()) => Ok(tonic::Response::new(CommandResp {
                successful: true,
                info: "".to_string(),
            })),
            Err(error) => {
                tracing::error!(?error, "Failed to add user");
                Ok(tonic::Response::new(CommandResp {
                    successful: false,
                    info: format!("Failed to add user: {:?}", error),
                }))
            }
        }
    }

    #[tracing::instrument(skip_all)]
    async fn remove_user(
        &self,
        request: tonic::Request<RemoveUserReq>,
    ) -> Result<tonic::Response<CommandResp>, tonic::Status> {
        tracing::info!(?request, "Remove user request");

        // TODO: Add support for removing users.
        // We can remove users locally, but there is not a way to remove users
        // from the directory service.

        Ok(tonic::Response::new(CommandResp {
            successful: false,
            info: "Removing users is not supported yet".to_string(),
        }))
    }
}

impl ManagementHandler {
    async fn add_user_hlp(
        &self,
        context: Arc<DauthContext>,
        add_user_req: AddUserReq,
    ) -> Result<(), DauthError> {
        let mut backups = Vec::new();
        for backup in add_user_req.backups {
            backups.push(BackupConfig {
                backup_id: backup.backup_id,
                sqn_slice: backup.slice,
                sqn_max: backup.sqn_max,
            });
        }

        let user_info = UserInfoConfig {
            user_id: add_user_req.user_id,
            k: add_user_req.k,
            opc: add_user_req.opc,
            sqn_max: add_user_req.sqn_max,
            backups,
        };

        management::add_user(context.clone(), &user_info).await?;

        Ok(())
    }
}
