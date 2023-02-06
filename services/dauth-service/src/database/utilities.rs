use auth_vector::types::{Kasme, Kseaf};
use sqlx::sqlite::SqliteRow;
use sqlx::Row;

use crate::data::error::DauthError;
use crate::data::keys;
use crate::data::user_info::UserInfo;
use crate::data::vector::AuthVectorRes;

pub trait DauthDataUtilities {
    fn to_auth_vector(&self) -> Result<AuthVectorRes, DauthError>;
    fn to_kseaf(&self) -> Result<Kseaf, DauthError>;
    fn to_kasme(&self) -> Result<Kasme, DauthError>;
    fn to_key_share(&self) -> Result<keys::KseafShare, DauthError>;
    fn to_user_info(&self) -> Result<UserInfo, DauthError>;
    fn to_backup_user_home_network_id(&self) -> Result<String, DauthError>;
}

/// Add functionality to the sqlite row
impl DauthDataUtilities for SqliteRow {
    fn to_auth_vector(&self) -> Result<AuthVectorRes, DauthError> {
        Ok(AuthVectorRes {
            user_id: String::from(self.try_get::<&str, &str>("user_id")?),
            seqnum: self.try_get::<i64, &str>("seqnum")?,
            xres_star_hash: self.try_get::<&[u8], &str>("xres_star_hash")?.try_into()?,
            autn: self.try_get::<&[u8], &str>("autn")?.try_into()?,
            rand: self.try_get::<&[u8], &str>("rand")?.try_into()?,
            xres_hash: self.try_get::<&[u8], &str>("xres_hash")?.try_into()?,
        })
    }

    fn to_kseaf(&self) -> Result<Kseaf, DauthError> {
        Ok(self.try_get::<&[u8], &str>("kseaf_data")?.try_into()?)
    }

    fn to_kasme(&self) -> Result<Kasme, DauthError> {
        Ok(self.try_get::<&[u8], &str>("kasme_data")?.try_into()?)
    }

    fn to_key_share(&self) -> Result<keys::KseafShare, DauthError> {
        Ok(self.try_get::<&[u8], &str>("key_share")?.try_into()?)
    }

    fn to_user_info(&self) -> Result<UserInfo, DauthError> {
        Ok(UserInfo {
            id: self.try_get::<String, &str>("id")?,
            k: self.try_get::<&[u8], &str>("k")?.try_into()?,
            opc: self.try_get::<&[u8], &str>("opc")?.try_into()?,
            sqn: self.try_get::<i64, &str>("sqn_max")?,
        })
    }

    fn to_backup_user_home_network_id(&self) -> Result<String, DauthError> {
        Ok(self.try_get::<&str, &str>("home_network_id")?.to_string())
    }
}
