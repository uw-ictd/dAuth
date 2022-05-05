use std::sync::Arc;

use crate::data::context::DauthContext;
use crate::data::error::DauthError;
use crate::database;
use crate::rpc::clients::{backup_network, directory};

/// Runs the update user task.
/// Iterates through user in the user update table.
/// First registers each user with the directory service,
/// then enrolls all of the user's backup networks.
pub async fn run_task(context: Arc<DauthContext>) -> Result<(), DauthError> {
    let mut transaction = context.local_context.database_pool.begin().await.unwrap();
    let user_ids = database::tasks::update_users::get_user_ids(&mut transaction).await?;
    transaction.commit().await.unwrap();

    if user_ids.is_empty() {
        tracing::info!("Nothing to do for update user task");
    } else {
        tracing::info!("Found {} user update(s) pending", user_ids.len());
        for user_id in user_ids {
            if let Err(e) = handle_user_update(context.clone(), &user_id).await {
                tracing::warn!("Failed to handle user update: {}", e);
                // move on to next user id
            }
        }
    }
    Ok(())
}

/// Adds the user and its backup networks to the directory service.
/// Then, enrolls each of the backup networks.
async fn handle_user_update(context: Arc<DauthContext>, user_id: &str) -> Result<(), DauthError> {
    let mut transaction = context.local_context.database_pool.begin().await.unwrap();

    let user_data =
        database::tasks::update_users::get_user_data(&mut transaction, &user_id).await?;

    let mut backup_network_ids = Vec::new();

    for (backup_network_id, sqn_slice) in user_data {
        let (address, _) = directory::lookup_network(context.clone(), &backup_network_id).await?;
        backup_network::enroll_backup_prepare(
            context.clone(),
            user_id,
            &backup_network_id,
            &address,
        )
        .await?;

        // TODO: Add vector/key share generation
        backup_network::enroll_backup_commit(
            context.clone(),
            &backup_network_id,
            user_id,
            vec![],
            vec![],
            &address,
        )
        .await?;

        backup_network_ids.push(backup_network_id);
    }

    directory::upsert_user(context.clone(), &user_id, backup_network_ids).await?;

    database::tasks::update_users::remove(&mut transaction, &user_id).await?;

    transaction.commit().await.unwrap();
    Ok(())
}
