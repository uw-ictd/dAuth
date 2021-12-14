use std::sync::Arc;

use crate::data::{context::DauthContext, error::DauthError, vector::AuthVectorRes};
use crate::rpc::dauth::local::AkaVectorResp;
use crate::rpc::dauth::remote::home_network_client::HomeNetworkClient;
use crate::rpc::dauth::remote::{
    GetHomeAuthVectorReq, GetHomeAuthVectorResp, GetHomeConfirmKeyReq,
};

/// Send out request to remote core for new auth vector.
pub async fn request_auth_vector_remote(
    context: Arc<DauthContext>,
    av_request: &GetHomeAuthVectorReq,
) -> Result<GetHomeAuthVectorResp, DauthError> {
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
    av_request: &GetHomeAuthVectorReq,
    addr: &String,
) -> Result<GetHomeAuthVectorResp, DauthError> {
    match context.rpc_context.client_stubs.lock().await.get_mut(addr) {
        Some(client) => {
            match client
                .get_auth_vector(tonic::Request::new(av_request.clone()))
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
pub async fn broadcast_auth_vector_used(context: Arc<DauthContext>, av_result: &AuthVectorRes) {
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

        match client_send_usage(context.clone(), &av_result.to_resp(), addr).await {
            Ok(()) => (),
            Err(e) => tracing::error!("Failed to send usage message to {}: {}", addr, e),
        }
    }
}

async fn client_send_usage(
    context: Arc<DauthContext>,
    _av_result: &AkaVectorResp,
    addr: &String,
) -> Result<(), DauthError> {
    unimplemented!();
    // match context.rpc_context.client_stubs.lock().await.get_mut(addr) {
    //     Some(client) => {
    //         match client
    //             .get_confirm_key(tonic::Request::new(GetHomeConfirmKeyReq {
    //                 payload: Some(
    //                     crate::rpc::dauth::remote::get_home_confirm_key_req::Payload {
    //                         kind: crate::rpc::dauth::remote::SignedMessageKind::GetHomeConfirmKeyReq
    //                             as i32,
    //                         serving_network_id: "Test".to_string(),
    //                         res_star: vec![0],
    //                         hash_xres_star: vec![0],
    //                     },
    //                 ),
    //                 serving_network_signature: vec![0],
    //             }))
    //             .await
    //         {
    //             Ok(_) => {
    //                 tracing::info!("Successfully sent usage message to {}", addr);
    //                 Ok(())
    //             }
    //             Err(e) => {
    //                 tracing::error!("Failed to send request: {}", e);
    //                 Err(DauthError::ClientError(format!(
    //                     "Failed to send request: {}",
    //                     e
    //                 )))
    //             }
    //         }
    //     }
    //     None => Err(DauthError::ClientError(format!(
    //         "Client stub not found (should have been added)"
    //     ))),
    // }
}

/// Determines address of the home network of the request.
fn resolve_request_to_addr(
    context: Arc<DauthContext>,
    _av_request: &GetHomeAuthVectorReq,
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
        match HomeNetworkClient::connect(format!("http://{}", addr)).await {
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
