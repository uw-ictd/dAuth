use sqlx::sqlite::SqlitePool;
use sqlx::{Row, Sqlite, Transaction};

use crate::data::error::DirectoryError;

/// Creates the backup networks table if it does not exist already.
/// Contains all networks that are used as a backup for this network
pub async fn init_table(pool: &SqlitePool) -> Result<(), DirectoryError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users_directory_table (
            user_id TEXT PRIMARY KEY,
            home_network_id TEXT NOT NULL
                REFERENCES networks_directory_table(network_id)
        );",
    )
    .execute(pool)
    .await?;
    Ok(())
}

/* Queries */

/// Adds a user with its home network.
pub async fn add(
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &str,
    home_network_id: &str,
) -> Result<(), DirectoryError> {
    sqlx::query(
        "INSERT INTO users_directory_table
        VALUES ($1,$2)",
    )
    .bind(user_id)
    .bind(home_network_id)
    .execute(transaction)
    .await?;

    Ok(())
}

/// Gets the home network id of the user.
pub async fn get(
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &str,
) -> Result<String, DirectoryError> {
    let row = sqlx::query(
        "SELECT * FROM users_directory_table
        WHERE user_id=$1;",
    )
    .bind(user_id)
    .fetch_one(transaction)
    .await?;

    Ok(row.try_get::<String, &str>("home_network_id")?)
}

/* Testing */

#[cfg(test)]
mod tests {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use sqlx::SqlitePool;
    use tempfile::{tempdir, TempDir};

    use crate::database::{general, networks, users};

    fn gen_name() -> String {
        let s: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();

        format!("sqlite_{}.db", s)
    }

    async fn init() -> (SqlitePool, TempDir) {
        let dir = tempdir().unwrap();
        let path = String::from(dir.path().join(gen_name()).to_str().unwrap());
        println!("Building temporary db: {}", path);

        let pool = general::build_pool(&path).await.unwrap();
        users::init_table(&pool).await.unwrap();
        networks::init_table(&pool).await.unwrap();

        (pool, dir)
    }

    #[tokio::test]
    async fn test_db_init() {
        init().await;
    }

    #[tokio::test]
    async fn test_add() {
        let (pool, _dir) = init().await;
        let num_rows = 10;

        let mut transaction = pool.begin().await.unwrap();
        networks::upsert(
            &mut transaction,
            &format!("test_home_network_id"),
            &format!("test_home_address"),
            &vec![0],
        )
        .await
        .unwrap();
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        for row in 0..num_rows {
            users::add(
                &mut transaction,
                &format!("test_user_id_{}", row),
                &format!("test_home_network_id"),
            )
            .await
            .unwrap();
        }
        transaction.commit().await.unwrap();
    }

    #[tokio::test]
    async fn test_get() {
        let (pool, _dir) = init().await;
        let num_rows = 10;

        let mut transaction = pool.begin().await.unwrap();
        networks::upsert(
            &mut transaction,
            &format!("test_home_network_id"),
            &format!("test_home_address"),
            &vec![0],
        )
        .await
        .unwrap();
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        for row in 0..num_rows {
            users::add(
                &mut transaction,
                &format!("test_user_id_{}", row),
                &format!("test_home_network_id"),
            )
            .await
            .unwrap();
        }
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        for row in 0..num_rows {
            let home_network_id = users::get(&mut transaction, &format!("test_user_id_{}", row))
                .await
                .unwrap();

            assert_eq!("test_home_network_id", home_network_id);
        }
        transaction.commit().await.unwrap();
    }

    #[tokio::test]
    async fn test_add_without_foreign_key_fail() {
        let (pool, _dir) = init().await;

        let mut transaction = pool.begin().await.unwrap();
        assert!(users::add(
            &mut transaction,
            &format!("test_user_id"),
            &format!("test_home_network"),
        )
        .await
        .is_err());
    }
}
