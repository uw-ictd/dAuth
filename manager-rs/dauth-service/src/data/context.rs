use ed25519_dalek::Keypair;
use sqlx::SqlitePool;
use std::time::{Duration, SystemTime};

/// Maintains the context for all components of
/// the dAuth service. All state exists here.
#[derive(Debug)]
pub struct DauthContext {
    pub local_context: LocalContext,
    pub rpc_context: RpcContext,
    pub tasks_context: TasksContext,
}

#[derive(Debug)]
pub struct LocalContext {
    pub id: String,
    pub database_pool: SqlitePool,
    pub signing_keys: Keypair,
}

#[derive(Debug)]
pub struct RpcContext {
    pub host_addr: String,
    pub directory_addr: String,
}

#[derive(Debug)]
pub struct TasksContext {
    pub start_time: SystemTime,
    pub startup_delay: Duration,
    pub interval: Duration,
    pub is_registered: tokio::sync::Mutex<bool>,
}
