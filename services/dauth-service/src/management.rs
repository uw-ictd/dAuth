use std::sync::Arc;

use crate::data::{config::UserInfoConfig, context::DauthContext, error::DauthError};
use crate::database;

/// Adds a new user to this network.
pub async fn add_user(
    context: Arc<DauthContext>,
    user_info: &UserInfoConfig,
) -> Result<(), DauthError> {
    let mut transaction = context.local_context.database_pool.begin().await?;

    database::user_infos::upsert(
        &mut transaction,
        &user_info.user_id,
        &user_info.get_k()?,
        &user_info.get_opc()?,
        user_info.sqn_max,
        0, // home network
    )
    .await?;

    database::tasks::update_users::add(
        &mut transaction,
        &user_info.user_id,
        0,
        &context.local_context.id,
    )
    .await?;

    for backup in &user_info.backups {
        database::user_infos::upsert(
            &mut transaction,
            &user_info.user_id,
            &user_info.get_k()?,
            &user_info.get_opc()?,
            backup.sqn_max,
            backup.sqn_slice,
        )
        .await?;

        database::tasks::update_users::add(
            &mut transaction,
            &user_info.user_id,
            backup.sqn_slice,
            &backup.backup_id,
        )
        .await?;
    }

    transaction.commit().await?;

    Ok(())
}
