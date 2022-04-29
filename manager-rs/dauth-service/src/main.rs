mod data;
mod database;
mod manager;
mod rpc;

use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};

use ed25519_dalek::Keypair;
use rand::rngs::OsRng;
use rpc::dauth::directory::directory_client::DirectoryClient;
use serde_yaml;
use structopt::StructOpt;
use tokio::runtime::Handle;
use tracing::Level;
use tracing_subscriber;

use crate::data::{
    config::DauthConfig,
    context::{DauthContext, LocalContext, RemoteContext, RpcContext},
    error::DauthError,
    opt::DauthOpt,
};
use crate::rpc::server;

#[tokio::main]
async fn main() {
    // TODO(nickfh7) Add configuring for logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let dauth_opt = DauthOpt::from_args();
    let context = build_context(dauth_opt)
        .await
        .expect("Failed to generate context");

    server::start_server(context.clone()).await;
}

async fn build_context(dauth_opt: DauthOpt) -> Result<Arc<DauthContext>, DauthError> {
    let config = build_config(dauth_opt.config_path)?;

    let keys = generate_keys(&config.ed25519_keyfile_path);
    let pool = database::general::database_init(&config.database_path).await?;

    let context = Arc::new(DauthContext {
        local_context: LocalContext {
            id: config.id,
            database_pool: pool,
            local_user_id_min: config.local_user_id_min,
            local_user_id_max: config.local_user_id_max,
            signing_keys: keys,
        },
        remote_context: RemoteContext {
            backup_networks: config.backup_networks,
        },
        rpc_context: RpcContext {
            runtime_handle: Handle::current(),
            host_addr: config.host_addr,
            backup_clients: tokio::sync::Mutex::new(HashMap::new()),
            home_clients: tokio::sync::Mutex::new(HashMap::new()),
            directory_client: tokio::sync::Mutex::new(
                DirectoryClient::connect(format!("http://{}", config.directory_addr)).await?,
            ),
        },
    });

    for (user_id, user_info_config) in config.users {
        let user_info = user_info_config.to_user_info()?;
        tracing::info!("inserting user info: {:?} - {:?}", user_id, user_info);

        let mut transaction = context.local_context.database_pool.begin().await?;
        database::user_infos::upsert(
            &mut transaction,
            &user_id,
            &user_info.k,
            &user_info.opc,
            &user_info.sqn_max,
        )
        .await?;
        transaction.commit().await?;
    }

    Ok(context)
}

fn generate_keys(keyfile_path: &String) -> Keypair {
    match fs::read(keyfile_path) {
        Ok(keypair_bytes) => match Keypair::from_bytes(&keypair_bytes) {
            Ok(keypair) => {
                tracing::info!("Existing keyfile found");
                keypair
            }
            Err(e) => panic!("Failed to parse existing key bytes in keyfile -- {}", e),
        },
        Err(e) => {
            tracing::warn!("Failed to read content from '{}' -- {}", keyfile_path, e);
            build_keyfile(keyfile_path)
        }
    }
}

fn build_keyfile(keyfile_path: &String) -> Keypair {
    tracing::info!("generating new keyfile at '{}'", keyfile_path);
    let mut csprng = OsRng {};
    let keypair = Keypair::generate(&mut csprng);

    let path = std::path::Path::new(keyfile_path);
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();
    fs::write(path, keypair.to_bytes()).unwrap();
    keypair
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
