use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use auth_vector::{
    constants::{K_LENGTH, OPC_LENGTH, SQN_LENGTH},
    types::{Opc, Sqn, K},
};

use crate::data::{error::DauthError, user_info::UserInfo, utilities};

/// Holds all configuration data from a corresponding YAML file
#[derive(Serialize, Deserialize, Debug)]
pub struct DauthConfig {
    pub users: HashMap<String, UserInfoConfig>,
    pub remote_addrs: Vec<String>,
    pub host_addr: String,
    pub local_user_id_min: String,
    pub local_user_id_max: String,
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
        let k: K = utilities::convert_hex_string_to_byte_vec_with_length(&self.k, K_LENGTH)?[..]
            .try_into()?;
        let opc: Opc =
            utilities::convert_hex_string_to_byte_vec_with_length(&self.opc, OPC_LENGTH)?[..]
                .try_into()?;
        let sqn_max: Sqn =
            utilities::convert_int_string_to_byte_vec_with_length(&self.sqn_max, SQN_LENGTH)?[..]
                .try_into()?;
        Ok(UserInfo { k, opc, sqn_max })
    }
}
