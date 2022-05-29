use sqlx::ConnectOptions;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};

use crate::data::error::DauthError;
use crate::database;

/// Constructs the sqlite pool for running queries.
pub async fn build_pool(database_path: &str) -> Result<SqlitePool, DauthError> {
    Ok(SqlitePoolOptions::new()
        .max_connections(10)
        .connect_with(
            SqliteConnectOptions::new()
                .disable_statement_logging()
                .clone()
                .create_if_missing(true)
                .filename(database_path),
        )
        .await?)
}

/// Builds the database connection pool.
/// Creates the database and tables if they don't exist.
pub async fn database_init(database_path: &str) -> Result<SqlitePool, DauthError> {
    let path = std::path::Path::new(database_path);
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();

    let pool: SqlitePool = database::general::build_pool(database_path).await?;

    database::flood_vectors::init_table(&pool).await?;
    database::auth_vectors::init_table(&pool).await?;
    database::kseafs::init_table(&pool).await?;
    database::user_infos::init_table(&pool).await?;
    database::key_shares::init_table(&pool).await?;
    database::backup_networks::init_table(&pool).await?;
    database::backup_users::init_table(&pool).await?;
    database::vector_state::init_table(&pool).await?;
    database::tasks::update_users::init_table(&pool).await?;
    database::tasks::replace_key_shares::init_table(&pool).await?;
    database::tasks::report_key_shares::init_table(&pool).await?;
    database::tasks::report_auth_vectors::init_table(&pool).await?;

    Ok(pool)
}

/* Testing */

#[cfg(test)]
mod tests {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use sqlx::SqlitePool;
    use tempfile::tempdir;

    use crate::database;

    fn gen_name() -> String {
        let s: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();

        format!("sqlite_{}.db", s)
    }

    async fn init() -> SqlitePool {
        let dir = tempdir().unwrap();
        let path = String::from(dir.path().join(gen_name()).to_str().unwrap());
        println!("Building temporary db: {}", path);

        let pool = database::general::database_init(&path).await.unwrap();

        pool
    }

    /// Test that db and table creation will work
    #[tokio::test]
    async fn test_db_init() {
        init().await;
    }
}
