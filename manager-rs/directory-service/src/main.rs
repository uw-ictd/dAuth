mod data;
mod database;
mod manager;
mod rpc;

use std::{path::PathBuf, sync::Arc};
use structopt::StructOpt;

use data::{
    config::DirectoryConfig, context::DirectoryContext, error::DirectoryError, opt::DirectoryOpt,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let context = build_context(DirectoryOpt::from_args())
        .await
        .expect("Failed to generate context");

    rpc::server::start_server(context).await;
}

async fn build_context(
    directory_opt: DirectoryOpt,
) -> Result<Arc<DirectoryContext>, DirectoryError> {
    let config = build_config(directory_opt.config_path)?;

    Ok(Arc::new(data::context::DirectoryContext {
        host_address: config.host_address,
        database_pool: database::general::database_init(&config.database_path).await?,
    }))
}

fn build_config(yaml_path: PathBuf) -> Result<DirectoryConfig, DirectoryError> {
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
