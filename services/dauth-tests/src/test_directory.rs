use std::sync::Arc;

use directory_service::data::error::DirectoryError;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use tempfile::{tempdir, TempDir};

use directory_service::data::config::DirectoryConfig;
use directory_service::data::context::DirectoryContext;
use tokio::task::JoinHandle;

/// Test directory object that wraps a directory instance/context and any
/// needed testing fields. Exposes functions that allow checking and
/// manipulation of the underlying instance of the directory.
pub struct TestDirectory {
    // Actual directory context
    pub context: Arc<DirectoryContext>,
    // Join handle to stop running
    join_handle: JoinHandle<()>,
    // Must not be dropped
    _temp_dir: TempDir,
}

impl TestDirectory {
    /// Builds a new test object with the provided host,
    /// but otherwise uses a default configuration.
    pub async fn new(host: &str) -> Result<Self, DirectoryError> {
        let rand_dir: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();
        let temp_dir = tempdir().or_else(|e| {
            Err(DirectoryError::ConfigError(format!(
                "Failed to generate temp dir {:?}",
                e
            )))
        })?;
        let database_path = String::from(
            temp_dir
                .path()
                .join(&rand_dir)
                .join("db.sqlite3")
                .to_str()
                .ok_or(DirectoryError::ConfigError(
                    "Failed to generate path".to_string(),
                ))?,
        );

        let config = DirectoryConfig {
            host_address: format!("{}:8900", host),
            database_path,
        };

        let context = directory_service::startup::build_context(config).await?;

        let temp_context = context.clone();
        let join_handle = tokio::spawn(async move {
            directory_service::rpc::server::start_server(temp_context).await;
        });

        Ok(Self {
            context,
            join_handle,
            _temp_dir: temp_dir,
        })
    }

    /// Aborts the internal server.
    pub fn stop(&self) {
        self.join_handle.abort()
    }

    /// Checks if all users in provided list exist, returns error if not.
    pub async fn check_users_exists(&self, user_ids: &Vec<String>) -> Result<(), DirectoryError> {
        let mut transaction = self.context.database_pool.begin().await?;
        for user_id in user_ids {
            directory_service::database::users::get(&mut transaction, &user_id).await?;
        }
        transaction.commit().await?;
        Ok(())
    }
}
