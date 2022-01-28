use auth_vector::types::{Autn, HresStar, Id, Rand};

use crate::data::error::DauthError;
use crate::rpc::dauth::common::AuthVector5G;
use crate::rpc::dauth::local::{AkaVectorReq, AkaVectorResp};

#[derive(Debug)]
pub struct AuthVectorReq {
    pub user_id: Id,
}

#[derive(Debug)]
pub struct AuthVectorRes {
    pub user_id: Id,
    pub xres_star_hash: HresStar,
    pub autn: Autn,
    pub rand: Rand,
}

impl AuthVectorReq {
    pub fn from_req(req: AkaVectorReq) -> Result<AuthVectorReq, DauthError> {
        tracing::info!("Converting id");
        match req.user_id_type() {
            crate::rpc::dauth::common::UserIdKind::Supi => {
                let id_string = std::str::from_utf8(req.user_id.as_slice())?;
                tracing::debug!("Converted id {:?}", id_string);
                Ok(AuthVectorReq {
                    user_id: id_string.to_string(),
                })
            }
            crate::rpc::dauth::common::UserIdKind::Unknown => Err(DauthError::InvalidMessageError(
                "user_id_kind is unknown".to_owned(),
            )),
        }
    }
}

impl AuthVectorRes {
    pub fn to_resp(&self) -> AkaVectorResp {
        AkaVectorResp {
            user_id: self.user_id.clone().into_bytes(),
            user_id_type: 1,
            error: 0,
            auth_vector: Some(AuthVector5G {
                rand: self.rand.to_vec(),
                xres_star_hash: self.xres_star_hash.to_vec(),
                autn: self.autn.to_vec(),
            }),
        }
    }
}
