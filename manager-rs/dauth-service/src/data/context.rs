use ed25519_dalek::Keypair;
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use tonic::transport::Channel;

use auth_vector::types::Id;

use crate::rpc::dauth::directory::directory_client::DirectoryClient;
use crate::rpc::dauth::remote::{
    backup_network_client::BackupNetworkClient, home_network_client::HomeNetworkClient,
};

/// Maintains the context for all components of
/// the dAuth service. All state exists here.
#[derive(Debug)]
pub struct DauthContext {
    pub local_context: LocalContext,
    pub remote_context: RemoteContext,
    pub rpc_context: RpcContext,
    pub tasks_context: TasksContext,
}

#[derive(Debug)]
pub struct LocalContext {
    pub id: String,
    pub database_pool: SqlitePool,
    pub local_user_id_min: Id,
    pub local_user_id_max: Id,
    pub signing_keys: Keypair,
}

#[derive(Debug)]
pub struct RemoteContext {
    pub backup_networks: Vec<String>,
}

#[derive(Debug)]
pub struct RpcContext {
    pub host_addr: String,
    pub home_clients: tokio::sync::Mutex<HashMap<String, HomeNetworkClient<Channel>>>,
    pub backup_clients: tokio::sync::Mutex<HashMap<String, BackupNetworkClient<Channel>>>,
    pub directory_client: tokio::sync::Mutex<DirectoryClient<Channel>>,
}

#[derive(Debug)]
pub struct TasksContext {
    pub start_time: SystemTime,
    pub startup_delay: Duration,
    pub interval: Duration,
}
