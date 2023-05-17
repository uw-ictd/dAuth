use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};

use ed25519_dalek::Keypair;
use rand::rngs::OsRng;
use serde_yaml;

use crate::database;
use crate::{
    data::{
        config::DauthConfig,
        context::{
            BackupContext, DauthContext, LocalContext, MetricsContext, RpcContext, TasksContext,
        },
        error::DauthError,
        keys,
    },
    management,
};

pub fn build_config_from_file(yaml_path: PathBuf) -> Result<DauthConfig, DauthError> {
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

pub async fn build_context(config: DauthConfig) -> Result<Arc<DauthContext>, DauthError> {
    let keys = generate_keys(&config.ed25519_keyfile_path);
    let pool = database::general::database_init(&config.database_path).await?;

    let context = Arc::new(DauthContext {
        local_context: LocalContext {
            id: config.id,
            database_pool: pool,
            signing_keys: keys,
            num_sqn_slices: config.num_sqn_slices,
            max_backup_vectors: config.max_backup_vectors,
            mcc: config.mcc,
            mnc: config.mnc,
        },
        backup_context: BackupContext {
            backup_key_threshold: config
                .backup_key_threshold
                .unwrap_or(keys::TEMPORARY_CONSTANT_THRESHOLD as i64)
                as u8,
            auth_states: tokio::sync::Mutex::new(HashMap::new()),
            directory_network_cache: tokio::sync::Mutex::new(HashMap::new()),
            directory_user_cache: tokio::sync::Mutex::new(HashMap::new()),
        },
        rpc_context: RpcContext {
            host_addr: config.host_addr,
            directory_addr: config.directory_addr,
            local_auth_addr: config
                .local_auth_addr
                .unwrap_or("127.0.0.1:50051".to_owned()),
            backup_clients: tokio::sync::Mutex::new(HashMap::new()),
            home_clients: tokio::sync::Mutex::new(HashMap::new()),
            directory_client: tokio::sync::Mutex::new(None),
            known_offline_networks: tokio::sync::Mutex::new(HashMap::new()),
            failed_connection_retry_cooldown: Duration::from_secs(30),
        },
        tasks_context: TasksContext {
            start_time: Instant::now(),
            startup_delay: Duration::from_secs_f64(config.task_startup_delay),
            interval: Duration::from_secs_f64(config.task_interval),
            is_registered: tokio::sync::Mutex::new(false),
            replace_key_share_delay: Duration::from_secs_f64(10.0),
            metrics_report_interval: Duration::from_secs_f64(10.0),
            metrics_last_report: tokio::sync::Mutex::new(Instant::now()),
        },
        metrics_context: MetricsContext {
            max_recorded_metrics: config.max_recorded_metrics.unwrap_or(100) as usize,
            metrics_map: tokio::sync::Mutex::new(HashMap::new()),
        },
    });

    for user_info in config.users {
        tracing::info!(?user_info, "Adding new user");
        management::add_user(context.clone(), &user_info).await?;
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
