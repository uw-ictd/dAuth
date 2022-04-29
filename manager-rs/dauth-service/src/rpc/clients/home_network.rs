use std::sync::Arc;

use auth_vector::types::{HresStar, Kseaf, ResStar};

use crate::data::context::DauthContext;
use crate::data::error::DauthError;
use crate::data::signing::{self, SignPayloadType};
use crate::data::vector::AuthVectorRes;
use crate::rpc::dauth::common::UserIdKind;
use crate::rpc::dauth::remote::home_network_client::HomeNetworkClient;
use crate::rpc::dauth::remote::{
    get_home_auth_vector_req::Payload as AVPayload, get_home_confirm_key_req::Payload as CKPayload,
};
use crate::rpc::dauth::remote::{GetHomeAuthVectorReq, GetHomeConfirmKeyReq};

/// Get an auth vector from a user's home network.
pub async fn get_auth_vector(
    context: Arc<DauthContext>,
    user_id: &str,
    address: &str,
) -> Result<AuthVectorRes, DauthError> {
    add_client(context.clone(), address).await?;

    let response = context
        .rpc_context
        .home_clients
        .lock()
        .await
        .get_mut(address)
        .ok_or_else(|| DauthError::ClientError("Failed to get client".to_string()))?
        .get_auth_vector(GetHomeAuthVectorReq {
            message: Some(signing::sign_message(
                context.clone(),
                SignPayloadType::GetHomeAuthVectorReq(AVPayload {
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
    add_client(context.clone(), address).await?;

    let response = context
        .rpc_context
        .home_clients
        .lock()
        .await
        .get_mut(address)
        .ok_or_else(|| DauthError::ClientError("Failed to get client".to_string()))?
        .get_confirm_key(GetHomeConfirmKeyReq {
            message: Some(signing::sign_message(
                context.clone(),
                SignPayloadType::GetHomeConfirmKeyReq(CKPayload {
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

/// Adds a client to the current context if it doesn't already exist.
/// Otherwise, does nothing.
async fn add_client(context: Arc<DauthContext>, address: &str) -> Result<(), DauthError> {
    let mut clients = context.rpc_context.home_clients.lock().await;

    if !clients.contains_key(address) {
        clients.insert(
            address.to_string(),
            HomeNetworkClient::connect(format!("http://{}", address)).await?,
        );
    }

    Ok(())
}
