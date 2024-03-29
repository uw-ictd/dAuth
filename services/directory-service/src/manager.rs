use std::sync::Arc;

use crate::data::{context::DirectoryContext, error::DirectoryError};
use crate::database;

/*  Manager handles all functionality of the directory service.
 *  Shares a 1:1 relation with the RPC handler.
 */

/// Registers a network with the directory.
/// Stores the networks address and public key.
pub async fn register(
    context: Arc<DirectoryContext>,
    network_id: &str,
    address: &str,
    public_key: &Vec<u8>,
) -> Result<(), DirectoryError> {
    tracing::info!(
        "Register called: {:?}-{:?}-{:?}",
        network_id,
        address,
        public_key
    );

    let mut transaction = context.database_pool.begin().await?;
    database::networks::upsert(&mut transaction, network_id, address, public_key).await?;
    transaction.commit().await?;

    Ok(())
}

/// Looks up a network by id and checks if it has been registered.
/// Returns the address and public key of the network.
pub async fn lookup_network(
    context: Arc<DirectoryContext>,
    network_id: &str,
) -> Result<(String, Vec<u8>), DirectoryError> {
    tracing::info!("Lookup network called: {:?}", network_id);

    let mut transaction = context.database_pool.begin().await?;
    let res = database::networks::get(&mut transaction, network_id).await?;
    transaction.commit().await?;

    Ok(res)
}

/// Looks up a user by id.
/// Returns the home network id and set of backup network ids.
pub async fn lookup_user(
    context: Arc<DirectoryContext>,
    user_id: &str,
) -> Result<(String, Vec<String>), DirectoryError> {
    tracing::info!("Lookup user called: {:?}", user_id);

    let mut transaction = context.database_pool.begin().await?;
    let home_network_id = database::users::get(&mut transaction, user_id).await?;

    let backup_network_ids = database::backups::get(&mut transaction, user_id).await?;
    transaction.commit().await?;

    Ok((home_network_id, backup_network_ids))
}

/// Stores the user with the provided home network and set of
/// backup networks.
/// If the user does not exist, the home network become the owner.
/// If the user already exists, the home network must be the owner
/// and the user info will be updated.
pub async fn upsert_user(
    context: Arc<DirectoryContext>,
    user_id: &str,
    home_network_id: &str,
    backup_network_ids: &Vec<String>,
) -> Result<(), DirectoryError> {
    tracing::info!(
        "Upsert user called: {:?}-{:?}-{:?}",
        user_id,
        home_network_id,
        backup_network_ids
    );

    let mut transaction = context.database_pool.begin().await?;

    if let Ok(network_id) = database::users::get(&mut transaction, user_id).await {
        if home_network_id == network_id {
            database::backups::remove(&mut transaction, user_id).await?;
        } else {
            return Err(DirectoryError::InvalidAccess(
                "User owned by another network".to_string(),
            ));
        }
    } else {
        database::users::add(&mut transaction, user_id, home_network_id).await?;
    }

    for backup_network_id in backup_network_ids {
        database::backups::add(&mut transaction, user_id, backup_network_id).await?
    }

    transaction.commit().await?;
    Ok(())
}
