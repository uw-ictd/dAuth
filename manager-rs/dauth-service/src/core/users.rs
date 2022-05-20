use std::sync::Arc;

use crate::data::{context::DauthContext, error::DauthError};
use crate::database;
use crate::database::utilities::DauthDataUtilities;

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

/// Gets the home network of the provided user id.
/// Fails if the user is not being backed up by this network.
pub async fn get_backup_user(
    context: Arc<DauthContext>,
    user_id: &str,
) -> Result<String, DauthError> {
    tracing::info!("Getting backup user: {:?}", user_id);

    let mut transaction = context.local_context.database_pool.begin().await?;
    let res = database::backup_users::get(&mut transaction, user_id)
        .await?
        .to_backup_user_home_network_id()?;
    transaction.commit().await?;
    Ok(res)
}

/// Removes the user from being backup up on this network.
/// Also removes all related auth vectors.
pub async fn remove_backup_user(
    context: Arc<DauthContext>,
    user_id: &str,
    home_network_id: &str,
) -> Result<(), DauthError> {
    tracing::info!("Getting backup user: {:?}", user_id);

    let mut transaction = context.local_context.database_pool.begin().await?;
    database::backup_users::remove(&mut transaction, user_id, home_network_id).await?;
    database::auth_vectors::remove_all(&mut transaction, user_id).await?;
    transaction.commit().await?;
    Ok(())
}

/// Gets the seqnum slice assigned to a backup network for a user.
pub async fn get_seqnum_slice(
    context: Arc<DauthContext>,
    user_id: &str,
    network_id: &str,
) -> Result<i64, DauthError> {
    if network_id == context.local_context.id {
        Ok(0)
    } else {
        let mut transaction = context.local_context.database_pool.begin().await?;
        let slice =
            database::backup_networks::get_slice(&mut transaction, user_id, network_id).await?;
        transaction.commit().await?;
        Ok(slice)
    }
}
