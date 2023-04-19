use std::collections::HashMap;
use std::sync::Arc;

use crate::data::config::UserInfoConfig;
use crate::data::context::DauthContext;
use crate::data::error::DauthError;
use crate::database;
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
        let user_id = add_user_req.user_id;
        let mut sqn_slice_max = HashMap::new();
        let mut backup_network_ids = HashMap::new();

        for backup in add_user_req.backups {
            sqn_slice_max.insert(backup.slice, backup.sqn_max);
            backup_network_ids.insert(backup.backup_id, backup.slice);
        }

        let user_info = UserInfoConfig {
            k: add_user_req.k,
            opc: add_user_req.opc,
            sqn_slice_max,
            backup_network_ids,
        };

        let mut transaction = context.local_context.database_pool.begin().await?;
        database::user_infos::upsert(
            &mut transaction,
            &user_id,
            &user_info.get_k()?,
            &user_info.get_opc()?,
            add_user_req.sqn_max,
            0, // home network
        )
        .await?;

        database::tasks::update_users::add(
            &mut transaction,
            &user_id,
            0,
            &context.local_context.id,
        )
        .await?;

        for (backup_network_id, sqn_slice) in &user_info.backup_network_ids {
            database::user_infos::upsert(
                &mut transaction,
                &user_id,
                &user_info.get_k()?,
                &user_info.get_opc()?,
                *user_info
                    .sqn_slice_max
                    .get(sqn_slice)
                    .ok_or(DauthError::ConfigError(format!(
                        "Missing key slice for {}",
                        sqn_slice
                    )))?,
                *sqn_slice,
            )
            .await?;

            database::tasks::update_users::add(
                &mut transaction,
                &user_id,
                *sqn_slice,
                &backup_network_id,
            )
            .await?;
        }

        transaction.commit().await?;

        Ok(())
    }
}
