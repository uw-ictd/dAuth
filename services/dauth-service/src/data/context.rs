use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

use ed25519_dalek::{Keypair, PublicKey};
use sqlx::SqlitePool;
use tokio_metrics::{TaskMetrics, TaskMonitor};
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
    pub metrics_report_interval: Duration,
    pub metrics_last_report: tokio::sync::Mutex<Instant>,
}

#[derive(Debug)]
pub struct MetricsContext {
    pub max_recorded_metrics: usize,
    pub metrics_map: tokio::sync::Mutex<HashMap<String, VecDeque<TaskMetrics>>>,
}

impl MetricsContext {
    /// Records the metrics data from the monitor and stores it under
    /// the provided metrics id.
    pub async fn record_metrics(&self, metrics_id: &str, monitor: TaskMonitor) {
        if self.max_recorded_metrics > 0 {
            let mut metrics_map = self.metrics_map.lock().await;

            match metrics_map.get_mut(metrics_id) {
                Some(queue) => {
                    if queue.len() >= self.max_recorded_metrics {
                        queue.pop_front();
                    }
                    queue.push_back(monitor.cumulative());
                }
                None => {
                    let mut queue = VecDeque::with_capacity(self.max_recorded_metrics);
                    queue.push_back(monitor.cumulative());
                    metrics_map.insert(metrics_id.to_string(), queue);
                }
            }
        }
    }

    /// Returns the set of metrics for each metric id.
    pub async fn get_metrics(&self) -> HashMap<String, VecDeque<TaskMetrics>> {
        let metrics_map = self.metrics_map.lock().await;

        metrics_map.clone()
    }
}
