use std::sync::Arc;

use crate::data::{context::DauthContext, error::DauthError};
use crate::rpc::dauth::common::remote_authentication_client::RemoteAuthenticationClient;
use crate::rpc::dauth::common::{AkaVectorReq, AkaVectorResp};

/// Send out request to remote core for new auth vector.
pub async fn request_auth_vector_remote(
    context: Arc<DauthContext>,
    av_request: &AkaVectorReq,
) -> Result<AkaVectorResp, DauthError> {
    tracing::info!("Sending remote request: {:?}", av_request);

    // Find the addr corresponding to the home network.
    let addr;
    match resolve_request_to_addr(context.clone(), av_request) {
        Some(res) => addr = res,
        None => {
            return Err(DauthError::ClientError(format!(
                "Could not find valid address for request"
            )))
        }
    };

    // Initialize and add client stub first.
    match add_client(context.clone(), &addr).await {
        Ok(()) => (),
        Err(e) => {
            tracing::error!("Could not add client to list: {}", e);
            return Err(DauthError::ClientError(format!(
                "Could not create client stub to send request"
            )));
        }
    };

    client_send_request(context.clone(), av_request, &addr).await
}

async fn client_send_request(
    context: Arc<DauthContext>,
    av_request: &AkaVectorReq,
    addr: &String,
) -> Result<AkaVectorResp, DauthError> {
    match context.rpc_context.client_stubs.lock().await.get_mut(addr) {
        Some(client) => {
            match client
                .get_auth_vector_remote(tonic::Request::new(av_request.clone()))
                .await
            {
                Ok(resp) => Ok(resp.into_inner()),
                Err(e) => {
                    tracing::error!("Failed to send request for {:?}: {}", av_request, e);
                    Err(DauthError::ClientError(format!(
                        "Failed to send request: {}",
                        e
                    )))
                }
            }
        }
        None => {
            tracing::error!(
                "Client stub not found for {:?} (should have been added)",
                av_request
            );
            Err(DauthError::ClientError(format!(
                "Client stub not found (should have been added)"
            )))
        }
    }
}

/// Broadcast to all other cores that an auth vector was used.
pub async fn broadcast_auth_vector_used(context: Arc<DauthContext>, av_result: &AkaVectorResp) {
    tracing::info!("Broadcasting usage: {:?}", av_result);
    for addr in &context.remote_context.remote_addrs {
        // Initialize and add client stub first.
        match add_client(context.clone(), addr).await {
            Ok(()) => (),
            Err(e) => {
                tracing::error!("Could not added client to list: {}", e);
                continue;
            }
        };

        match client_send_usage(context.clone(), av_result, addr).await {
            Ok(()) => (),
            Err(e) => tracing::error!("Failed to send usage message to {}: {}", addr, e),
        }
    }
}

async fn client_send_usage(
    context: Arc<DauthContext>,
    av_result: &AkaVectorResp,
    addr: &String,
) -> Result<(), DauthError> {
    match context.rpc_context.client_stubs.lock().await.get_mut(addr) {
        Some(client) => {
            match client
                .report_used_auth_vector(tonic::Request::new(av_result.clone()))
                .await
            {
                Ok(_) => {
                    tracing::info!("Successfully sent usage message to {}", addr);
                    Ok(())
                }
                Err(e) => {
                    tracing::error!("Failed to send request: {}", e);
                    Err(DauthError::ClientError(format!(
                        "Failed to send request: {}",
                        e
                    )))
                }
            }
        }
        None => Err(DauthError::ClientError(format!(
            "Client stub not found (should have been added)"
        ))),
    }
}

/// Determines address of the home network of the request.
fn resolve_request_to_addr(
    context: Arc<DauthContext>,
    _av_request: &AkaVectorReq,
) -> Option<String> {
    // TODO(nickfh7) Add logic to resolve to address.
    // This may be a long way down the road.
    match context.remote_context.remote_addrs.get(0) {
        Some(addr) => Some(addr.clone()),
        None => None,
    }
}

/// Adds a client to the current context if it doesn't already exist.
/// Otherwise, does nothing.
async fn add_client(context: Arc<DauthContext>, addr: &String) -> Result<(), DauthError> {
    let mut client_stubs = context.rpc_context.client_stubs.lock().await;

    if !client_stubs.contains_key(addr) {
        match RemoteAuthenticationClient::connect(format!("http://{}", addr)).await {
            Ok(client) => {
                client_stubs.insert(addr.clone(), client);
                tracing::info!("New client created for address: {}", addr);
                Ok(())
            }
            Err(e) => {
                tracing::error!("Failed to connect to server: {}", e);
                Err(DauthError::ClientError(format!(
                    "Failed to connect to server: {}",
                    e
                )))
            }
        }
    } else {
        Ok(())
    }
}
