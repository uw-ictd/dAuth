use std::{path::PathBuf, sync::Arc};

use crate::data::{
    self, config::DirectoryConfig, context::DirectoryContext, error::DirectoryError,
};
use crate::database;

pub async fn build_context(
    config: DirectoryConfig,
) -> Result<Arc<DirectoryContext>, DirectoryError> {
    Ok(Arc::new(data::context::DirectoryContext {
        host_address: config.host_address,
        database_pool: database::general::database_init(&config.database_path).await?,
    }))
}

pub fn build_config(yaml_path: PathBuf) -> Result<DirectoryConfig, DirectoryError> {
    match std::fs::read_to_string(yaml_path) {
        Ok(yaml_string) => match serde_yaml::from_str(&yaml_string) {
            Ok(config) => Ok(config),
            Err(e) => Err(DirectoryError::ConfigError(format!(
                "Config contents invalid: {}",
                e
            ))),
        },
        Err(e) => Err(DirectoryError::ConfigError(format!(
            "Failed to open config file: {}",
            e
        ))),
    }
}
