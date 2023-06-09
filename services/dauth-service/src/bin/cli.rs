use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_yaml;
use structopt::StructOpt;
use tracing_subscriber::{filter, EnvFilter};

use dauth_service::data::config::{BackupConfig, UserInfoConfig};
use dauth_service::rpc::dauth::management::management_client::ManagementClient;
use dauth_service::rpc::dauth::management::{add_user_req::Backup, AddUserReq};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "dAuth Management CLI",
    about = "Run management commands for an active instance of dAuth"
)]
enum CliOpt {
    /// Adds users through the specified config file.
    Config {
        #[structopt(short, long, parse(from_os_str))]
        config: PathBuf,
    },

    /// Adds a user directly through the CLI.
    /// sqn-max is used for backups as well.
    AddUser {
        host_addr: String,
        user_id: String,
        k: String,
        opc: String,
        sqn_max: i64,
        backup_ids: Vec<String>,
    },
}

/// Holds user data from a corresponding YAML file
#[derive(Serialize, Deserialize, Debug)]
pub struct CliConfig {
    pub users: Vec<UserInfoConfig>,
    pub host_addr: String,
}

#[tokio::main]
async fn main() {
    let log_filter = EnvFilter::builder()
        .with_default_directive(filter::LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::fmt().with_env_filter(log_filter).init();
    tracing::info!("Running dAuth management CLI");

    let config = match CliOpt::from_args() {
        CliOpt::Config { config } => {
            let yaml_string = std::fs::read_to_string(config).expect("Failed to read config");
            serde_yaml::from_str(&yaml_string).expect("Failed to parse yaml")
        }
        CliOpt::AddUser {
            host_addr,
            user_id,
            k,
            opc,
            sqn_max,
            backup_ids,
        } => {
            let sqn_max = if sqn_max % 32 == 0 {
                sqn_max
            } else {
                sqn_max + 32 - (sqn_max % 32)
            };
            let mut sqn_slice = 1;
            let mut backups = Vec::new();
            for backup_id in backup_ids {
                backups.push(BackupConfig {
                    backup_id,
                    sqn_slice,
                    sqn_max: sqn_max + sqn_slice,
                });
                sqn_slice += 1;
            }

            CliConfig {
                host_addr,
                users: vec![UserInfoConfig {
                    user_id,
                    k,
                    opc,
                    sqn_max,
                    backups,
                }],
            }
        }
    };

    let mut client = ManagementClient::connect(format!("http://{}", config.host_addr))
        .await
        .expect("Unable to connect to dauth server");

    for user_info in config.users {
        tracing::info!(?user_info, "Adding user");

        let mut backups = Vec::new();

        for backup in user_info.backups {
            backups.push(Backup {
                backup_id: backup.backup_id,
                slice: backup.sqn_slice,
                sqn_max: backup.sqn_slice,
            });
        }

        let res = client
            .add_user(tonic::Request::new(AddUserReq {
                user_id: user_info.user_id,
                k: user_info.k,
                opc: user_info.opc,
                sqn_max: user_info.sqn_max,
                backups,
            }))
            .await
            .expect("Unable to send request to dAuth server")
            .into_inner();

        if res.successful {
            tracing::info!("Request successful");
        } else {
            tracing::error!("Request failed")
        }

        if !res.info.is_empty() {
            tracing::warn!("dAuth response -- {}", res.info);
        }
    }
}
