use std::sync::Arc;

use ed25519_dalek::PublicKey;

use crate::data::context::DauthContext;
use crate::data::error::DauthError;

/// Contacts directory service to find the address
/// and public key of the provided network id
/// Returns pair (address, public key)
pub async fn lookup_network(
    context: Arc<DauthContext>,
    network_id: &str,
) -> Result<(String, PublicKey), DauthError> {
    todo!()
}

/// Contacts directory service to find the home network
/// and the backup networks of the provided user.
/// Returns pair (home nework, vec<backup networks>)
pub async fn lookup_user(
    context: Arc<DauthContext>,
    user_id: &str,
) -> Result<(String, Vec<String>), DauthError> {
    todo!()
}
