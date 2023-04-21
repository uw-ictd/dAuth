use std::sync::Arc;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use tempfile::{tempdir, TempDir};

use dauth_service::data::config::{DauthConfig, UserInfoConfig};
use dauth_service::data::context::DauthContext;

/// Test context that wraps a standard dAuth context.
/// Includes test-specific fields as well.
pub struct TestContext {
    // Actual dauth context
    pub context: Arc<DauthContext>,
    // Must not be dropped
    _temp_dir: TempDir,
}

impl TestContext {
    pub async fn build_test_context(id: String, users: Vec<UserInfoConfig>, addr: String) -> Self {
        let rand_dir: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();
        let temp_dir = tempdir().unwrap();
        let ed25519_keyfile_path = String::from(
            temp_dir
                .path()
                .join(&rand_dir)
                .join("ed25519_keys")
                .to_str()
                .unwrap(),
        );
        let database_path = String::from(
            temp_dir
                .path()
                .join(&rand_dir)
                .join("db.sqlite3")
                .to_str()
                .unwrap(),
        );

        let config = DauthConfig {
            id,
            users,
            host_addr: format!("{}:50052", addr),
            local_auth_addr: Some(format!("{}:50051", addr)),
            directory_addr: "127.0.0.1:8900".to_string(),
            ed25519_keyfile_path,
            database_path,
            task_startup_delay: 1.0,
            task_interval: 1.0,
            num_sqn_slices: 32,
            max_backup_vectors: 10,
            mcc: "901".to_string(),
            mnc: "70".to_string(),
            max_recorded_metrics: Some(1),
            backup_key_threshold: Some(1),
        };

        TestContext {
            context: dauth_service::startup::build_context(config).await.unwrap(),
            _temp_dir: temp_dir,
        }
    }
}

pub async fn run_test_env(context: Arc<DauthContext>) {
    dauth_service::tasks::task_manager::start(context.clone())
        .await
        .expect("Failed to start task manager");

    dauth_service::rpc::server::start_servers(context.clone()).await;
}
