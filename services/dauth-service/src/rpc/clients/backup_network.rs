use std::sync::Arc;

use auth_vector::types::{XResStarHash, ResStar,Res,XResHash};
use tonic::transport::Channel;

use crate::data::context::DauthContext;
use crate::data::error::DauthError;
use crate::data::keys;
use crate::data::signing;
use crate::data::signing::SignPayloadType;
use crate::data::vector::AuthVectorRes;
use crate::database::tasks::replace_key_shares::ReplaceKeyShareTask;
use crate::rpc::dauth::common::UserIdKind;
use crate::rpc::dauth::remote::backup_network_client::BackupNetworkClient;
use crate::rpc::dauth::remote::{
    enroll_backup_prepare_req, flood_vector_req, get_backup_auth_vector_req, get_key_share_req,
    withdraw_backup_req, withdraw_shares_req, ReplaceShareReq,
};
use crate::rpc::dauth::remote::{
    EnrollBackupCommitReq, EnrollBackupPrepareReq, FloodVectorReq, GetBackupAuthVectorReq,
    GetKeyShareReq, WithdrawBackupReq, WithdrawSharesReq,
};
use crate::rpc::utilities;

/// Request a network to become a backup network.
pub async fn enroll_backup_prepare(
    context: Arc<DauthContext>,
    user_id: &str,
    backup_network_id: &str,
    address: &str,
) -> Result<(), DauthError> {
    let mut client = get_client(context.clone(), address).await?;

    let sent_payload = enroll_backup_prepare_req::Payload {
        home_network_id: context.local_context.id.clone(),
        backup_network_id: backup_network_id.to_string(),
        user_id_kind: UserIdKind::Supi as i32,
        user_id: user_id.as_bytes().to_vec(),
    };

    let response = client
        .enroll_backup_prepare(EnrollBackupPrepareReq {
            message: Some(signing::sign_message(
                context.clone(),
                SignPayloadType::EnrollBackupPrepareReq(sent_payload.clone()),
            )),
        })
        .await?
        .into_inner();

    let message = response.message.ok_or(DauthError::ClientError(
        "Missing signed message".to_string(),
    ))?;

    if let SignPayloadType::EnrollBackupPrepareReq(payload) =
        signing::verify_message(context.clone(), &message).await?
    {
        if payload == sent_payload {
            Ok(())
        } else {
            Err(DauthError::ClientError(format!(
                "Failed to verify enroll backup contents: {:?}/s {:?}/r",
                sent_payload, payload
            )))
        }
    } else {
        Err(DauthError::ClientError(format!(
            "Incorrect message type received: {:?}",
            message
        )))
    }
}

/// Send the set of initial vectors and key shares after
/// a network has agreed to be a backup.
pub async fn enroll_backup_commit(
    context: Arc<DauthContext>,
    backup_network_id: &str,
    user_id: &str,
    vectors: &Vec<AuthVectorRes>,
    key_shares: &Vec<keys::CombinedKeyShare>,
    address: &str,
) -> Result<(), DauthError> {
    let mut client = get_client(context.clone(), address).await?;

    let mut dvectors = Vec::new();
    let mut dshares = Vec::new();

    for vector in vectors {
        dvectors.push(utilities::build_delegated_vector(
            context.clone(),
            vector,
            backup_network_id,
        ))
    }
    for share in key_shares {
        dshares.push(utilities::build_delegated_share(
            context.clone(),
            &share,
        ))
    }

    client
        .enroll_backup_commit(EnrollBackupCommitReq {
            vectors: dvectors,
            shares: dshares,
            user_id_kind: UserIdKind::Supi as i32,
            user_id: user_id.as_bytes().to_vec(),
        })
        .await?;

    Ok(())
}

