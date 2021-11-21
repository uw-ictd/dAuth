use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::data::{conversion, error::DauthError, user_info::UserInfo};

/// Holds all configuration data from a corresponding YAML file
#[derive(Serialize, Deserialize, Debug)]
pub struct DauthConfig {
    pub users: HashMap<String, UserInfoConfig>,
    pub remote_addrs: Vec<String>,
    pub host_addr: String,
}

/// For ease of inputting content in the yaml file
/// Likely be removed in later stages
#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfoConfig {
    pub k: String,
    pub opc: String,
    pub sqn_max: String,
}

impl UserInfoConfig {
    /// Generates a user info object with byte arrays
    pub fn to_user_info(&self) -> Result<UserInfo, DauthError> {
        let k = conversion::convert_string_to_byte_vec(&self.k)?;
        let opc = conversion::convert_string_to_byte_vec(&self.opc)?;
        let sqn_max = conversion::convert_string_to_byte_vec(&self.sqn_max)?;
        Ok(UserInfo { k, opc, sqn_max })
    }
}
