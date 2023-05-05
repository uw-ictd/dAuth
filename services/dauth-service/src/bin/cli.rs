use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_yaml;
use structopt::StructOpt;
use tracing_subscriber::{filter, EnvFilter};

use dauth_service::rpc::dauth::management::management_client::ManagementClient;
use dauth_service::rpc::dauth::management::{add_user_req::Backup, AddUserReq};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "dAuth Management CLI",
    about = "Run management commands for a running instance of dAuth"
)]
struct CliOpt {
    /// Yaml config file path
    #[structopt(parse(from_os_str))]
    pub config_path: PathBuf,
}

/// All info needed to configure a user.
#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfoConfig {
    pub k: String,
    pub opc: String,
    pub sqn_max: i64,
    pub backups: HashMap<String, (i64, i64)>,
}

/// Holds user data from a corresponding YAML file
#[derive(Serialize, Deserialize, Debug)]
pub struct CliConfig {
    pub users: HashMap<String, UserInfoConfig>,
    pub host_addr: String,
}

#[tokio::main]
async fn main() {
    let log_filter = EnvFilter::builder()
        .with_default_directive(filter::LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::fmt().with_env_filter(log_filter).init();
    tracing::info!("Running dAuth management CLI");

    let config = build_config(CliOpt::from_args().config_path);

    let mut client = ManagementClient::connect(format!("http://{}", config.host_addr))
        .await
        .expect("Unable to connect to dauth server");

    for (user_id, user_info) in config.users {
        tracing::info!(?user_id, ?user_info, "Adding user");

        let mut backups = Vec::new();

        for (backup_id, (slice, sqn_max)) in user_info.backups {
            backups.push(Backup {
                backup_id,
                slice,
                sqn_max,
            });
        }

        let res = client
            .add_user(tonic::Request::new(AddUserReq {
                user_id,
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

fn build_config(yaml_path: PathBuf) -> CliConfig {
    let yaml_string = std::fs::read_to_string(yaml_path).expect("Failed to read config");
    serde_yaml::from_str(&yaml_string).expect("Failed to parse yaml")
}
