use std::sync::Arc;

use crate::data::{context::DauthContext, error::DauthError};
use crate::database;

/// Removes the user from being backup up on this network.
/// Also removes all related auth vectors.
#[tracing::instrument(skip(context), name = "backup::withdraw_backup")]
pub async fn withdraw_backup(
    context: Arc<DauthContext>,
    user_id: &str,
    home_network_id: &str,
) -> Result<(), DauthError> {
    tracing::info!("Withdrawing backup");

    let mut transaction = context.local_context.database_pool.begin().await?;
    let actual_network_id = database::backup_users::get(&mut transaction, user_id).await;
    transaction.commit().await?;

    if let Ok(actual_network_id) = actual_network_id {
        if actual_network_id != home_network_id {
            Err(DauthError::InvalidMessageError(format!(
                "Not the correct home network",
            )))
        } else {
            transaction = context.local_context.database_pool.begin().await?;
            database::backup_users::remove(&mut transaction, user_id, home_network_id).await?;
            database::auth_vectors::remove_all(&mut transaction, user_id).await?;
            database::key_shares::remove_all(&mut transaction, user_id).await?;
            transaction.commit().await?;
    
            Ok(())
        }
    } else {
        tracing::warn!(?user_id, "User is not being backed up by this network");
        Ok(())
    }
}
