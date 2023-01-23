use std::sync::Arc;

use crate::data::{context::DauthContext, error::DauthError};
use crate::database;
use crate::database::utilities::DauthDataUtilities;

/// Removes the user from being backup up on this network.
/// Also removes all related auth vectors.
pub async fn withdraw_backup(
    context: Arc<DauthContext>,
    user_id: &str,
    home_network_id: &str,
) -> Result<(), DauthError> {
    tracing::info!("Withdrawing backup for user: {:?}", user_id);

    let mut transaction = context.local_context.database_pool.begin().await?;
    let actual_network_id = database::backup_users::get(&mut transaction, user_id)
        .await?
        .to_backup_user_home_network_id()?;
    transaction.commit().await?;

    if actual_network_id != home_network_id {
        Err(DauthError::InvalidMessageError(format!(
            "Not the correct home network",
        )))
    } else {
        transaction = context.local_context.database_pool.begin().await?;
        database::backup_users::remove(&mut transaction, user_id, home_network_id).await?;
        database::auth_vectors::remove_all(&mut transaction, user_id).await?;
        transaction.commit().await?;

        Ok(())
    }
}
