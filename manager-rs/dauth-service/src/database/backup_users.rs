use sqlx::sqlite::{SqlitePool, SqliteRow};
use sqlx::Error as SqlxError;
use sqlx::{Sqlite, Transaction};

use crate::data::error::DauthError;

/// Creates the backup users table if it does not exist already.
/// Contains all users that are backed up on this network
pub async fn init_table(pool: &SqlitePool) -> Result<(), DauthError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS backup_users_table (
            user_id TEXT NOT NULL,
            home_network_id TEXT NOT NULL,
            PRIMARY KEY (user_id,home_network_id)
        );",
    )
    .execute(pool)
    .await?;
    Ok(())
}

/* Queries */

/// Adds the user id to set of backups on this network
pub async fn add(
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &str,
    home_network_id: &str,
) -> Result<(), SqlxError> {
    sqlx::query(
        "INSERT INTO backup_users_table
        VALUES ($1,$2)",
    )
    .bind(user_id)
    .bind(home_network_id)
    .execute(transaction)
    .await?;

    Ok(())
}

/// Gets the home network of a given backed up user id
pub async fn get(
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &str,
) -> Result<SqliteRow, SqlxError> {
    Ok(sqlx::query(
        "SELECT * FROM backup_users_table
        WHERE user_id=$1;",
    )
    .bind(user_id)
    .fetch_one(transaction)
    .await?)
}

/// Removes the user id from the backups
pub async fn remove(
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &str,
    home_network_id: &str,
) -> Result<(), SqlxError> {
    sqlx::query(
        "DELETE FROM backup_users_table
        WHERE (user_id,home_network_id)=($1,$2)",
    )
    .bind(user_id)
    .bind(home_network_id)
    .execute(transaction)
    .await?;
    Ok(())
}

/* Testing */

#[cfg(test)]
mod tests {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use sqlx::SqlitePool;
    use tempfile::{tempdir, TempDir};

    use crate::database::{backup_users, general};

    fn gen_name() -> String {
        let s: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();

        format!("sqlite_{}.db", s)
    }

    async fn init() -> (SqlitePool, TempDir) {
        let dir = tempdir().unwrap();
        let path = String::from(dir.path().join(gen_name()).to_str().unwrap());
        println!("Building temporary db: {}", path);

        let pool = general::build_pool(&path).await.unwrap();
        backup_users::init_table(&pool).await.unwrap();

        (pool, dir)
    }

    /// Test that db and table creation will work
    #[tokio::test]
    async fn test_db_init() {
        init().await;
    }

    /// Test that insert works
    #[tokio::test]
    async fn test_add() {
        let (pool, _dir) = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;

        for row in 0..num_rows {
            backup_users::add(
                &mut transaction,
                &format!("test_user_id_{}", row),
                "test_home_network",
            )
            .await
            .unwrap();
        }
        transaction.commit().await.unwrap();
    }

    /// Tests that get works
    #[tokio::test]
    async fn test_get() {
        let (pool, _dir) = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;

        for row in 0..num_rows {
            backup_users::add(
                &mut transaction,
                &format!("test_user_id_{}", row),
                "test_home_network",
            )
            .await
            .unwrap();
        }

        transaction.commit().await.unwrap();
        let mut transaction = pool.begin().await.unwrap();

        for row in 0..num_rows {
            backup_users::get(&mut transaction, &format!("test_user_id_{}", row))
                .await
                .unwrap();
        }
        transaction.commit().await.unwrap();
    }

    /// Test that deletes work
    #[tokio::test]
    async fn test_remove() {
        let (pool, _dir) = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;

        for row in 0..num_rows {
            backup_users::add(
                &mut transaction,
                &format!("test_user_id_{}", row),
                "test_home_network",
            )
            .await
            .unwrap();
        }

        for row in 0..num_rows {
            backup_users::remove(
                &mut transaction,
                &format!("test_user_id_{}", row),
                "test_home_network",
            )
            .await
            .unwrap();
        }

        transaction.commit().await.unwrap();
        let mut transaction = pool.begin().await.unwrap();

        for row in 0..num_rows {
            assert!(
                backup_users::get(&mut transaction, &format!("test_user_id_{}", row),)
                    .await
                    .is_err()
            );
        }
        transaction.commit().await.unwrap();
    }
}
