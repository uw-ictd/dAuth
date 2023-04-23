use std::sync::Arc;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use tempfile::{tempdir, TempDir};

use dauth_service::data::config::{DauthConfig, UserInfoConfig};
use dauth_service::data::context::DauthContext;
use tokio::task::JoinHandle;

/// Known functional K.
pub const TEST_K: &str = "465B5CE8B199B49FAA5F0A2EE238A6BC";
/// Known functional OPC.
pub const TEST_OPC: &str = "E8ED289DEBA952E4283B54E88E6183CA";

/// Test context that wraps a standard dAuth context.
/// Includes test-specific fields as well.
pub struct TestDauthContext {
    // Actual dauth context
    pub context: Arc<DauthContext>,
    // Join handle to stop running
    _join_handle: JoinHandle<()>,
    // Must not be dropped
    _temp_dir: TempDir,
}

impl TestDauthContext {
    pub async fn new(id: String, addr: String) -> Self {
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
            users: Vec::new(),
            host_addr: format!("{}:50052", addr),
            local_auth_addr: Some(format!("{}:50051", addr)),
            directory_addr: "127.0.0.1:8900".to_string(),
            ed25519_keyfile_path,
            database_path,
            task_startup_delay: 0.1,
            task_interval: 0.1,
            num_sqn_slices: 32,
            max_backup_vectors: 10,
            mcc: "901".to_string(),
            mnc: "70".to_string(),
            max_recorded_metrics: Some(1),
            backup_key_threshold: Some(1),
        };

        let context = dauth_service::startup::build_context(config).await.unwrap();

        let temp_context = context.clone();
        let join_handle = tokio::spawn(async move {
            dauth_service::tasks::task_manager::start(temp_context.clone())
                .await
                .expect("Failed to start task manager");

            dauth_service::rpc::server::start_servers(temp_context).await;
        });

        Self {
            context,
            _join_handle: join_handle,
            _temp_dir: temp_dir,
        }
    }

    /// Adds num_user users, panics on any failure.
    pub async fn add_users(&self, num_users: usize) -> Vec<String> {
        let mut users = Vec::new();
        for user_id in 0..num_users {
            let user_id = format!("user-{}-{}", self.context.local_context.id, user_id);
            dauth_service::management::add_user(
                self.context.clone(),
                &UserInfoConfig {
                    user_id: user_id.clone(),
                    k: TEST_K.to_string(),
                    opc: TEST_OPC.to_string(),
                    sqn_max: 32,
                    backups: Vec::new(),
                },
            )
            .await
            .unwrap();

            users.push(user_id);
        }

        users
    }

    /// Checks if all users in provided list exist, panics if not.
    pub async fn check_users_exists(&self, user_ids: &Vec<String>) {
        let mut transaction = self
            .context
            .local_context
            .database_pool
            .begin()
            .await
            .unwrap();
        for user_id in user_ids {
            assert_eq!(
                &dauth_service::database::user_infos::get(&mut transaction, &user_id, 0)
                    .await
                    .unwrap()
                    .id,
                user_id
            );
        }
        transaction.commit().await.unwrap();
    }
}
