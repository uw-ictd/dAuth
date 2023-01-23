use sqlx::sqlite::SqlitePool;
use sqlx::{Row, Sqlite, Transaction};

use crate::data::error::DauthError;
use crate::data::keys;
use auth_vector::types::{XResHash, XResStarHash};

#[derive(sqlx::FromRow)]
struct KeyShareRow {
    pub xres_star_hash: Vec<u8>,
    pub xres_hash: Vec<u8>,
    pub user_id: String,
    pub kseaf_share: Vec<u8>,
    pub kasme_share: Vec<u8>,
}

/// Creates the kseaf table if it does not exist already.
pub async fn init_table(pool: &SqlitePool) -> Result<(), DauthError> {
    sqlx::query(
        &"CREATE TABLE IF NOT EXISTS key_share_table (
            xres_star_hash BLOB PRIMARY KEY,
            xres_hash BLOB NOT NULL,
            user_id TEXT NOT NULL,
            kseaf_share BLOB NOT NULL,
            kasme_share BLOB NOT NULL
        );",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_key_share_xres_hash
        ON key_share_table (xres_hash);",
    )
    .execute(pool)
    .await?;

    Ok(())
}

/* Queries */

/// Inserts a key share
pub async fn add(
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &str,
    key_share: &keys::CombinedKeyShare,
) -> Result<(), DauthError> {
    sqlx::query(
        "INSERT INTO key_share_table
        (xres_star_hash, xres_hash, user_id, kseaf_share, kasme_share)
        VALUES ($1,$2,$3,$4,$5)",
    )
    .bind(key_share.xres_star_hash.as_slice())
    .bind(key_share.xres_hash.as_slice())
    .bind(user_id)
    .bind(key_share.kseaf_share.as_slice())
    .bind(key_share.kasme_share.as_slice())
    .execute(transaction)
    .await?;

    Ok(())
}

/// Returns a key share if found.
pub async fn get_from_xres_star_hash(
    transaction: &mut Transaction<'_, Sqlite>,
    xres_star_hash: &XResStarHash,
) -> Result<keys::CombinedKeyShare, DauthError> {
    tracing::debug!(?xres_star_hash, "querying for key share by xres_star_hash");
    let row: KeyShareRow = sqlx::query_as(
        "SELECT * FROM key_share_table
        WHERE xres_star_hash=$1;",
    )
    .bind(xres_star_hash.as_slice())
    .fetch_one(transaction)
    .await?;

    Ok(keys::CombinedKeyShare {
        kasme_share: row.kasme_share.try_into()?,
        kseaf_share: row.kseaf_share.try_into()?,
        xres_hash: row
            .xres_hash
            .try_into()
            .or(Err(DauthError::DataError("Xres".to_string())))?,
        xres_star_hash: row
            .xres_star_hash
            .try_into()
            .or(Err(DauthError::DataError("xres_star".to_string())))?,
    })
}

/// Returns a key share if found.
pub async fn get_from_xres_hash(
    transaction: &mut Transaction<'_, Sqlite>,
    xres_hash: &XResHash,
) -> Result<keys::CombinedKeyShare, DauthError> {
    tracing::debug!(?xres_hash, "querying for key share by xres_hash");
    let row: KeyShareRow = sqlx::query_as(
        "SELECT * FROM key_share_table
        WHERE xres_hash=$1;",
    )
    .bind(xres_hash.as_slice())
    .fetch_one(transaction)
    .await?;

    Ok(keys::CombinedKeyShare {
        kasme_share: row.kasme_share.try_into()?,
        kseaf_share: row.kseaf_share.try_into()?,
        xres_hash: row
            .xres_hash
            .try_into()
            .or(Err(DauthError::DataError("Xres".to_string())))?,
        xres_star_hash: row
            .xres_star_hash
            .try_into()
            .or(Err(DauthError::DataError("xres_star".to_string())))?,
    })
}

