use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};

use crate::data::error::DauthError;

/// Constructs the sqlite pool for running queries.
pub async fn build_pool(database_path: &str) -> Result<SqlitePool, DauthError> {
    Ok(SqlitePoolOptions::new()
        .max_connections(10)
        .connect_with(
            SqliteConnectOptions::new()
                .create_if_missing(true)
                .filename(database_path),
        )
        .await?)
}

/* Testing */

#[cfg(test)]
mod tests {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use sqlx::SqlitePool;
    use tempfile::tempdir;

    use crate::local::queries;

    fn gen_name() -> String {
        let s: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();

        format!("sqlite_{}.db", s)
    }

    async fn init() -> SqlitePool {
        let dir = tempdir().unwrap();
        let path = String::from(dir.path().join(gen_name()).to_str().unwrap());
        println!("Building temporary db: {}", path);

        let pool = queries::general::build_pool(&path).await.unwrap();
        queries::flood_vectors::init_table(&pool).await.unwrap();
        queries::auth_vectors::init_table(&pool).await.unwrap();
        queries::kseafs::init_table(&pool).await.unwrap();
        queries::user_infos::init_table(&pool).await.unwrap();
        queries::key_shares::init_table(&pool).await.unwrap();
        queries::backup_networks::init_table(&pool).await.unwrap();
        queries::backup_users::init_table(&pool).await.unwrap();

        pool
    }

    /// Test that db and table creation will work
    #[tokio::test]
    async fn test_db_init() {
        init().await;
    }
}
