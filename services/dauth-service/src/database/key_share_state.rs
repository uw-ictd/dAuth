use auth_vector::types::Rand;
use sqlx::sqlite::SqlitePool;
use sqlx::{FromRow, Row, Sqlite, Transaction};

use auth_vector::types::{XResHash, XResStarHash};

use crate::data::error::DauthError;

#[allow(dead_code)]
#[derive(Debug, FromRow, Clone)]
struct ShareStateRow {
    user_id: String,
    backup_network_id: String,
    rand: Vec<u8>,
    xres_star_hash: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShareState {
    pub user_id: String,
    pub rand: Rand,
}

impl TryFrom<ShareStateRow> for ShareState {
    type Error = DauthError;
    fn try_from(value: ShareStateRow) -> Result<Self, Self::Error> {
        Ok(ShareState {
            user_id: value.user_id,
            rand: value.rand.try_into()?,
        })
    }
}

/// Creates the key share state table if it does not exist already.
#[tracing::instrument(skip(pool), name = "database::key_share_state")]
pub async fn init_table(pool: &SqlitePool) -> Result<(), DauthError> {
    tracing::info!("Initialzing table");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS key_share_state_table (
            xres_star_hash BLOB NOT NULL,
            xres_hash BLOB NOT NULL,
            backup_network_id TEXT NOT NULL,
            user_id TEXT NOT NULL,
            rand BLOB NOT NULL,
            PRIMARY KEY (xres_star_hash, backup_network_id)
        );",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_key_share_state_xres_hash
        ON key_share_state_table (xres_hash, backup_network_id);",
    )
    .execute(pool)
    .await?;

    Ok(())
}

/* Queries */

/// Adds the key share as owned by the backup network.
/// Use xres* hash and the backup network id as the reference.
#[tracing::instrument(skip(transaction), name = "database::key_share_state")]
pub async fn add(
    transaction: &mut Transaction<'_, Sqlite>,
    xres_star_hash: &XResStarHash,
    xres_hash: &XResHash,
    backup_network_id: &str,
    user_id: &str,
    rand: &[u8],
) -> Result<(), DauthError> {
    tracing::debug!("Adding key share state");

    sqlx::query(
        "INSERT INTO key_share_state_table
        VALUES ($1,$2,$3,$4,$5)",
    )
    .bind(xres_star_hash.as_slice())
    .bind(xres_hash.as_slice())
    .bind(backup_network_id)
    .bind(user_id)
    .bind(rand)
    .execute(transaction)
    .await?;

    Ok(())
}

/// Returns the user_id and rand for the key share.
#[tracing::instrument(skip(transaction), name = "database::key_share_state")]
pub async fn get_by_xres_star_hash(
    transaction: &mut Transaction<'_, Sqlite>,
    xres_star_hash: &XResStarHash,
    backup_network_id: &str,
) -> Result<Option<ShareState>, DauthError> {
    tracing::debug!("Getting key share state with xres* hash");

    let possible_row: Option<ShareStateRow> = sqlx::query_as(
        "SELECT * FROM key_share_state_table
        WHERE (xres_star_hash,backup_network_id)=($1,$2)",
    )
    .bind(xres_star_hash.as_slice())
    .bind(backup_network_id)
    .fetch_optional(transaction)
    .await?;

    match possible_row {
        Some(row) => Ok(Some(row.try_into()?)),
        None => Ok(None),
    }
}

/// Returns the user_id and rand for the key share.
#[tracing::instrument(skip(transaction), name = "database::key_share_state")]
pub async fn get_by_xres_hash(
    transaction: &mut Transaction<'_, Sqlite>,
    xres_hash: &XResHash,
    backup_network_id: &str,
) -> Result<Option<ShareState>, DauthError> {
    tracing::debug!("Getting key share state with xres hash");

    let possible_row: Option<ShareStateRow> = sqlx::query_as(
        "SELECT * FROM key_share_state_table
        WHERE (xres_hash,backup_network_id)=($1,$2)",
    )
    .bind(xres_hash.as_slice())
    .bind(backup_network_id)
    .fetch_optional(transaction)
    .await?;

    match possible_row {
        Some(row) => Ok(Some(row.try_into()?)),
        None => Ok(None),
    }
}

