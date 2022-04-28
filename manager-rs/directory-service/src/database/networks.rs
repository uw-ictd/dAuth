use sqlx::sqlite::{SqlitePool, SqliteRow};
use sqlx::Error as SqlxError;
use sqlx::{Sqlite, Transaction};

use crate::data::error::DirectoryError;

/// Creates the backup networks table if it does not exist already.
/// Contains all networks that are used as a backup for this network
pub async fn init_table(pool: &SqlitePool) -> Result<(), DirectoryError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS networks_directory_table (
            network_id TEXT PRIMARY KEY,
            address TEXT NOT NULL,
            public_key BLOB NOT NULL
        );",
    )
    .execute(pool)
    .await?;
    Ok(())
}

/* Queries */

/// Adds a network with its address and public key.
pub async fn upsert(
    transaction: &mut Transaction<'_, Sqlite>,
    network_id: &str,
    address: &str,
    public_key: &Vec<u8>,
) -> Result<(), SqlxError> {
    sqlx::query(
        "REPLACE INTO networks_directory_table
        VALUES ($1,$2,$3)",
    )
    .bind(network_id)
    .bind(address)
    .bind(public_key)
    .execute(transaction)
    .await?;

    Ok(())
}

/// Gets the address and public key of a given network.
pub async fn get(
    transaction: &mut Transaction<'_, Sqlite>,
    network_id: &str,
) -> Result<SqliteRow, SqlxError> {
    Ok(sqlx::query(
        "SELECT * FROM networks_directory_table
        WHERE network_id=$1;",
    )
    .bind(network_id)
    .fetch_one(transaction)
    .await?)
}

/* Testing */

#[cfg(test)]
mod tests {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use sqlx::{Row, SqlitePool};
    use tempfile::{tempdir, TempDir};

    use crate::database::{general, networks};

    fn gen_name() -> String {
        let s: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();

        format!("sqlite_{}.db", s)
    }

    async fn init() -> (SqlitePool, TempDir) {
        let dir = tempdir().unwrap();
        let path = String::from(dir.path().join(gen_name()).to_str().unwrap());
        println!("Building temporary db: {}", path);

        let pool = general::build_pool(&path).await.unwrap();
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
        for row in 0..num_rows {
            networks::upsert(
                &mut transaction,
                &format!("test_network_id_{}", row),
                &format!("test_address_{}", row),
                &vec![0, 0, 0],
            )
            .await
            .unwrap();
        }
        transaction.commit().await.unwrap();
    }

    #[tokio::test]
    async fn test_update() {
        let (pool, _dir) = init().await;

        let mut transaction = pool.begin().await.unwrap();
        networks::upsert(
            &mut transaction,
            "test_network_0",
            "test_address_0",
            &vec![0, 0, 0],
        )
        .await
        .unwrap();
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        let res = networks::get(&mut transaction, "test_network_0")
            .await
            .unwrap();
        assert_eq!("test_address_0", res.get_unchecked::<&str, &str>("address"));
        assert_eq!(
            vec![0, 0, 0],
            res.get_unchecked::<Vec<u8>, &str>("public_key")
        );
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        networks::upsert(
            &mut transaction,
            "test_network_0",
            "test_address_0a",
            &vec![1, 1, 1],
        )
        .await
        .unwrap();
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        let res = networks::get(&mut transaction, "test_network_0")
            .await
            .unwrap();
        assert_eq!(
            "test_address_0a",
            res.get_unchecked::<&str, &str>("address")
        );
        assert_eq!(
            vec![1, 1, 1],
            res.get_unchecked::<Vec<u8>, &str>("public_key")
        );
        transaction.commit().await.unwrap();
    }

    /// Tests that get works
    #[tokio::test]
    async fn test_get() {
        let (pool, _dir) = init().await;

        let num_rows = 10;

        let mut transaction = pool.begin().await.unwrap();
        for row in 0..num_rows {
            networks::upsert(
                &mut transaction,
                &format!("test_network_id_{}", row),
                &format!("test_address_{}", row),
                &vec![0, 0, 0],
            )
            .await
            .unwrap();
        }
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        for row in 0..num_rows {
            let res = networks::get(&mut transaction, &format!("test_network_id_{}", row))
                .await
                .unwrap();

            assert_eq!(
                &format!("test_address_{}", row),
                res.get_unchecked::<&str, &str>("address")
            );
            assert_eq!(
                vec![0, 0, 0],
                res.get_unchecked::<Vec<u8>, &str>("public_key")
            );
        }
        transaction.commit().await.unwrap();
    }
}
