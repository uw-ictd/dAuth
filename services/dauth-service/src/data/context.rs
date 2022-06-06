use ed25519_dalek::{Keypair, PublicKey};
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
    pub metrics_context: MetricsContext,
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
    pub directory_network_cache: tokio::sync::Mutex<HashMap<String, (String, PublicKey)>>,
    pub directory_user_cache: tokio::sync::Mutex<HashMap<String, (String, Vec<String>)>>,
}

#[derive(Debug)]
pub struct RpcContext {
    pub host_addr: String,
    pub directory_addr: String,
    pub home_clients: tokio::sync::Mutex<HashMap<String, HomeNetworkClient<Channel>>>,
    pub backup_clients: tokio::sync::Mutex<HashMap<String, BackupNetworkClient<Channel>>>,
    pub directory_client: tokio::sync::Mutex<Option<DirectoryClient<Channel>>>,
    pub local_auth_addr: String,
}

#[derive(Debug)]
pub struct TasksContext {
    pub start_time: Instant,
    pub startup_delay: Duration,
    pub interval: Duration,
    pub is_registered: tokio::sync::Mutex<bool>,
    pub replace_key_share_delay: Duration,
}

/// Allow implementation of debug for TaskMonitor.
struct TaskMonitorDebug(tokio_metrics::TaskMonitor);

impl std::fmt::Debug for TaskMonitorDebug {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "TaskMonitorDebug")
    }
}

#[derive(Debug, Default)]
pub struct MetricsContext {
    monitors: tokio::sync::Mutex<HashMap<String, TaskMonitorDebug>>,
}

impl MetricsContext {
    /// Returns the monitor for the provided monitor id.
    /// Creates a new monitor if one does not exist.
    async fn get_monitor(&self, monitor_id: &str) -> tokio_metrics::TaskMonitor {
        let mut monitors = self.monitors.lock().await;

        match monitors.get(monitor_id) {
            Some(monitor) => monitor.0.clone(),
            None => {
                let monitor = tokio_metrics::TaskMonitor::new();
                monitors.insert(monitor_id.to_string(), TaskMonitorDebug(monitor.clone()));
                monitor
            }
        }
    }

    /// Returns a mapping of monitor ids to their current metrics.
    /// Metrics are cumulative up to the point of calling this function.
    async fn get_metrics(&self) -> HashMap<String, tokio_metrics::TaskMetrics> {
        let monitors = self.monitors.lock().await;
        let mut metrics = HashMap::with_capacity(monitors.len());

        for (monitor_id, monitor) in monitors.iter() {
            metrics.insert(monitor_id.clone(), monitor.0.cumulative());
        }

        metrics
    }
}
