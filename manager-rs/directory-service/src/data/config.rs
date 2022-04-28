use serde::{Deserialize, Serialize};

/// Holds all configuration data from a corresponding YAML file
#[derive(Serialize, Deserialize, Debug)]
pub struct DirectoryConfig {
    pub host_address: String,
    pub database_path: String,
}
