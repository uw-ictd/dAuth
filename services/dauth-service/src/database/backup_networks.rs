use sqlx::sqlite::SqlitePool;
use sqlx::Row;
use sqlx::{Sqlite, Transaction};

use crate::data::error::DauthError;

/// Creates the backup networks table if it does not exist already.
/// Contains all networks that are used as a backup for this network
#[tracing::instrument(skip(pool), name = "database::backup_networks")]
pub async fn init_table(pool: &SqlitePool) -> Result<(), DauthError> {
    tracing::info!("Initialzing table");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS backup_networks_table (
            user_id TEXT NOT NULL,
            backup_network_id TEXT NOT NULL,
            seq_num_slice INT NOT NULL,
            PRIMARY KEY (user_id,backup_network_id)
        );",
    )
    .execute(pool)
    .await?;
    Ok(())
}

/* Queries */

/// Adds a network as a backup for the user id and seqnum slice.
/// Changes seqnum slice if user/network pair exists.
#[tracing::instrument(skip(transaction), name = "database::backup_networks")]
pub async fn upsert(
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &str,
    backup_network_id: &str,
    seqnum_slice: i64,
) -> Result<(), DauthError> {
    tracing::debug!("Upserting backup info");

    sqlx::query(
        "REPLACE INTO backup_networks_table
        VALUES ($1,$2,$3)",
    )
    .bind(user_id)
    .bind(backup_network_id)
    .bind(seqnum_slice)
    .execute(transaction)
    .await?;

    Ok(())
}

/// Gets the backup info for a given network and user id
/// Not currently used.
#[allow(dead_code)]
#[tracing::instrument(skip(transaction), name = "database::backup_networks")]
pub async fn get(
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &str,
    backup_network_id: &str,
) -> Result<i64, DauthError> {
    tracing::debug!("Getting backup info");

    Ok(sqlx::query(
        "SELECT * FROM backup_networks_table
        WHERE (user_id,backup_network_id)=($1,$2);",
    )
    .bind(user_id)
    .bind(backup_network_id)
    .fetch_one(transaction)
    .await?
    .try_get::<i64, &str>("seq_num_slice")?)
}

/// Gets all backup network ids for the user.
#[tracing::instrument(skip(transaction), name = "database::backup_networks")]
pub async fn get_all(
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &str,
) -> Result<Vec<String>, DauthError> {
    tracing::debug!("Getting backup info");

    let rows = sqlx::query(
        "SELECT * FROM backup_networks_table
        WHERE (user_id)=($1);",
    )
    .bind(user_id)
    .fetch_all(transaction)
    .await?;

    let mut results = Vec::new();

    for row in rows {
        results.push(row.try_get::<String, &str>("backup_network_id")?);
    }

    return Ok(results)
}

/// Gets the seqnum slice for a given network and user id
#[tracing::instrument(skip(transaction), name = "database::backup_networks")]
pub async fn get_slice(
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &str,
    backup_network_id: &str,
) -> Result<i64, DauthError> {
    tracing::debug!("Getting seqnum slice");

    Ok(sqlx::query(
        "SELECT seq_num_slice FROM backup_networks_table
        WHERE (user_id,backup_network_id)=($1,$2);",
    )
    .bind(user_id)
    .bind(backup_network_id)
    .fetch_one(transaction)
    .await?
    .try_get::<i64, &str>("seq_num_slice")?)
}

/// Removes the network as a backup for this network
/// Not currently used.
#[allow(dead_code)]
#[tracing::instrument(skip(transaction), name = "database::backup_networks")]
pub async fn remove(
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &str,
    backup_network_id: &str,
) -> Result<(), DauthError> {
    tracing::debug!("Removing backup info");

    sqlx::query(
        "DELETE FROM backup_networks_table
        WHERE (user_id,backup_network_id)=($1,$2)",
    )
    .bind(user_id)
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

    use crate::database::{backup_networks, general};

    fn gen_name() -> String {
        let s: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();

        format!("sqlite_{}.db", s)
    }

    async fn init() -> (SqlitePool, TempDir) {
        let dir = tempdir().unwrap();
        let path = String::from(dir.path().join(gen_name()).to_str().unwrap());
        println!("Building temporary db: {}", path);

        let pool = general::build_pool(&path).await.unwrap();
        backup_networks::init_table(&pool).await.unwrap();

        (pool, dir)
    }

    /// Test that db and table creation will work
    #[tokio::test]
    async fn test_db_init() {
        init().await;
    }

    /// Test that insert works
    #[tokio::test]
    async fn test_upsert() {
        let (pool, _dir) = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                backup_networks::upsert(
                    &mut transaction,
                    &format!("test_user_id_{}", row),
                    &format!("test_network_id_{}", section),
                    section,
                )
                .await
                .unwrap();
            }
        }
        transaction.commit().await.unwrap();
    }

    /// Tests that get works
    #[tokio::test]
    async fn test_get() {
        let (pool, _dir) = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                backup_networks::upsert(
                    &mut transaction,
                    &format!("test_user_id_{}", row),
                    &format!("test_network_id_{}", section),
                    section,
                )
                .await
                .unwrap();
            }
        }

        transaction.commit().await.unwrap();
        let mut transaction = pool.begin().await.unwrap();

        for section in 0..num_sections {
            for row in 0..num_rows {
                let res = backup_networks::get(
                    &mut transaction,
                    &format!("test_user_id_{}", row),
                    &format!("test_network_id_{}", section),
                )
                .await
                .unwrap();

                assert_eq!(section, res);
            }
        }
        transaction.commit().await.unwrap();
    }

    /// Test that deletes work
    #[tokio::test]
    async fn test_get_all() {
        let (pool, _dir) = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_sections = 10;

        for section in 0..num_sections {
            backup_networks::upsert(
                &mut transaction,
                &format!("test_user_id"),
                &format!("test_network_id_{}", section),
                section,
            )
            .await
            .unwrap();
        }
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        let results = backup_networks::get_all(&mut transaction, "test_user_id").await.unwrap();
        transaction.commit().await.unwrap();

        for section in 0..num_sections {
            assert!(results.contains(&format!("test_network_id_{}", section)));
        }
    }

    /// Test that deletes work
    #[tokio::test]
    async fn test_remove() {
        let (pool, _dir) = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                backup_networks::upsert(
                    &mut transaction,
                    &format!("test_user_id_{}", row),
                    &format!("test_network_id_{}", section),
                    section,
                )
                .await
                .unwrap();
            }
        }

        for section in 0..num_sections {
            for row in 0..num_rows {
                backup_networks::remove(
                    &mut transaction,
                    &format!("test_user_id_{}", row),
                    &format!("test_network_id_{}", section),
                )
                .await
                .unwrap();
            }
        }

        transaction.commit().await.unwrap();
        let mut transaction = pool.begin().await.unwrap();

        for section in 0..num_sections {
            for row in 0..num_rows {
                assert!(backup_networks::get(
                    &mut transaction,
                    &format!("test_user_id_{}", row),
                    &format!("test_network_id_{}", section),
                )
                .await
                .is_err());
            }
        }
        transaction.commit().await.unwrap();
    }
}
