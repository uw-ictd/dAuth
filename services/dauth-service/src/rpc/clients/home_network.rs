use std::sync::Arc;
use std::time::Duration;

use auth_vector::types::{Kasme, Kseaf, Res, ResStar, XResHash, XResStarHash};
use prost::Message;
use tonic::transport::Channel;
use tonic::transport::Endpoint;

use crate::data::context::DauthContext;
use crate::data::error::DauthError;
use crate::data::signing::{self, SignPayloadType};
use crate::data::vector::AuthVectorRes;
use crate::rpc::dauth::common::UserIdKind;
use crate::rpc::dauth::remote::home_network_client::HomeNetworkClient;
use crate::rpc::dauth::remote::{
    get_home_auth_vector_req, get_home_confirm_key_req, get_home_confirm_key_resp,
    ReportHomeAuthConsumedReq, ReportHomeKeyShareConsumedReq, SignedMessage,
};
use crate::rpc::dauth::remote::{GetHomeAuthVectorReq, GetHomeConfirmKeyReq};
use crate::rpc::utilities;

/// Get an auth vector from a user's home network.
pub async fn get_auth_vector(
    context: Arc<DauthContext>,
    user_id: &str,
    address: &str,
    timeout: Duration,
) -> Result<AuthVectorRes, DauthError> {
    let mut client = get_client(context.clone(), address).await?;

    let mut request = tonic::Request::new(GetHomeAuthVectorReq {
        message: Some(signing::sign_message(
            context.clone(),
            SignPayloadType::GetHomeAuthVectorReq(get_home_auth_vector_req::Payload {
                serving_network_id: context.local_context.id.clone(),
                user_id_type: UserIdKind::Supi as i32,
                user_id: user_id.as_bytes().to_vec(),
            }),
        )),
    });

    request.set_timeout(timeout);

    let response = match client.get_auth_vector(request).await{
        Ok(res) => res.into_inner(),
        Err(status) => {
            if status.code() == tonic::Code::Unavailable {
                mark_endpoint_offline(&context, address).await;
            }
            return Err(status.into());
        }
    };

    let message = response
        .vector
        .ok_or(DauthError::ClientError(
            "Missing delegated vector".to_string(),
        ))?
        .message
        .ok_or(DauthError::ClientError(
            "Missing signed message".to_string(),
        ))?;

    if let SignPayloadType::DelegatedAuthVector5G(dvector) =
        signing::verify_message(&context, &message).await?
    {
        Ok(AuthVectorRes::from_av5_g(
            user_id,
            dvector
                .v
                .ok_or(DauthError::ClientError("Missing 5G vector".to_string()))?,
        )?)
    } else {
        Err(DauthError::ClientError(format!(
            "Incorrect message type received: {:?}",
            message
        )))
    }
}

/// Get the kseaf value at the end of an auth vector transaction.
pub async fn get_confirm_key_kseaf(
    context: Arc<DauthContext>,
    res_star: &ResStar,
    xres_star_hash: &XResStarHash,
    address: &str,
) -> Result<Kseaf, DauthError> {
    let mut client = get_client(context.clone(), address).await?;

    let response = match client
        .get_confirm_key(GetHomeConfirmKeyReq {
            message: Some(signing::sign_message(
                context.clone(),
                SignPayloadType::GetHomeConfirmKeyReq(get_home_confirm_key_req::Payload {
                    serving_network_id: context.local_context.id.clone(),
                    preimage: Some(get_home_confirm_key_req::payload::Preimage::ResStar(
                        res_star.to_vec(),
                    )),
                    hash: Some(get_home_confirm_key_req::payload::Hash::XresStarHash(
                        xres_star_hash.to_vec(),
                    )),
                }),
            )),
        })
        .await {
            Ok(res) => res.into_inner(),
            Err(status) => {
                if status.code() == tonic::Code::Unavailable {
                    mark_endpoint_offline(&context, address).await;
                }
                return Err(status.into());
            }
        };

    if let Some(get_home_confirm_key_resp::Key::Kseaf(kseaf)) = response.key {
        Ok(kseaf[..].try_into()?)
    } else {
        Err(DauthError::KeyTypeError(
            "Expected kseaf but unable to extract from message".to_string(),
        ))
    }
}

/// Get the kseaf value at the end of an auth vector transaction.
pub async fn get_confirm_key_kasme(
    context: Arc<DauthContext>,
    res: &Res,
    xres_hash: &XResHash,
    address: &str,
) -> Result<Kasme, DauthError> {
    let mut client = get_client(context.clone(), address).await?;

    let response = match client
        .get_confirm_key(GetHomeConfirmKeyReq {
            message: Some(signing::sign_message(
                context.clone(),
                SignPayloadType::GetHomeConfirmKeyReq(get_home_confirm_key_req::Payload {
                    serving_network_id: context.local_context.id.clone(),
                    preimage: Some(get_home_confirm_key_req::payload::Preimage::Res(
                        res.to_vec(),
                    )),
                    hash: Some(get_home_confirm_key_req::payload::Hash::XresHash(
                        xres_hash.to_vec(),
                    )),
                }),
            )),
        })
        .await {
            Ok(res) => res.into_inner(),
            Err(status) => {
                if status.code() == tonic::Code::Unavailable {
                    mark_endpoint_offline(&context, address).await;
                }
                return Err(status.into());
            }
        };

    if let Some(get_home_confirm_key_resp::Key::Kasme(kasme)) = response.key {
        Ok(kasme[..].try_into()?)
    } else {
        Err(DauthError::KeyTypeError(
            "Expected kasme but unable to extract from message".to_string(),
        ))
    }
}

