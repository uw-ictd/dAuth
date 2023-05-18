use std::sync::Arc;

use dauth_service::data::error::DauthError;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use tempfile::{tempdir, TempDir};

use dauth_service::data::config::{DauthConfig, UserInfoConfig};
use dauth_service::data::context::DauthContext;
use tokio::task::JoinHandle;

/// Test dauth object that wraps a dauth instance/context and any
/// needed testing fields. Exposes functions that allow checking and
/// manipulation of the underlying instance of dAuth.
pub struct TestDauth {
    // Internal dauth context
    pub context: Arc<DauthContext>,
    // Join handle to stop running
    _join_handle: JoinHandle<()>,
    // Must not be dropped
    _temp_dir: TempDir,
}

impl TestDauth {
    /// Builds a new test object with the provided id and host,
    /// but otherwise uses a default configuration.
    pub async fn new(id: &str, host: &str, dir_host: &str) -> Result<Self, DauthError> {
        let rand_dir: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();
        let temp_dir = tempdir()?;
        let ed25519_keyfile_path = String::from(
            temp_dir
                .path()
                .join(&rand_dir)
                .join("ed25519_keys")
                .to_str()
                .ok_or(DauthError::ConfigError(
                    "Failed to generate path".to_string(),
                ))?,
        );
        let database_path = String::from(
            temp_dir
                .path()
                .join(&rand_dir)
                .join("db.sqlite3")
                .to_str()
                .ok_or(DauthError::ConfigError(
                    "Failed to generate path".to_string(),
                ))?,
        );

        let config = DauthConfig {
            id: id.to_string(),
            users: Vec::new(),
            host_addr: format!("{}:50052", host),
            local_auth_addr: Some(format!("{}:50051", host)),
            directory_addr: format!("{}:8900", dir_host),
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

        let context = dauth_service::startup::build_context(config).await?;

        let temp_context = context.clone();
        let join_handle = tokio::spawn(async move {
            dauth_service::tasks::task_manager::start(temp_context.clone())
                .await
                .expect("Failed to start task manager");

            dauth_service::rpc::server::start_servers(temp_context).await;
        });

        Ok(Self {
            context,
            _join_handle: join_handle,
            _temp_dir: temp_dir,
        })
    }

    /// Adds the provided users, panics on any failure.
    pub async fn add_users(&self, user_infos: &Vec<UserInfoConfig>) -> Result<(), DauthError> {
        for user_info in user_infos {
            dauth_service::management::add_user(self.context.clone(), user_info).await?;
        }

        Ok(())
    }

    /// Checks if all users in provided list exist, panics if not.
    pub async fn check_users_exists(&self, user_ids: &Vec<String>, sqn_slice: i64) -> Result<(), DauthError> {
        let mut transaction = self.context.local_context.database_pool.begin().await?;
        for user_id in user_ids {
            assert_eq!(
                &dauth_service::database::user_infos::get(&mut transaction, &user_id, sqn_slice)
                    .await?
                    .id,
                user_id
            );
        }
        transaction.commit().await?;
        Ok(())
    }


    /// Checks if all users in provided list exist, panics if not.
    pub async fn check_backup_user_exists(&self, user_ids: &Vec<String>) -> Result<(), DauthError> {
        let mut transaction = self.context.local_context.database_pool.begin().await?;
        for user_id in user_ids {
            dauth_service::database::backup_users::get(&mut transaction, &user_id).await?;
        }
        transaction.commit().await?;
        Ok(())
    }
}
