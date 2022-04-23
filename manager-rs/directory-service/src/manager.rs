use crate::data::{context::DirectoryContext, error::DirectoryError};

/*  Manager handles all functionality of the directory service.
 *  Shares a 1:1 relation with the RPC handler.
 */

/// Registers a network with the directory.
/// Stores the networks address and public key.
pub async fn register(
    context: DirectoryContext,
    network_id: &str,
    address: &str,
    public_key: &Vec<u8>,
) -> Result<(), DirectoryError> {
    todo!()
}

/// Looks up a network by id and checks if it has been registered.
/// Returns the address and public key of the network.
pub async fn lookup_network(
    context: DirectoryContext,
    network_id: &str,
) -> Result<(String, Vec<u8>), DirectoryError> {
    todo!()
}

/// Looks up a user by id.
/// Returns the home network id and set of backup network ids.
pub async fn lookup_user(
    context: DirectoryContext,
    user_id: &str,
) -> Result<(String, Vec<String>), DirectoryError> {
    todo!()
}

/// Stores the user with the provided home network and set of
/// backup networks.
/// If the user does not exist, the home network become the owner.
/// If the user already exists, the home network must be the owner
/// and the user info will be updated.
pub async fn upsert_user(
    context: DirectoryContext,
    user_id: &str,
    home_network_id: &str,
    backup_network_ids: Vec<&str>,
) -> Result<(), DirectoryError> {
    todo!()
}
