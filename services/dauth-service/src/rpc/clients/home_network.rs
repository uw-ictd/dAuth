use std::sync::Arc;
use std::time::Duration;

use auth_vector::types::{HresStar, Kseaf, ResStar};
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
    get_home_auth_vector_req, get_home_confirm_key_req, ReportHomeAuthConsumedReq,
    ReportHomeKeyShareConsumedReq, SignedMessage,
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

    let response = client.get_auth_vector(request).await?.into_inner();

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
        signing::verify_message(context.clone(), &message).await?
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
pub async fn get_confirm_key(
    context: Arc<DauthContext>,
    res_star: &ResStar,
    xres_star_hash: &HresStar,
    address: &str,
) -> Result<Kseaf, DauthError> {
    let mut client = get_client(context.clone(), address).await?;

    let response = client
        .get_confirm_key(GetHomeConfirmKeyReq {
            message: Some(signing::sign_message(
                context.clone(),
                SignPayloadType::GetHomeConfirmKeyReq(get_home_confirm_key_req::Payload {
                    serving_network_id: context.local_context.id.clone(),
                    res_star: res_star.to_vec(),
                    hash_xres_star: xres_star_hash.to_vec(),
                }),
            )),
        })
        .await?
        .into_inner();

    Ok(response.kseaf[..].try_into()?)
}

/// Reports an auth vector as used to the home network.
/// Sends the original signed request for the auth vector as
/// proof of the auth vector request.
pub async fn report_auth_consumed(
    context: Arc<DauthContext>,
    xres_star_hash: &HresStar,
    user_id: &str,
    original_request: &Vec<u8>,
    address: &str,
) -> Result<Option<AuthVectorRes>, DauthError> {
    let mut client = get_client(context.clone(), address).await?;

    let signed_message = SignedMessage::decode(&original_request[..])?;

    let dvector = client
        .report_auth_consumed(ReportHomeAuthConsumedReq {
            backup_network_id: context.local_context.id.clone(),
            hash_xres_star: xres_star_hash.to_vec(),
            backup_auth_vector_req: Some(signed_message),
        })
        .await?
        .into_inner()
        .vector;

    match dvector {
        None => Ok(None),
        Some(dvector) => {
            Ok(Some(utilities::handle_delegated_vector(context, dvector, user_id).await?))
        }
    }
}

/// Reports a key share as used to the home network.
/// Sends the original signed request for the auth vector as
/// proof of the auth vector request.
pub async fn report_key_share_consumed(
    context: Arc<DauthContext>,
    original_request: &Vec<u8>,
    address: &str,
) -> Result<(), DauthError> {
    let mut client = get_client(context.clone(), address).await?;

    let signed_message = SignedMessage::decode(&original_request[..])?;

    // no key share is sent in return yet
    let _res = client
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
async fn get_client(
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

    // TODO(matt9j) Keep track of if we've attempted and failed to open a
    // connection before and don't retry for every request.
    let endpoint = Endpoint::from_shared(format!("http://{}", address))
        .unwrap()
        .concurrency_limit(256)
        .timeout(Duration::from_millis(100))
        .connect_timeout(Duration::from_millis(50));
    let client = HomeNetworkClient::connect(endpoint).await?;

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
