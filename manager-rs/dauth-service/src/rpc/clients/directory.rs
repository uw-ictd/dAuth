use std::sync::Arc;

use ed25519_dalek::PublicKey;

use crate::data::context::DauthContext;
use crate::data::error::DauthError;
use crate::rpc::dauth::directory::directory_client::DirectoryClient;
use crate::rpc::dauth::directory::{LookupUserReq, LooukupNetworkReq, RegisterReq, UpsertUserReq};

/// Registers this network with the directory service.
/// Provides this network's id, address, and public key.
pub async fn register(context: Arc<DauthContext>) -> Result<(), DauthError> {
    check_connection(context.clone()).await?;

    match &mut *(context.rpc_context.directory_client.lock().await) {
        Some(client) => {
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
        None => Err(DauthError::ClientError(
            "Directory client does not exist!".to_string(),
        )),
    }
}

/// Contacts directory service to find the address
/// and public key of the provided network id
/// Returns pair (address, public key)
pub async fn lookup_network(
    context: Arc<DauthContext>,
    network_id: &str,
) -> Result<(String, PublicKey), DauthError> {
    check_connection(context.clone()).await?;

    match &mut *(context.rpc_context.directory_client.lock().await) {
        Some(client) => {
            let response = client
                .lookup_network(LooukupNetworkReq {
                    network_id: network_id.to_string(),
                })
                .await?
                .into_inner();

            Ok((
                response.address,
                PublicKey::from_bytes(&response.public_key)?,
            ))
        }
        None => Err(DauthError::ClientError(
            "Directory client does not exist!".to_string(),
        )),
    }
}

/// Contacts directory service to find the home network
/// and the backup networks of the provided user.
/// Returns pair (home nework, vec<backup networks>)
pub async fn lookup_user(
    context: Arc<DauthContext>,
    user_id: &str,
) -> Result<(String, Vec<String>), DauthError> {
    check_connection(context.clone()).await?;

    match &mut *(context.rpc_context.directory_client.lock().await) {
        Some(client) => {
            let response = client
                .lookup_user(LookupUserReq {
                    user_id: user_id.to_string(),
                })
                .await?
                .into_inner();

            Ok((response.home_network_id, response.backup_network_ids))
        }
        None => Err(DauthError::ClientError(
            "Directory client does not exist!".to_string(),
        )),
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
    check_connection(context.clone()).await?;

    match &mut *(context.rpc_context.directory_client.lock().await) {
        Some(client) => {
            client
                .upsert_user(UpsertUserReq {
                    user_id: user_id.to_string(),
                    home_network_id: context.local_context.id.clone(),
                    backup_network_ids,
                })
                .await?;

            Ok(())
        }
        None => Err(DauthError::ClientError(
            "Directory client does not exist!".to_string(),
        )),
    }
}

pub async fn check_connection(context: Arc<DauthContext>) -> Result<(), DauthError> {
    let mut client = context.rpc_context.directory_client.lock().await;
    if client.is_none() {
        *client = Some(
            DirectoryClient::connect(format!("http://{}", context.rpc_context.directory_addr))
                .await?,
        );
    }
    Ok(())
}
