use std::sync::Arc;

use crate::data::{context::DauthContext, error::DauthError};
use crate::database;

/// Sets the provided user id as a being backed up by this network.
pub async fn set_backup_user(
    context: Arc<DauthContext>,
    user_id: &str,
    home_network_id: &str,
) -> Result<(), DauthError> {
    tracing::info!("Setting backup user: {:?} - {:?}", user_id, home_network_id);

    let mut transaction = context.local_context.database_pool.begin().await?;
    database::backup_users::add(&mut transaction, user_id, home_network_id).await?;
    transaction.commit().await?;
    Ok(())
}