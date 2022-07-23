use std::sync::Arc;

use ed25519_dalek::PublicKey;
use tonic::transport::Channel;

use crate::data::context::DauthContext;
use crate::data::error::DauthError;
use crate::rpc::dauth::directory::directory_client::DirectoryClient;
use crate::rpc::dauth::directory::{LookupUserReq, LooukupNetworkReq, RegisterReq, UpsertUserReq};

/// Registers this network with the directory service.
/// Provides this network's id, address, and public key.
pub async fn register(context: Arc<DauthContext>) -> Result<(), DauthError> {
    let mut client = get_client(context.clone()).await?;

    client
        .register(RegisterReq {
            network_id: context.local_context.id.clone(),
            address: context.rpc_context.host_addr.clone(),
            public_key: context
                .local_context
                .signing_keys
                .public
                .as_bytes()
                .to_vec(),
        })
        .await?;
    Ok(())
}

/// Contacts directory service to find the address
/// and public key of the provided network id
/// Returns pair (address, public key)
pub async fn lookup_network(
    context: &Arc<DauthContext>,
    network_id: &str,
) -> Result<(String, PublicKey), DauthError> {
    // Acquire the lock and attempt to look up the network information.
    {
        let cache = context.backup_context.directory_network_cache.lock().await;
        match cache.get(network_id) {
            Some(cached) => {
                return Ok(cached.clone());
            }
            None => {
                // Fall through to lookup the network information
            }
        }
    }

    // No cached info was found, so look it up
    let mut client = get_client(context.clone()).await?;

    let response = client
        .lookup_network(LooukupNetworkReq {
            network_id: network_id.to_string(),
        })
        .await?
        .into_inner();

    let res = (response.address,
        PublicKey::from_bytes(&response.public_key)?,);

    // Re-acquire the lock and update the cache
    {
        let mut cache = context.backup_context.directory_network_cache.lock().await;
        cache.insert(network_id.to_string(), res.clone());
        Ok(res)
    }
}

/// Contacts directory service to find the home network
/// and the backup networks of the provided user.
/// Returns pair (home nework, vec<backup networks>)
pub async fn lookup_user(
    context: &Arc<DauthContext>,
    user_id: &str,
) -> Result<(String, Vec<String>), DauthError> {
    // Acquire the lock and attempt to look up the user information.
    {
        let cache = context.backup_context.directory_user_cache.lock().await;
        match cache.get(user_id) {
            Some(cached) => {
                return Ok(cached.clone());
            }
            None => {
                // Fall through to lookup the user information
            }
        }
    }

    // No cached info was found, so look it up
    let mut client = get_client(context.clone()).await?;

    let response = client
        .lookup_user(LookupUserReq {
            user_id: user_id.to_string(),
        })
        .await?
        .into_inner();

    let res = (response.home_network_id, response.backup_network_ids);

    // Re-acquire the lock and update the cache
    {
        let mut cache = context.backup_context.directory_user_cache.lock().await;
        cache.insert(user_id.to_string(), res.clone());
        Ok(res)
    }
}

/// Sends user info to the directory service.
/// If the user doesn't exist, this network claims ownership.
/// Otherwise, user info is updated iff this network owns the user.
pub async fn upsert_user(
    context: Arc<DauthContext>,
    user_id: &str,
    backup_network_ids: Vec<String>,
) -> Result<(), DauthError> {
    let mut client = get_client(context.clone()).await?;

    client
        .upsert_user(UpsertUserReq {
            user_id: user_id.to_string(),
            home_network_id: context.local_context.id.clone(),
            backup_network_ids,
        })
        .await?;

    Ok(())
}

/// Returns a client to the directory service at the provided address.
/// Builds and caches the client if one does not exist.
async fn get_client(context: Arc<DauthContext>) -> Result<DirectoryClient<Channel>, DauthError> {
    let mut client_option = context.rpc_context.directory_client.lock().await;
    if client_option.is_none() {
        *client_option = Some(
            DirectoryClient::connect(format!("http://{}", context.rpc_context.directory_addr))
                .await?,
        );
    }

    Ok(client_option
        .clone()
        .ok_or(DauthError::ClientError("Client not found".to_string()))?)
}
