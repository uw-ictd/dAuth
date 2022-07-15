use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use auth_vector::{
    types::{Opc, K, K_LENGTH, OPC_LENGTH},
};

use crate::data::{error::DauthError, utilities};

/// Holds all configuration data from a corresponding YAML file
#[derive(Serialize, Deserialize, Debug)]
pub struct DauthConfig {
    pub id: String,
    pub users: HashMap<String, UserInfoConfig>,
    pub host_addr: String,
    pub directory_addr: String,
    pub ed25519_keyfile_path: String,
    pub database_path: String,
    pub num_sqn_slices: i64,
    pub max_backup_vectors: i64,
    pub task_startup_delay: f64,
    pub task_interval: f64,
    pub local_auth_addr: Option<String>,
    pub max_recorded_metrics: Option<i64>,
    pub backup_key_threshold: Option<i64>,
}

/// For ease of inputting content in the yaml file
/// Likely be removed in later stages
#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfoConfig {
    pub k: String,
    pub opc: String,
    pub sqn_slice_max: HashMap<i64, i64>,
    pub backup_network_ids: HashMap<String, i64>,
}

impl UserInfoConfig {
    /// Generates a user info object with byte arrays
    pub fn get_k(&self) -> Result<K, DauthError> {
        Ok(
            utilities::convert_hex_string_to_byte_vec_with_length(&self.k, K_LENGTH)?[..]
                .try_into()?,
        )
    }
    /// Generates a user info object with byte arrays
    pub fn get_opc(&self) -> Result<Opc, DauthError> {
        Ok(
            utilities::convert_hex_string_to_byte_vec_with_length(&self.opc, OPC_LENGTH)?[..]
                .try_into()?,
        )
    }
}
