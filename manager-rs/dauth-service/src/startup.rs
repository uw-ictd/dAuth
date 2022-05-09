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

use crate::data::{
    config::DauthConfig,
    context::{DauthContext, LocalContext, RpcContext, TasksContext},
    error::DauthError,
    opt::DauthOpt,
};
use crate::database;

pub async fn build_context(dauth_opt: DauthOpt) -> Result<Arc<DauthContext>, DauthError> {
    let config = build_config(dauth_opt.config_path)?;

    let keys = generate_keys(&config.ed25519_keyfile_path);
    let pool = database::general::database_init(&config.database_path).await?;

    let context = Arc::new(DauthContext {
        local_context: LocalContext {
            id: config.id,
            database_pool: pool,
            signing_keys: keys,
            num_sqn_slices: config.num_sqn_slices,
            max_backup_vectors: config.max_backup_vectors,
        },
        rpc_context: RpcContext {
            host_addr: config.host_addr,
            directory_addr: config.directory_addr,
            backup_clients: tokio::sync::Mutex::new(HashMap::new()),
            home_clients: tokio::sync::Mutex::new(HashMap::new()),
            directory_client: tokio::sync::Mutex::new(None),
        },
        tasks_context: TasksContext {
            start_time: Instant::now(),
            startup_delay: Duration::from_secs_f64(config.task_startup_delay),
            interval: Duration::from_secs_f64(config.task_interval),
            is_registered: tokio::sync::Mutex::new(false),
        },
    });

    for (user_id, user_info_config) in config.users {
        tracing::info!(
            "inserting user info: {:?} - {:?}",
            user_id,
            user_info_config
        );

        let mut transaction = context.local_context.database_pool.begin().await?;
        database::user_infos::upsert(
            &mut transaction,
            &user_id,
            &user_info_config.get_k()?,
            &user_info_config.get_opc()?,
            *user_info_config
                .sqn_slice_max
                .get(&0)
                .ok_or(DauthError::ConfigError("No home network slice".to_string()))?,
            0, // home network
        )
        .await?;

        if user_info_config.backup_network_ids.len() as i64 - 1
            > context.local_context.num_sqn_slices
        {
            return Err(DauthError::ConfigError(format!(
                "Not enough slices for all backup networks: {}",
                user_id
            )));
        }

        for (backup_network_id, sqn_slice) in &user_info_config.backup_network_ids {
            database::user_infos::upsert(
                &mut transaction,
                &user_id,
                &user_info_config.get_k()?,
                &user_info_config.get_opc()?,
                *user_info_config
                    .sqn_slice_max
                    .get(&0)
                    .ok_or(DauthError::ConfigError(format!(
                        "Missing key slice for {}",
                        sqn_slice
                    )))?,
                *sqn_slice,
            )
            .await?;

            database::tasks::update_users::add(
                &mut transaction,
                &user_id,
                *sqn_slice,
                &backup_network_id,
            )
            .await?;
        }

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
