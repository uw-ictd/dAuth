use crate::data::error::DirectoryError;

/*  Manager handles all functionality of the directory service.
 *  Shares a 1:1 relation with the RPC handler.
 */

/// Registers a network with the directory.
/// Stores the networks address and public key.
pub async fn register(network_id: String, address: String, public_key: Vec<u8>) -> Result<(), DirectoryError> {
    todo!()
}

/// Looks up a network by id and checks if it has been registered.
/// Returns the address and public key of the network.
pub async fn lookup_network(network_id: String) -> Result<(String, Vec<u8>), DirectoryError> {
    todo!()
}

/// Looks up a user by id.
/// Returns the home network id and set of backup network ids.
pub async fn lookup_user(user_id: String) -> Result<(String, Vec<String>), DirectoryError> {
    todo!()
}

/// Stores the user with the provided home network and set of
/// backup networks.
/// If the user does not exist, the home network become the owner.
/// If the user already exists, the home network must be the owner
/// and the user info will be updated.
pub async fn upsert_user(user_id: String, home_network_id: String, backup_network_ids: Vec<String>) {
    todo!()
}
