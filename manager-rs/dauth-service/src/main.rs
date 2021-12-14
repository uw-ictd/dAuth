mod data;
mod local;
mod rpc;

use data::user_info::UserInfo;
use serde_yaml;
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use structopt::StructOpt;
use tokio::runtime::Handle;
use tracing::Level;
use tracing_subscriber;

use auth_vector::{constants::ID_LENGTH, types::Id};

use crate::data::{
    config::DauthConfig,
    context::{DauthContext, LocalContext, RemoteContext, RpcContext},
    error::DauthError,
    opt::DauthOpt,
    utilities,
};
use crate::rpc::server;

#[tokio::main]
async fn main() {
    let dauth_opt = DauthOpt::from_args();
    let context = build_context(dauth_opt).expect("Failed to generate context");

    // TODO(nickfh7) Add configuring for logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    server::start_server(context.clone()).await;
}

fn build_context(dauth_opt: DauthOpt) -> Result<Arc<DauthContext>, DauthError> {
    let config = build_config(dauth_opt.config_path)?;
    let mut user_map: HashMap<Id, UserInfo> = HashMap::new();

    for (id, user_info_config) in config.users {
        tracing::info!("inserting ID {:?}", id);
        user_map.insert(
            id.clone(),
            user_info_config.to_user_info()?,
        );
    }

    Ok(Arc::new(DauthContext {
        local_context: LocalContext {
            database: Mutex::new(HashMap::new()),
            kseaf_map: Mutex::new(HashMap::new()),
            user_info_database: Mutex::new(user_map),
            local_user_id_min: config.local_user_id_min,
            local_user_id_max: config.local_user_id_max,
        },
        remote_context: RemoteContext {
            remote_addrs: config.remote_addrs,
        },
        rpc_context: RpcContext {
            runtime_handle: Handle::current(),
            host_addr: config.host_addr,
            client_stubs: tokio::sync::Mutex::new(HashMap::new()),
        },
    }))
}

fn build_config(yaml_path: PathBuf) -> Result<DauthConfig, DauthError> {
    match std::fs::read_to_string(yaml_path) {
        Ok(yaml_string) => match serde_yaml::from_str(&yaml_string) {
            Ok(config) => Ok(config),
            Err(e) => Err(DauthError::ConfigError(format!(
                "Config contents invalid: {}",
                e
            ))),
        },
        Err(e) => Err(DauthError::ConfigError(format!(
            "Failed to open config file: {}",
            e
        ))),
    }
}
