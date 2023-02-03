use std::sync::Arc;

use crate::data::{context::DauthContext, error::DauthError};
use crate::database;

/// Sets the provided user id as a being backed up by this network.
#[tracing::instrument(skip(context), name = "backup::enroll_backup_prepare")]
pub async fn enroll_backup_prepare(
    context: Arc<DauthContext>,
    user_id: &str,
    home_network_id: &str,
) -> Result<(), DauthError> {
    tracing::info!("Preparing backup enrollment");

    let mut transaction = context.local_context.database_pool.begin().await?;
    database::backup_users::add(&mut transaction, user_id, home_network_id).await?;
    transaction.commit().await?;

    Ok(())
}
