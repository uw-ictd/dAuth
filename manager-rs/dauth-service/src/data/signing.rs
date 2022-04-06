use std::sync::Arc;

use ed25519_dalek::{Signature, Signer, Verifier};
use prost::Message;

use crate::data::context::DauthContext;
use crate::data::error::DauthError;
use crate::rpc::dauth::remote;

/// All payload types that expect to be signed
#[derive(Debug)]
pub enum SignPayloadType {
    DelegatedAuthVector5G(remote::delegated_auth_vector5_g::Payload),
    GetHomeAuthVectorReq(remote::get_home_auth_vector_req::Payload),
    GetHomeConfirmKeyReq(remote::get_home_confirm_key_req::Payload),
    EnrollBackupPrepareReq(remote::enroll_backup_prepare_req::Payload),
    GetBackupAuthVectorReq(remote::get_backup_auth_vector_req::Payload),
    GetKeyShareReq(remote::get_key_share_req::Payload),
    WithdrawBackupReq(remote::withdraw_backup_req::Payload),
    WithdrawSharesReq(remote::withdraw_shares_req::Payload),
    FloodVectorReq(remote::flood_vector_req::Payload),
}

pub fn sign_message(context: Arc<DauthContext>, payload: SignPayloadType) -> remote::SignedMessage {
    let (payload_bytes, payload_kind) = match payload {
        SignPayloadType::DelegatedAuthVector5G(payload_message) => (
            payload_message.encode_to_vec(),
            remote::SignedMessageKind::DelegatedAuthVector5G,
        ),
        SignPayloadType::GetHomeAuthVectorReq(payload_message) => (
            payload_message.encode_to_vec(),
            remote::SignedMessageKind::GetHomeAuthVectorReq,
        ),
        SignPayloadType::GetHomeConfirmKeyReq(payload_message) => (
            payload_message.encode_to_vec(),
            remote::SignedMessageKind::GetHomeConfirmKeyReq,
        ),
        SignPayloadType::EnrollBackupPrepareReq(payload_message) => (
            payload_message.encode_to_vec(),
            remote::SignedMessageKind::EnrollBackupPrepareReq,
        ),
        SignPayloadType::GetBackupAuthVectorReq(payload_message) => (
            payload_message.encode_to_vec(),
            remote::SignedMessageKind::GetBackupAuthVectorReq,
        ),
        SignPayloadType::GetKeyShareReq(payload_message) => (
            payload_message.encode_to_vec(),
            remote::SignedMessageKind::GetKeyShareReq,
        ),
        SignPayloadType::WithdrawBackupReq(payload_message) => (
            payload_message.encode_to_vec(),
            remote::SignedMessageKind::WithdrawBackupReq,
        ),
        SignPayloadType::WithdrawSharesReq(payload_message) => (
            payload_message.encode_to_vec(),
            remote::SignedMessageKind::WithdrawSharesReq,
        ),
        SignPayloadType::FloodVectorReq(payload_message) => (
            payload_message.encode_to_vec(),
            remote::SignedMessageKind::FloodVectorReq,
        ),
    };

    let container = remote::signed_message::Container {
        kind: payload_kind as i32,
        payload: payload_bytes,
    }
    .encode_to_vec();

    let signature = Vec::from(
        context
            .local_context
            .signing_keys
            .sign(&container)
            .to_bytes(),
    );

    remote::SignedMessage {
        container,
        signature,
        signer_id: context.local_context.id.clone(),
    }
}

fn verify_message_with_id(
    context: Arc<DauthContext>,
    message: &remote::SignedMessage,
    signer_id: &str,
) -> Result<(), DauthError> {
    let public_key = context
        .remote_context
        .remote_keys
        .get(signer_id)
        .ok_or_else(|| DauthError::NotFoundError(format!("No key for signer id: {}", signer_id)))?;

    public_key.verify(
        &message.container,
        &Signature::from_bytes(&message.signature)?,
    )?;

    Ok(())
}

pub fn verify_message(
    context: Arc<DauthContext>,
    message: &remote::SignedMessage,
) -> Result<SignPayloadType, DauthError> {
    let container = remote::signed_message::Container::decode(message.container.as_slice())?;

    let payload = match container.kind() {
        remote::SignedMessageKind::DelegatedAuthVector5G => {
            let payload =
                remote::delegated_auth_vector5_g::Payload::decode(container.payload.as_slice())?;
            verify_message_with_id(context.clone(), message, &message.signer_id)?;
            Ok(SignPayloadType::DelegatedAuthVector5G(payload))
        }
        remote::SignedMessageKind::GetHomeAuthVectorReq => {
            let payload =
                remote::get_home_auth_vector_req::Payload::decode(container.payload.as_slice())?;
            verify_message_with_id(context.clone(), message, &message.signer_id)?;
            Ok(SignPayloadType::GetHomeAuthVectorReq(payload))
        }
        remote::SignedMessageKind::GetHomeConfirmKeyReq => {
            let payload =
                remote::get_home_confirm_key_req::Payload::decode(container.payload.as_slice())?;
            verify_message_with_id(context.clone(), message, &message.signer_id)?;
            Ok(SignPayloadType::GetHomeConfirmKeyReq(payload))
        }
        remote::SignedMessageKind::EnrollBackupPrepareReq => {
            let payload =
                remote::enroll_backup_prepare_req::Payload::decode(container.payload.as_slice())?;
            // TODO (nickfh7): Figure out how to get id from context
            verify_message_with_id(context.clone(), message, &message.signer_id)?;
            Ok(SignPayloadType::EnrollBackupPrepareReq(payload))
        }
        remote::SignedMessageKind::GetBackupAuthVectorReq => {
            let payload =
                remote::get_backup_auth_vector_req::Payload::decode(container.payload.as_slice())?;
            verify_message_with_id(context.clone(), message, &message.signer_id)?;
            Ok(SignPayloadType::GetBackupAuthVectorReq(payload))
        }
        remote::SignedMessageKind::GetKeyShareReq => {
            let payload = remote::get_key_share_req::Payload::decode(container.payload.as_slice())?;
            verify_message_with_id(context.clone(), message, &message.signer_id)?;
            Ok(SignPayloadType::GetKeyShareReq(payload))
        }
        remote::SignedMessageKind::WithdrawBackupReq => {
            let payload =
                remote::withdraw_backup_req::Payload::decode(container.payload.as_slice())?;
            // TODO (nickfh7): Figure out how to get id from context
            verify_message_with_id(context.clone(), message, &message.signer_id)?;
            Ok(SignPayloadType::WithdrawBackupReq(payload))
        }
        remote::SignedMessageKind::WithdrawSharesReq => {
            let payload =
                remote::withdraw_shares_req::Payload::decode(container.payload.as_slice())?;
            verify_message_with_id(context.clone(), message, &message.signer_id)?;
            Ok(SignPayloadType::WithdrawSharesReq(payload))
        }
        remote::SignedMessageKind::FloodVectorReq => {
            let payload = remote::flood_vector_req::Payload::decode(container.payload.as_slice())?;
            verify_message_with_id(context.clone(), message, &message.signer_id)?;
            Ok(SignPayloadType::FloodVectorReq(payload))
        }
        _ => Err(DauthError::InvalidMessageError(format!(
            "Unsupported type: {:?}",
            container.kind()
        ))),
    }?;

    Ok(payload)
}
