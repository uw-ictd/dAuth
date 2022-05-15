use ed25519_dalek::Keypair;
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tonic::transport::Channel;

use crate::data::state::AuthState;
use crate::rpc::dauth::directory::directory_client::DirectoryClient;
use crate::rpc::dauth::remote::{
    backup_network_client::BackupNetworkClient, home_network_client::HomeNetworkClient,
};

/// Maintains the context for all components of
/// the dAuth service. All state exists here.
#[derive(Debug)]
pub struct DauthContext {
    pub local_context: LocalContext,
    pub backup_context: BackupContext,
    pub rpc_context: RpcContext,
    pub tasks_context: TasksContext,
}

#[derive(Debug)]
pub struct LocalContext {
    pub id: String,
    pub database_pool: SqlitePool,
    pub signing_keys: Keypair,
    pub num_sqn_slices: i64,
    pub max_backup_vectors: i64,
}

#[derive(Debug)]
pub struct BackupContext {
    pub auth_states: tokio::sync::Mutex<HashMap<String, AuthState>>,
}

#[derive(Debug)]
pub struct RpcContext {
    pub host_addr: String,
    pub directory_addr: String,
    pub home_clients: tokio::sync::Mutex<HashMap<String, HomeNetworkClient<Channel>>>,
    pub backup_clients: tokio::sync::Mutex<HashMap<String, BackupNetworkClient<Channel>>>,
    pub directory_client: tokio::sync::Mutex<Option<DirectoryClient<Channel>>>,
}

#[derive(Debug)]
pub struct TasksContext {
    pub start_time: Instant,
    pub startup_delay: Duration,
    pub interval: Duration,
    pub is_registered: tokio::sync::Mutex<bool>,
    pub replace_key_share_delay: Duration,
}
