use std::sync::Arc;

use crate::data::{config::UserInfoConfig, context::DauthContext, error::DauthError};
use crate::database;

/// Adds a new user to this network.
pub async fn add_user(
    context: Arc<DauthContext>,
    user_info: &UserInfoConfig,
) -> Result<(), DauthError> {
    let mut transaction = context.local_context.database_pool.begin().await?;
    let user_exists = database::user_infos::get(&mut transaction, &user_info.user_id, 0).await.is_ok();
    transaction.commit().await?;

    // Get all backups that need to have their old vectors removed
    let existing_backups = if user_exists{
        transaction = context.local_context.database_pool.begin().await?;
        let backups = database::backup_networks::get_all(&mut transaction, &user_info.user_id).await?;
        transaction.commit().await?;

        backups
    } else {
        Vec::new()
    };

    transaction = context.local_context.database_pool.begin().await?;

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
        0,
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

        let withdraw = if existing_backups.contains(&backup.backup_id) {
            1
        } else{
            0
        };

        database::tasks::update_users::add(
            &mut transaction,
            &user_info.user_id,
            backup.sqn_slice,
            &backup.backup_id,
            withdraw,
        )
        .await?;
    }

    transaction.commit().await?;

    Ok(())
}