/// Returns the user id that the key share/vector belongs to.
pub async fn get_user_id(
    transaction: &mut Transaction<'_, Sqlite>,
    xres_star_hash: &[u8],
) -> Result<String, DauthError> {
    tracing::debug!(
        ?xres_star_hash,
        "querying for key share user_id by xres_star_hash"
    );
    Ok(sqlx::query(
        "SELECT * FROM key_share_table
        WHERE xres_star_hash=$1;",
    )
    .bind(xres_star_hash)
    .fetch_one(transaction)
    .await?
    .try_get::<String, &str>("user_id")?)
}

/// Deletes a key share if found.
pub async fn remove(
    transaction: &mut Transaction<'_, Sqlite>,
    xres_star_hash: &[u8],
) -> Result<(), DauthError> {
    sqlx::query(
        "DELETE FROM key_share_table
        WHERE xres_star_hash=$1",
    )
    .bind(xres_star_hash)
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

    use auth_vector::types::{
        KSEAF_LENGTH, XRES_HASH_LENGTH, XRES_STAR_HASH_LENGTH, XRES_STAR_LENGTH,
    };

    use crate::data::keys::CombinedKeyShare;
    use crate::database::{general, key_shares};

    fn gen_name() -> String {
        let s: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();

        format!("sqlite_{}.db", s)
    }

    async fn init() -> (SqlitePool, TempDir) {
        let dir = tempdir().unwrap();
        let path = String::from(dir.path().join(gen_name()).to_str().unwrap());
        println!("Building temporary db: {}", path);

        let pool = general::build_pool(&path).await.unwrap();
        key_shares::init_table(&pool).await.unwrap();

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
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                let combined_share = CombinedKeyShare {
                    xres_star_hash: [section * num_rows + row; XRES_STAR_HASH_LENGTH],
                    xres_hash: [section * num_rows + row; XRES_HASH_LENGTH],
                    kseaf_share: vec![section * num_rows + row; KSEAF_LENGTH + 1]
                        .try_into()
                        .unwrap(),
                    kasme_share: vec![section * num_rows + row; KSEAF_LENGTH + 1]
                        .try_into()
                        .unwrap(),
                };
                key_shares::add(&mut transaction, "test_user_id", &combined_share)
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
                let combined_share = CombinedKeyShare {
                    xres_star_hash: [section * num_rows + row; XRES_STAR_HASH_LENGTH],
                    xres_hash: [section * num_rows + row; XRES_HASH_LENGTH],
                    kseaf_share: vec![section * num_rows + row; KSEAF_LENGTH + 1]
                        .try_into()
                        .unwrap(),
                    kasme_share: vec![section * num_rows + row; KSEAF_LENGTH + 1]
                        .try_into()
                        .unwrap(),
                };
                key_shares::add(&mut transaction, "test_user_id", &combined_share)
                    .await
                    .unwrap();
            }
        }

        transaction.commit().await.unwrap();
        let mut transaction = pool.begin().await.unwrap();

        for section in 0..num_sections {
            for row in 0..num_rows {
                let res = key_shares::get_from_xres_star_hash(
                    &mut transaction,
                    &[section * num_rows + row; XRES_STAR_HASH_LENGTH],
                )
                .await
                .unwrap();

                assert_eq!(
                    &[section * num_rows + row; KSEAF_LENGTH + 1],
                    res.kseaf_share.as_slice()
                );

                assert_eq!(
                    key_shares::get_user_id(
                        &mut transaction,
                        &[section * num_rows + row; XRES_STAR_HASH_LENGTH],
                    )
                    .await
                    .unwrap(),
                    "test_user_id"
                );
            }
        }
        transaction.commit().await.unwrap();
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
                let combined_share = CombinedKeyShare {
                    xres_star_hash: [section * num_rows + row; XRES_STAR_HASH_LENGTH],
                    xres_hash: [section * num_rows + row; XRES_HASH_LENGTH],
                    kseaf_share: vec![section * num_rows + row; KSEAF_LENGTH + 1]
                        .try_into()
                        .unwrap(),
                    kasme_share: vec![section * num_rows + row; KSEAF_LENGTH + 1]
                        .try_into()
                        .unwrap(),
                };
                key_shares::add(&mut transaction, "test_user_id", &combined_share)
                    .await
                    .unwrap();
            }
        }

        transaction.commit().await.unwrap();
        let mut transaction = pool.begin().await.unwrap();

        for section in 0..num_sections {
            for row in 0..num_rows {
                key_shares::remove(
                    &mut transaction,
                    &[section * num_rows + row; XRES_STAR_LENGTH],
                )
                .await
                .unwrap();
            }
        }

        transaction.commit().await.unwrap();
        let mut transaction = pool.begin().await.unwrap();

        for section in 0..num_sections {
            for row in 0..num_rows {
                // should have been deleted
                assert!(key_shares::get_from_xres_star_hash(
                    &mut transaction,
                    &[section * num_rows + row; XRES_STAR_LENGTH],
                )
                .await
                .is_err());
            }
        }
        transaction.commit().await.unwrap();
    }

    /// Test that gets work before a delete
    #[tokio::test]
    async fn test_get_with_delete() {
        let (pool, _dir) = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                let combined_share = CombinedKeyShare {
                    xres_star_hash: [section * num_rows + row; XRES_STAR_HASH_LENGTH],
                    xres_hash: [section * num_rows + row; XRES_HASH_LENGTH],
                    kseaf_share: vec![section * num_rows + row; KSEAF_LENGTH + 1]
                        .try_into()
                        .unwrap(),
                    kasme_share: vec![section * num_rows + row; KSEAF_LENGTH + 1]
                        .try_into()
                        .unwrap(),
                };
                key_shares::add(&mut transaction, "test_user_id", &combined_share)
                    .await
                    .unwrap();
            }
        }

        transaction.commit().await.unwrap();
        let mut transaction = pool.begin().await.unwrap();

        for section in 0..num_sections {
            for row in 0..num_rows {
                let res = key_shares::get_from_xres_star_hash(
                    &mut transaction,
                    &[section * num_rows + row; XRES_STAR_LENGTH],
                )
                .await
                .unwrap();

                assert_eq!(
                    &[section * num_rows + row; KSEAF_LENGTH + 1],
                    res.kseaf_share.as_slice()
                );

                key_shares::remove(&mut transaction, res.xres_star_hash.as_slice())
                    .await
                    .unwrap();
            }
        }

        transaction.commit().await.unwrap();
        let mut transaction = pool.begin().await.unwrap();

        for section in 0..num_sections {
            for row in 0..num_rows {
                // should have been deleted
                assert!(key_shares::get_from_xres_star_hash(
                    &mut transaction,
                    &[section * num_rows + row; XRES_STAR_LENGTH],
                )
                .await
                .is_err());
            }
        }
        transaction.commit().await.unwrap();
    }

    /// Test that duplicate inserts cause an error
    #[tokio::test]
    #[should_panic]
    async fn test_add_dupicate_fail() {
        let (pool, _dir) = init().await;
        let mut transaction = pool.begin().await.unwrap();
        let combined_share = CombinedKeyShare {
            xres_star_hash: [1_u8; XRES_STAR_HASH_LENGTH],
            xres_hash: [1_u8; XRES_HASH_LENGTH],
            kseaf_share: vec![1_u8; KSEAF_LENGTH + 1].try_into().unwrap(),
            kasme_share: vec![1_u8; KSEAF_LENGTH + 1].try_into().unwrap(),
        };
        key_shares::add(&mut transaction, "test_user_id", &combined_share)
            .await
            .unwrap();

        key_shares::add(&mut transaction, "test_user_id", &combined_share)
            .await
            .unwrap();

        transaction.commit().await.unwrap();
    }
}
