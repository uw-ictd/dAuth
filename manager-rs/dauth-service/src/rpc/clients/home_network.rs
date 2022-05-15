use std::sync::Arc;

use auth_vector::types::{HresStar, Kseaf, ResStar};
use prost::Message;
use tonic::transport::Channel;

use crate::data::context::DauthContext;
use crate::data::error::DauthError;
use crate::data::signing::{self, SignPayloadType};
use crate::data::vector::AuthVectorRes;
use crate::rpc::dauth::common::UserIdKind;
use crate::rpc::dauth::remote::home_network_client::HomeNetworkClient;
use crate::rpc::dauth::remote::{
    get_home_auth_vector_req, get_home_confirm_key_req, ReportHomeKeyShareConsumedReq,
    SignedMessage,
};
use crate::rpc::dauth::remote::{GetHomeAuthVectorReq, GetHomeConfirmKeyReq};

/// Get an auth vector from a user's home network.
pub async fn get_auth_vector(
    context: Arc<DauthContext>,
    user_id: &str,
    address: &str,
) -> Result<AuthVectorRes, DauthError> {
    let mut client = get_client(context.clone(), address).await?;

    let response = client
        .get_auth_vector(GetHomeAuthVectorReq {
            message: Some(signing::sign_message(
                context.clone(),
                SignPayloadType::GetHomeAuthVectorReq(get_home_auth_vector_req::Payload {
                    serving_network_id: context.local_context.id.clone(),
                    user_id_type: UserIdKind::Supi as i32,
                    user_id: user_id.as_bytes().to_vec(),
                }),
            )),
        })
        .await?
        .into_inner();

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
    original_request: Vec<u8>,
) -> Result<AuthVectorRes, DauthError> {
    // TODO: Delete auths during report, add to task to report to home network.
    todo!()
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

    client
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
    let mut clients = context.rpc_context.home_clients.lock().await;

    if !clients.contains_key(address) {
        clients.insert(
            address.to_string(),
            HomeNetworkClient::connect(format!("http://{}", address)).await?,
        );
    }

    Ok(clients
        .get(address)
        .ok_or(DauthError::ClientError("Client not found".to_string()))?
        .clone())
}
