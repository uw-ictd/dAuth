use auth_vector::types::{Autn, XResStarHash, Id, Rand, XResHash};

use crate::data::error::DauthError;
use crate::rpc::dauth::common::AuthVector5G;
use crate::rpc::dauth::local::AkaVectorResp;

#[derive(Debug)]
pub struct AuthVectorReq {
    pub user_id: Id,
}

#[derive(Debug)]
pub struct AuthVectorRes {
    pub user_id: Id,
    pub seqnum: i64,
    pub xres_star_hash: XResStarHash,
    pub xres_hash: XResHash,
    pub autn: Autn,
    pub rand: Rand,
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
                seqnum: self.seqnum,
                xres_hash: self.xres_hash.to_vec(),
            }),
        }
    }

    pub fn from_av5_g(user_id: &str, vector: AuthVector5G) -> Result<AuthVectorRes, DauthError> {
        Ok(AuthVectorRes {
            user_id: user_id.to_string(),
            seqnum: vector.seqnum,
            xres_star_hash: vector.xres_star_hash[..].try_into()?,
            autn: vector.autn[..].try_into()?,
            rand: vector.rand[..].try_into()?,
            xres_hash: vector.xres_hash[..].try_into()?,
        })
    }
}
