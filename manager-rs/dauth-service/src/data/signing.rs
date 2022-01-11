use crate::rpc::dauth::remote;

/// All message types that expect an encrypted and signed payload
pub enum SignPayloadType {
    DelegatedConfirmationShare(<remote::DelegatedConfirmationShare as Trait >::Payload),
    GetHomeConfirmKeyReq(remote::GetHomeConfirmKeyReq::Payload),
    EnrollBackupPrepareReq(remote::EnrollBackupPrepareReq::Payload),
    EnrollBackupCommitReq(remote::EnrollBackupCommitReq::Payload),
    GetBackupAuthVectorReq(remote::GetBackupAuthVectorReq::Payload),
    GetKeyShareReq(remote::GetKeyShareReq::Payload),
    WithdrawBackupReq(remote::WithdrawBackupReq::Payload),
    WithdrawSharesReq(remote::WithdrawSharesReq::Payload),
    FloodVectorReq(remote::FloodVectorReq::Payload),
}
