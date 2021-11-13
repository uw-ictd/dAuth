use std::sync::Arc;

use crate::data::context::DauthContext;
use crate::rpc::d_auth::remote_authentication_client::RemoteAuthenticationClient;
use crate::rpc::d_auth::{AkaVectorReq, AkaVectorResp};

/// Send out request to remote core for new auth vector.
pub fn request_auth_vector_remote(
    context: Arc<DauthContext>,
    av_request: &AkaVectorReq,
) -> Option<AkaVectorResp> {
    tracing::info!("Sending remote request: {:?}", av_request);

    // Find the addr corresponding to the home network.
    let addr;
    match resolve_request_to_addr(context.clone(), av_request) {
        Some(res) => addr = res,
        None => return None,
    };

    // Initialize and add client stub first.
    match context
        .rpc_context
        .runtime_handle
        .block_on(add_client(context.clone(), &addr))
    {
        Ok(()) => (),
        Err(e) => {
            tracing::error!("Could not add client to list: {}", e);
            return None;
        }
    };

    context
        .rpc_context
        .runtime_handle
        .block_on(client_send_request(context.clone(), av_request, &addr))
}

async fn client_send_request(
    context: Arc<DauthContext>,
    av_request: &AkaVectorReq,
    addr: &String,
) -> Option<AkaVectorResp> {
    // Make client call.
    match context.rpc_context.client_stubs.lock() {
        Ok(mut client_stubs) => {
            match client_stubs.get_mut(addr) {
                Some(client) => {
                    match client
                        .get_auth_vector_remote(tonic::Request::new(av_request.clone()))
                        .await
                    {
                        Ok(resp) => {
                            let av_result = resp.into_inner();
                            if av_result.error == 0 {
                                // Should be ErrorKind
                                tracing::info!("Vector received from remote: {:?}", av_result);
                                Some(av_result)
                            } else {
                                tracing::info!(
                                    "Remote failed to make auth vector: {:?}",
                                    av_result
                                );
                                None
                            }
                        }
                        Err(e) => {
                            tracing::error!("Failed to send request for {:?}: {}", av_request, e);
                            None
                        }
                    }
                }
                None => {
                    tracing::error!(
                        "Client stub not found for {:?} (should have been added)",
                        av_request
                    );
                    None
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to get mutex {}", e);
            None
        }
    }
}

/// Broadcast to all other cores that an auth vector was used.
pub fn broadcast_auth_vector_used(context: Arc<DauthContext>, av_result: &AkaVectorResp) {
    tracing::info!("Broadcasting usage: {:?}", av_result);
    for addr in &context.remote_context.remote_addrs {
        // Initialize and add client stub first.
        match context
            .rpc_context
            .runtime_handle
            .block_on(add_client(context.clone(), addr))
        {
            Ok(()) => (),
            Err(e) => {
                tracing::error!("Could not add client to list: {}", e);
                continue;
            }
        };

        match context
            .rpc_context
            .runtime_handle
            .block_on(client_send_usage(context.clone(), av_result, addr))
        {
            Ok(()) => (),
            Err(e) => tracing::error!("Failed to send usage message to {}: {}", addr, e),
        }
    }
}

async fn client_send_usage(
    context: Arc<DauthContext>,
    av_result: &AkaVectorResp,
    addr: &String,
) -> Result<(), String> {
    match context.rpc_context.client_stubs.lock() {
        Ok(mut client_stubs) => match client_stubs.get_mut(addr) {
            Some(client) => {
                match client
                    .report_used_auth_vector(tonic::Request::new(av_result.clone()))
                    .await
                {
                    Ok(_) => {
                        tracing::info!("Successfully sent usage message to {}", addr);
                        Ok(())
                    }
                    Err(e) => Err(format!("Failed to send request: {}", e)),
                }
            }
            None => Err(format!("Client stub not found (should have been added)")),
        },
        Err(e) => Err(format!("Failed to get mutex {}", e)),
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
async fn add_client(context: Arc<DauthContext>, addr: &String) -> Result<(), &'static str> {
    match context.rpc_context.client_stubs.lock() {
        Ok(mut client_stubs) => {
            if !client_stubs.contains_key(addr) {
                match RemoteAuthenticationClient::connect(format!("http://{}", addr)).await {
                    Ok(client) => {
                        client_stubs.insert(addr.clone(), client);
                        tracing::info!("New client created for address: {}", addr);
                        Ok(())
                    }
                    Err(e) => {
                        tracing::error!("Failed to connect to server: {}", e);
                        Err("Failed to connect to server, client not created")
                    }
                }
            } else {
                Ok(())
            }
        }
        Err(e) => {
            tracing::error!("Failed to get mutex {}", e);
            Err("Failed to get mutex")
        }
    }
}
