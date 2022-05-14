use sqlx::sqlite::SqlitePool;
use sqlx::{Row, Sqlite, Transaction};

use crate::data::error::DauthError;

/// Creates the key share state table if it does not exist already.
pub async fn init_table(pool: &SqlitePool) -> Result<(), DauthError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS key_share_state_table (
            xres_star_hash BLOB NOT NULL,
            user_id TEXT NOT NULL,
            backup_network_id TEXT NOT NULL,
            PRIMARY KEY (xres_star_hash, backup_network_id)
        );",
    )
    .execute(pool)
    .await?;
    Ok(())
}

/* Queries */

/// Adds the key share as owned by the backup network.
/// Use xres* hash and the backup network id as the reference.
pub async fn add(
    transaction: &mut Transaction<'_, Sqlite>,
    xres_star_hash: &[u8],
    user_id: &str,
    backup_network_id: &str,
) -> Result<(), DauthError> {
    sqlx::query(
        "INSERT INTO key_share_state_table
        VALUES ($1,$2,$3)",
    )
    .bind(xres_star_hash)
    .bind(user_id)
    .bind(backup_network_id)
    .execute(transaction)
    .await?;

    Ok(())
}

/// Returns the user_id for the key share.
pub async fn get(
    transaction: &mut Transaction<'_, Sqlite>,
    xres_star_hash: &[u8],
    backup_network_id: &str,
) -> Result<String, DauthError> {
    let row = sqlx::query(
        "SELECT * FROM key_share_state_table
        WHERE (xres_star_hash,backup_network_id)=($1,$2)",
    )
    .bind(xres_star_hash)
    .bind(backup_network_id)
    .fetch_one(transaction)
    .await?;

    Ok(row.try_get::<String, &str>("user_id")?)
}

/// Deletes a key share reference if found.
pub async fn remove(
    transaction: &mut Transaction<'_, Sqlite>,
    xres_star_hash: &[u8],
    backup_network_id: &str,
) -> Result<(), DauthError> {
    sqlx::query(
        "DELETE FROM key_share_state_table
        WHERE (xres_star_hash,backup_network_id)=($1,$2)",
    )
    .bind(xres_star_hash)
    .bind(backup_network_id)
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

    use crate::database::{general, key_share_state};

    fn gen_name() -> String {
        let s: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();

        format!("sqlite_{}.db", s)
    }

    async fn init() -> (SqlitePool, TempDir) {
        let dir = tempdir().unwrap();
        let path = String::from(dir.path().join(gen_name()).to_str().unwrap());
        println!("Building temporary db: {}", path);

        let pool = general::build_pool(&path).await.unwrap();
        key_share_state::init_table(&pool).await.unwrap();

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
        for row in 0..num_rows {
            key_share_state::add(
                &mut transaction,
                &[row as u8; 1],
                "test_user_id",
                &format!("test_backup_network_{}", row),
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
        for row in 0..num_rows {
            key_share_state::add(
                &mut transaction,
                &[row as u8; 1],
                "test_user_id",
                &format!("test_backup_network_{}", row),
            )
            .await
            .unwrap();
        }
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        for row in 0..num_rows {
            assert_eq!(
                key_share_state::get(
                    &mut transaction,
                    &[row as u8; 1],
                    &format!("test_backup_network_{}", row),
                )
                .await
                .unwrap(),
                "test_user_id",
            );
        }
        transaction.commit().await.unwrap();
    }

    #[tokio::test]
    async fn test_remove() {
        let (pool, _dir) = init().await;
        let num_rows = 10;

        let mut transaction = pool.begin().await.unwrap();
        for row in 0..num_rows {
            key_share_state::add(
                &mut transaction,
                &[row as u8; 1],
                "test_user_id",
                &format!("test_backup_network_{}", row),
            )
            .await
            .unwrap();
        }
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        for row in 0..num_rows {
            assert_eq!(
                key_share_state::get(
                    &mut transaction,
                    &[row as u8; 1],
                    &format!("test_backup_network_{}", row),
                )
                .await
                .unwrap(),
                "test_user_id",
            );
        }
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        for row in 0..num_rows {
            key_share_state::remove(
                &mut transaction,
                &[row as u8; 1],
                &format!("test_backup_network_{}", row),
            )
            .await
            .unwrap();
        }
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        for row in 0..num_rows {
            assert!(key_share_state::get(
                &mut transaction,
                &[row as u8; 1],
                &format!("test_backup_network_{}", row),
            )
            .await
            .is_err());
        }
        transaction.commit().await.unwrap();
    }
}