#[allow(dead_code)]
#[tracing::instrument(skip(transaction), name = "database::key_share_state")]
pub async fn get_all(
    transaction: &mut Transaction<'_, Sqlite>,
    xres_star_hash: &[u8],
    backup_network_id: &str,
) -> Result<(), DauthError> {
    let rows = sqlx::query("SELECT * FROM key_share_state_table")
        .bind(xres_star_hash)
        .bind(backup_network_id)
        .fetch_all(transaction)
        .await?;

    tracing::info!("dumping key share state");

    for row in rows {
        tracing::info!("{}", row.try_get::<String, &str>("user_id")?);
        tracing::info!("{:?}", row.try_get::<Vec<u8>, &str>("xres_star_hash")?);
        tracing::info!("{:?}", row.try_get::<String, &str>("backup_network_id")?);
        // tracing::info!("{}", row.try_get::<Vec<u8>, &str>("rand")?[..].try_into()?);
    }

    Ok(())
}

/// Deletes a key share reference if found.
#[tracing::instrument(skip(transaction), name = "database::key_share_state")]
pub async fn remove_by_xres_star_hash(
    transaction: &mut Transaction<'_, Sqlite>,
    xres_star_hash: &[u8],
    backup_network_id: &str,
) -> Result<(), DauthError> {
    tracing::debug!("Removing key share state with xres* hash");

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

#[tracing::instrument(skip(transaction), name = "database::key_share_state")]
pub async fn remove_by_xres_hash(
    transaction: &mut Transaction<'_, Sqlite>,
    xres_hash: &XResHash,
    backup_network_id: &str,
) -> Result<(), DauthError> {
    tracing::debug!("Removing key share state with xres hash");

    sqlx::query(
        "DELETE FROM key_share_state_table
        WHERE (xres_hash,backup_network_id)=($1,$2)",
    )
    .bind(xres_hash.as_slice())
    .bind(backup_network_id)
    .execute(transaction)
    .await?;

    Ok(())
}

/* Testing */

#[cfg(test)]
mod tests {
    use auth_vector::types::RAND_LENGTH;
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
                &[row as u8; 16],
                &[row as u8; 16],
                &format!("test_backup_network_{}", row),
                "test_user_id",
                &[0u8; RAND_LENGTH],
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
                &[row as u8; 16],
                &[row as u8; 16],
                &format!("test_backup_network_{}", row),
                "test_user_id",
                &[0u8; RAND_LENGTH],
            )
            .await
            .unwrap();
        }
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        for row in 0..num_rows {
            assert_eq!(
                key_share_state::get_by_xres_star_hash(
                    &mut transaction,
                    &[row as u8; 16],
                    &format!("test_backup_network_{}", row),
                )
                .await
                .unwrap()
                .unwrap(),
                key_share_state::ShareState {
                    user_id: "test_user_id".to_string(),
                    rand: vec![0u8; RAND_LENGTH].try_into().unwrap()
                },
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
                &[row as u8; 16],
                &[row as u8; 16],
                &format!("test_backup_network_{}", row),
                "test_user_id",
                &[0u8; RAND_LENGTH],
            )
            .await
            .unwrap();
        }
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        for row in 0..num_rows {
            assert_eq!(
                key_share_state::get_by_xres_star_hash(
                    &mut transaction,
                    &[row as u8; 16],
                    &format!("test_backup_network_{}", row),
                )
                .await
                .unwrap()
                .unwrap(),
                key_share_state::ShareState {
                    user_id: "test_user_id".to_string(),
                    rand: vec![0u8; RAND_LENGTH].try_into().unwrap()
                }
            );
        }
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        for row in 0..num_rows {
            key_share_state::remove_by_xres_star_hash(
                &mut transaction,
                &[row as u8; 16],
                &format!("test_backup_network_{}", row),
            )
            .await
            .unwrap();
        }
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        for row in 0..num_rows {
            assert!(key_share_state::get_by_xres_star_hash(
                &mut transaction,
                &[row as u8; 16],
                &format!("test_backup_network_{}", row),
            )
            .await
            .unwrap()
            .is_none());
        }
        transaction.commit().await.unwrap();
    }
}