/// Get an auth vector from one of a user's backup networks.
pub async fn get_auth_vector(
    context: Arc<DauthContext>,
    user_id: &str,
    address: &str,
) -> Result<AuthVectorRes, DauthError> {
    let mut client = get_client(context.clone(), address).await?;

    let response = client
        .get_auth_vector(GetBackupAuthVectorReq {
            message: Some(signing::sign_message(
                context.clone(),
                SignPayloadType::GetBackupAuthVectorReq(get_backup_auth_vector_req::Payload {
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

    if let SignPayloadType::DelegatedAuthVector5G(payload) =
        signing::verify_message(context.clone(), &message).await?
    {
        let vector = payload.v.ok_or(DauthError::ClientError(
            "Missing vector content".to_string(),
        ))?;
        Ok(AuthVectorRes {
            user_id: user_id.to_string(),
            seqnum: vector.seqnum,
            xres_star_hash: vector.xres_star_hash[..].try_into()?,
            autn: vector.autn[..].try_into()?,
            rand: vector.rand[..].try_into()?,
            xres_hash: vector.xres_hash[..].try_into()?,
        })
    } else {
        Err(DauthError::ClientError(format!(
            "Incorrect message type received: {:?}",
            message
        )))
    }
}

/// Get a key share from one of a user's backup networks.
pub async fn get_kseaf_key_share(
    context: Arc<DauthContext>,
    xres_star_hash: XResStarHash,
    res_star: ResStar,
    address: String,
) -> Result<keys::KseafShare, DauthError> {
    let mut client = get_client(context.clone(), &address).await?;

    let response = client
        .get_key_share(GetKeyShareReq {
            message: Some(signing::sign_message(
                context.clone(),
                SignPayloadType::GetKeyShareReq(get_key_share_req::Payload {
                    serving_network_id: context.local_context.id.clone(),
                    preimage: Some(get_key_share_req::payload::Preimage::ResStar(res_star.to_vec())),
                    hash: Some(get_key_share_req::payload::Hash::XresStarHash(xres_star_hash.to_vec())),
                }),
            )),
        })
        .await?
        .into_inner();

    let message = response
        .share
        .ok_or_else(|| DauthError::ClientError("Missing delegated key share".to_string()))?
        .message
        .ok_or_else(|| DauthError::ClientError("Missing delegated key share".to_string()))?;

    if let SignPayloadType::DelegatedConfirmationShare(payload) =
        signing::verify_message(context.clone(), &message).await?
    {
        Ok(payload.kseaf_confirmation_share[..].try_into()?)
    } else {
        Err(DauthError::ClientError(format!(
            "Incorrect message type received: {:?}",
            message
        )))
    }
}

/// Get a kasme key share from one of a user's backup networks.
pub async fn get_kasme_key_share(
    context: Arc<DauthContext>,
    xres_hash: XResHash,
    res: Res,
    address: String,
) -> Result<keys::KasmeShare, DauthError> {
    let mut client = get_client(context.clone(), &address).await?;

    let response = client
        .get_key_share(GetKeyShareReq {
            message: Some(signing::sign_message(
                context.clone(),
                SignPayloadType::GetKeyShareReq(get_key_share_req::Payload {
                    serving_network_id: context.local_context.id.clone(),
                    preimage: Some(get_key_share_req::payload::Preimage::Res(res.to_vec())),
                    hash: Some(get_key_share_req::payload::Hash::XresHash(xres_hash.to_vec())),
                }),
            )),
        })
        .await?
        .into_inner();

    let message = response
        .share
        .ok_or_else(|| DauthError::ClientError("Missing delegated key share".to_string()))?
        .message
        .ok_or_else(|| DauthError::ClientError("Missing delegated key share".to_string()))?;

    if let SignPayloadType::DelegatedConfirmationShare(payload) =
        signing::verify_message(context.clone(), &message).await?
    {
        Ok(payload.kasme_confirmation_share[..].try_into()?)
    } else {
        Err(DauthError::ClientError(format!(
            "Incorrect message type received: {:?}",
            message
        )))
    }
}

/// Requests for a key share be removed and for a new key share
/// to be stored.
pub async fn replace_key_share(
    context: Arc<DauthContext>,
    replace: &ReplaceKeyShareTask,
    address: &str,
) -> Result<(), DauthError> {
    let mut client = get_client(context.clone(), address).await?;

    client
        .replace_key_share(ReplaceShareReq {
            new_share: Some(utilities::build_delegated_share(
                context,
                &replace.key_share,
            )),
            replaced_share_xres_star_hash: replace.old_xres_star_hash.clone(),
        })
        .await?;

    Ok(())
}

/// Withdraws backup status from a backup network.
pub async fn withdraw_backup(
    context: Arc<DauthContext>,
    user_id: &str,
    backup_network_id: &str,
    address: &str,
) -> Result<(), DauthError> {
    let mut client = get_client(context.clone(), address).await?;

    client
        .withdraw_backup(WithdrawBackupReq {
            message: Some(signing::sign_message(
                context.clone(),
                SignPayloadType::WithdrawBackupReq(withdraw_backup_req::Payload {
                    home_network_id: context.local_context.id.clone(),
                    backup_network_id: backup_network_id.to_string(),
                    user_id_kind: UserIdKind::Supi as i32,
                    user_id: user_id.as_bytes().to_vec(),
                }),
            )),
        })
        .await?;

    Ok(())
}

/// Withdraws all matching shares from a backup network.
pub async fn withdraw_shares(
    context: Arc<DauthContext>,
    xres_star_hashs: Vec<XResStarHash>,
    address: &str,
) -> Result<(), DauthError> {
    let mut client = get_client(context.clone(), address).await?;

    let mut proc_xrhs = Vec::new();

    for xrhs_slice in xres_star_hashs {
        proc_xrhs.push(xrhs_slice.to_vec());
    }

    client
        .withdraw_shares(WithdrawSharesReq {
            message: Some(signing::sign_message(
                context.clone(),
                SignPayloadType::WithdrawSharesReq(withdraw_shares_req::Payload {
                    home_network_id: context.local_context.id.clone(),
                    xres_star_hash: proc_xrhs,
                }),
            )),
        })
        .await?;

    Ok(())
}

/// Sends a flood vector to a backup network.
pub async fn flood_vector(
    context: Arc<DauthContext>,
    backup_network_id: &str,
    user_id: &str,
    vector: &AuthVectorRes,
    address: &str,
) -> Result<(), DauthError> {
    let mut client = get_client(context.clone(), address).await?;

    client
        .flood_vector(FloodVectorReq {
            message: Some(signing::sign_message(
                context.clone(),
                SignPayloadType::FloodVectorReq(flood_vector_req::Payload {
                    home_network_id: context.local_context.id.clone(),
                    user_id_kind: UserIdKind::Supi as i32,
                    user_id: user_id.as_bytes().to_vec(),
                    vector: Some(utilities::build_delegated_vector(
                        context.clone(),
                        vector,
                        backup_network_id,
                    )),
                }),
            )),
        })
        .await?;

    Ok(())
}

/// Returns a client to the service at the provided address.
/// Builds and caches a client if one does not exist.
async fn get_client(
    context: Arc<DauthContext>,
    address: &str,
) -> Result<BackupNetworkClient<Channel>, DauthError> {
    let mut clients = context.rpc_context.backup_clients.lock().await;

    if !clients.contains_key(address) {
        clients.insert(
            address.to_string(),
            BackupNetworkClient::connect(format!("http://{}", address)).await?,
        );
    }

    Ok(clients
        .get(address)
        .ok_or(DauthError::ClientError("Client not found".to_string()))?
        .clone())
}
