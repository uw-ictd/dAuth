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
        Ok(AuthVectorReq {
            user_id: req.user_id[..].try_into()?,
        })
    }
}

impl AuthVectorRes {
    pub fn to_resp(&self) -> AkaVectorResp {
        AkaVectorResp {
            user_id: self.user_id.to_vec(),
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
