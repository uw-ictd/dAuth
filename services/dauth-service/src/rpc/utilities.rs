use std::sync::Arc;

use auth_vector::types::XResStarHash;

use crate::data::{
    context::DauthContext, error::DauthError, keys, signing, signing::SignPayloadType,
    vector::AuthVectorRes,
};
use crate::rpc::dauth::common::AuthVector5G;
use crate::rpc::dauth::remote::{
    delegated_auth_vector5_g, delegated_confirmation_share, DelegatedAuthVector5G,
    DelegatedConfirmationShare,
};

pub fn build_delegated_vector(
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
            xres_hash: vector.xres_hash.to_vec(),
        }),
    };

    DelegatedAuthVector5G {
        message: Some(signing::sign_message(
            context,
            SignPayloadType::DelegatedAuthVector5G(payload),
        )),
    }
}

pub fn build_delegated_share(
    context: Arc<DauthContext>,
    share: &keys::CombinedKeyShare,
) -> DelegatedConfirmationShare {
    let payload = delegated_confirmation_share::Payload {
        xres_star_hash: share.xres_star_hash.to_vec(),
        xres_hash: share.xres_hash.to_vec(),
        kasme_confirmation_share: share.kasme_share.to_vec(),
        kseaf_confirmation_share: share.kseaf_share.to_vec(),
    };

    DelegatedConfirmationShare {
        message: Some(signing::sign_message(
            context,
            SignPayloadType::DelegatedConfirmationShare(payload),
        )),
    }
}

pub async fn handle_delegated_vector(
    context: &Arc<DauthContext>,
    dvector: DelegatedAuthVector5G,
    user_id: &str,
) -> Result<AuthVectorRes, DauthError> {
    let verify_result = signing::verify_message(
        context.clone(),
        &dvector.message.ok_or(DauthError::InvalidMessageError(
            "Missing content".to_string(),
        ))?,
    )
    .await?;

    if let SignPayloadType::DelegatedAuthVector5G(payload) = verify_result {
        Ok(AuthVectorRes::from_av5_g(
            user_id,
            payload.v.ok_or(DauthError::InvalidMessageError(
                "Missing content".to_string(),
            ))?,
        )?)
    } else {
        Err(DauthError::InvalidMessageError(format!(
            "Incorrect message type: {:?}",
            verify_result
        )))
    }
}

pub async fn handle_key_share(
    context: Arc<DauthContext>,
    dshare: DelegatedConfirmationShare,
) -> Result<(keys::CombinedKeyShare), DauthError> {
    let verify_result = signing::verify_message(
        context,
        &dshare.message.ok_or(DauthError::InvalidMessageError(
            "Missing content".to_string(),
        ))?,
    )
    .await?;

    if let SignPayloadType::DelegatedConfirmationShare(payload) = verify_result {
        Ok(keys::CombinedKeyShare{
            xres_star_hash: payload.xres_star_hash.as_slice().try_into()?,
            xres_hash: payload.xres_hash.as_slice().try_into()?,
            kasme_share: payload.kasme_confirmation_share.as_slice().try_into()?,
            kseaf_share: payload.kseaf_confirmation_share.as_slice().try_into()?,
        })
    } else {
        Err(DauthError::InvalidMessageError(format!(
            "Incorrect message type: {:?}",
            verify_result
        )))
    }
}
