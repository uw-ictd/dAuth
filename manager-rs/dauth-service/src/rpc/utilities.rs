use std::sync::Arc;

use auth_vector::types::{HresStar, Kseaf};

use crate::data::context::DauthContext;
use crate::data::signing;
use crate::data::signing::SignPayloadType;
use crate::data::vector::AuthVectorRes;
use crate::rpc::dauth::common::AuthVector5G;
use crate::rpc::dauth::remote::{
    DelegatedAuthVector5G, DelegatedConfirmationShare,
};
use crate::rpc::dauth::remote::{
    delegated_auth_vector5_g, delegated_confirmation_share,
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
