use prost::Message;

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

pub fn sign_message(payload: SignPayloadType) -> remote::SignedMessage {
    let (bytes, signed_type) = match payload {
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
    // TODO (nickfh7) Add signing logic, i.e. via dalek
}