/// Reports an auth vector as used to the home network.
/// Sends the original signed request for the auth vector as
/// proof of the auth vector request.
pub async fn report_auth_consumed(
    context: &Arc<DauthContext>,
    xres_star_hash: &XResStarHash,
    user_id: &str,
    original_request: &Vec<u8>,
    home_net_client: &mut HomeNetworkClient<Channel>,
) -> Result<Option<AuthVectorRes>, DauthError> {
    let signed_message = SignedMessage::decode(&original_request[..])?;

    let dvector = home_net_client
        .report_auth_consumed(ReportHomeAuthConsumedReq {
            backup_network_id: context.local_context.id.clone(),
            xres_star_hash: xres_star_hash.to_vec(),
            backup_auth_vector_req: Some(signed_message),
        })
        .await?
        .into_inner()
        .vector;

    match dvector {
        None => Ok(None),
        Some(dvector) => Ok(Some(
            utilities::handle_delegated_vector(context, dvector, user_id).await?,
        )),
    }
}

/// Reports a key share as used to the home network.
/// Sends the original signed request for the auth vector as
/// proof of the auth vector request.
pub async fn report_key_share_consumed(
    context: &Arc<DauthContext>,
    original_request: &Vec<u8>,
    home_net_client: &mut HomeNetworkClient<Channel>,
) -> Result<(), DauthError> {
    let signed_message = SignedMessage::decode(&original_request[..])?;

    // no key share is sent in return yet
    let _res = home_net_client
        .report_key_share_consumed(ReportHomeKeyShareConsumedReq {
            backup_network_id: context.local_context.id.clone(),
            get_key_share_req: Some(signed_message),
        })
        .await?
        .into_inner();

    Ok(())
}

/// Returns a client to the service at the provided address.
/// Builds and caches a client if one does not exist.
pub async fn get_client(
    context: Arc<DauthContext>,
    address: &str,
) -> Result<HomeNetworkClient<Channel>, DauthError> {
    // Acquire the lock and attempt to look up the client connection.
    {
        let clients = context.rpc_context.home_clients.lock().await;
        match clients.get(address) {
            Some(cached_client) => {
                return Ok(cached_client.clone());
            }
            None => {
                // Fall through to create a client handling
            }
        }
    }

    // No cached client was found, so attempt to open a connection

    // First check if this network is known to be offline
    {
        let mut offline_cache = context.rpc_context.known_offline_networks.lock().await;
        if let Some(retry_time) = offline_cache.get(address) {
            if &std::time::Instant::now() < retry_time {
                tracing::debug!(?address, "Attempted client connection to unavailable network");
                return Err(DauthError::ClientError("Not re-attempting connection before retry timeout".to_string()));
            } else {
                offline_cache.remove(address);
            }
        }
    }

    // Attempt a connection
    let endpoint = Endpoint::from_shared(format!("http://{}", address))
        .unwrap()
        .concurrency_limit(256)
        .timeout(Duration::from_millis(100))
        .connect_timeout(Duration::from_millis(50));
    let client = HomeNetworkClient::connect(endpoint).await;

    // If the connection fails, keep track and don't retry until the timeout expires.
    if client.is_err() {
        let mut offline_cache = context.rpc_context.known_offline_networks.lock().await;
        offline_cache.insert(address.to_string(), std::time::Instant::now() + context.rpc_context.failed_connection_retry_cooldown);
    }

    let client = client?;

    // Store a clone in the cache for future connections

    // By not holding the lock it is possible that another context has already
    // added a client, but it's okay to overwrite it and drop it in this case.
    let cache_client = client.clone();
    {
        let mut clients = context.rpc_context.home_clients.lock().await;
        clients.insert(address.to_string(), cache_client);
    }

    Ok(client)
}

pub async fn mark_endpoint_offline(
    context: &Arc<DauthContext>,
    address: &str,
) -> () {
    let retry_timeout = std::time::Instant::now() + context.rpc_context.failed_connection_retry_cooldown;
    let address_string = address.to_string();

    {
        let mut clients = context.rpc_context.home_clients.lock().await;
        let mut offline_cache = context.rpc_context.known_offline_networks.lock().await;
        clients.remove(address);
        offline_cache.insert(address_string, retry_timeout);
    }
}
