use std::sync::Arc;

use ed25519_dalek::{PublicKey, Signature, Signer, Verifier};
use prost::Message;

use crate::data::context::DauthContext;
use crate::data::error::DauthError;
use crate::rpc::dauth::remote;

/// All payload types that expect to be signed
pub enum SignPayloadType {
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
    }
}

pub fn verify_message(
    _context: Arc<DauthContext>,
    public_key: PublicKey,
    message: remote::SignedMessage,
) -> Result<SignPayloadType, DauthError> {
    public_key.verify(
        &message.container,
        &Signature::from_bytes(&message.signature)?,
    )?;
    let container = remote::signed_message::Container::decode(message.container.as_slice())?;

    match container.kind() {
        remote::SignedMessageKind::GetHomeConfirmKeyReq => {
            Ok(SignPayloadType::GetHomeConfirmKeyReq(
                remote::get_home_confirm_key_req::Payload::decode(container.payload.as_slice())?,
            ))
        }
        remote::SignedMessageKind::EnrollBackupPrepareReq => {
            Ok(SignPayloadType::EnrollBackupPrepareReq(
                remote::enroll_backup_prepare_req::Payload::decode(container.payload.as_slice())?,
            ))
        }
        remote::SignedMessageKind::GetBackupAuthVectorReq => {
            Ok(SignPayloadType::GetBackupAuthVectorReq(
                remote::get_backup_auth_vector_req::Payload::decode(container.payload.as_slice())?,
            ))
        }
        remote::SignedMessageKind::GetKeyShareReq => Ok(SignPayloadType::GetKeyShareReq(
            remote::get_key_share_req::Payload::decode(container.payload.as_slice())?,
        )),
        remote::SignedMessageKind::WithdrawBackupReq => Ok(SignPayloadType::WithdrawBackupReq(
            remote::withdraw_backup_req::Payload::decode(container.payload.as_slice())?,
        )),
        remote::SignedMessageKind::WithdrawSharesReq => Ok(SignPayloadType::WithdrawSharesReq(
            remote::withdraw_shares_req::Payload::decode(container.payload.as_slice())?,
        )),
        remote::SignedMessageKind::FloodVectorReq => Ok(SignPayloadType::FloodVectorReq(
            remote::flood_vector_req::Payload::decode(container.payload.as_slice())?,
        )),
        _ => Err(DauthError::InvalidMessageError(format!(
            "Unsupported type: {:?}",
            container.kind()
        ))),
    }
}
