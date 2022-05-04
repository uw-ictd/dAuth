use std::sync::Arc;

use auth_vector::types::{HresStar, Kseaf};
use tonic::transport::Channel;

use crate::data::context::DauthContext;
use crate::data::error::DauthError;
use crate::data::signing;
use crate::data::signing::SignPayloadType;
use crate::data::vector::AuthVectorRes;
use crate::rpc::dauth::common::{AuthVector5G, UserIdKind};
use crate::rpc::dauth::remote::backup_network_client::BackupNetworkClient;
use crate::rpc::dauth::remote::{
    delegated_auth_vector5_g, delegated_confirmation_share, enroll_backup_prepare_req,
    flood_vector_req, get_backup_auth_vector_req, get_key_share_req, withdraw_backup_req,
    withdraw_shares_req,
};
use crate::rpc::dauth::remote::{
    DelegatedAuthVector5G, DelegatedConfirmationShare, EnrollBackupCommitReq,
    EnrollBackupPrepareReq, FloodVectorReq, GetBackupAuthVectorReq, GetKeyShareReq,
    WithdrawBackupReq, WithdrawSharesReq,
};

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
    vectors: Vec<AuthVectorRes>,
    key_shares: Vec<(HresStar, Kseaf)>,
    address: &str,
) -> Result<(), DauthError> {
    let mut client = get_client(context.clone(), address).await?;

    let mut dvectors = Vec::new();
    let mut dshares = Vec::new();

    for vector in &vectors {
        dvectors.push(build_delegated_vector(
            context.clone(),
            vector,
            backup_network_id,
        ))
    }
    for (xres_star_hash, confirmation_share) in &key_shares {
        dshares.push(build_delegated_share(
            context.clone(),
            xres_star_hash,
            confirmation_share,
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
        })
    } else {
        Err(DauthError::ClientError(format!(
            "Incorrect message type received: {:?}",
            message
        )))
    }
}

/// Get a key share from one of a user's backup networks.
pub async fn get_key_share(
    context: Arc<DauthContext>,
    xres_star_hash: HresStar,
    res_star: Kseaf,
    address: &str,
) -> Result<Kseaf, DauthError> {
    let mut client = get_client(context.clone(), address).await?;

    let response = client
        .get_key_share(GetKeyShareReq {
            message: Some(signing::sign_message(
                context.clone(),
                SignPayloadType::GetKeyShareReq(get_key_share_req::Payload {
                    serving_network_id: context.local_context.id.clone(),
                    res_star: res_star.to_vec(),
                    hash_xres_star: xres_star_hash.to_vec(),
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
        Ok(payload.confirmation_share[..].try_into()?)
    } else {
        Err(DauthError::ClientError(format!(
            "Incorrect message type received: {:?}",
            message
        )))
    }
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
    xres_star_hashs: Vec<HresStar>,
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
                    vector: Some(build_delegated_vector(
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

fn build_delegated_vector(
    context: Arc<DauthContext>,
    vector: &AuthVectorRes,
    serving_network_id: &str,
) -> DelegatedAuthVector5G {
    let payload = delegated_auth_vector5_g::Payload {
        serving_network_id: serving_network_id.to_string(),
        v: Some(AuthVector5G {
            rand: vector.rand.to_vec(),
            xres_star_hash: vector.xres_star_hash.to_vec(),
            autn: vector.autn.to_vec(),
            seqnum: vector.seqnum,
        }),
    };

    DelegatedAuthVector5G {
        message: Some(signing::sign_message(
            context,
            SignPayloadType::DelegatedAuthVector5G(payload),
        )),
    }
}

fn build_delegated_share(
    context: Arc<DauthContext>,
    xres_star_hash: &HresStar,
    confirmation_share: &Kseaf,
) -> DelegatedConfirmationShare {
    let payload = delegated_confirmation_share::Payload {
        xres_star_hash: xres_star_hash.to_vec(),
        confirmation_share: confirmation_share.to_vec(),
    };

    DelegatedConfirmationShare {
        message: Some(signing::sign_message(
            context,
            SignPayloadType::DelegatedConfirmationShare(payload),
        )),
    }
}
