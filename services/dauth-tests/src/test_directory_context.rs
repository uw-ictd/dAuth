use std::sync::Arc;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use tempfile::{tempdir, TempDir};

use directory_service::data::config::DirectoryConfig;
use directory_service::data::context::DirectoryContext;
use tokio::task::JoinHandle;

/// Test context that wraps a standard dAuth context.
/// Includes test-specific fields as well.
pub struct TestDirectoryContext {
    // Actual directory context
    pub context: Arc<DirectoryContext>,
    // Join handle to stop running
    _join_handle: JoinHandle<()>,
    // Must not be dropped
    _temp_dir: TempDir,
}

impl TestDirectoryContext {
    pub async fn new() -> Self {
        let rand_dir: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();
        let temp_dir = tempdir().unwrap();
        let database_path = String::from(
            temp_dir
                .path()
                .join(&rand_dir)
                .join("db.sqlite3")
                .to_str()
                .unwrap(),
        );

        let config = DirectoryConfig {
            host_address: "127.0.0.1:8900".to_string(),
            database_path,
        };

        let context = directory_service::startup::build_context(config)
            .await
            .unwrap();

        let temp_context = context.clone();
        let join_handle = tokio::spawn(async move {
            directory_service::rpc::server::start_server(temp_context).await;
        });

        Self {
            context,
            _join_handle: join_handle,
            _temp_dir: temp_dir,
        }
    }

    /// Checks if all users in provided list exist, panics if not.
    pub async fn check_users_exists(&self, user_ids: &Vec<String>) {
        let mut transaction = self.context.database_pool.begin().await.unwrap();
        for user_id in user_ids {
            directory_service::database::users::get(&mut transaction, &user_id)
                .await
                .unwrap();
        }
        transaction.commit().await.unwrap();
    }
}
