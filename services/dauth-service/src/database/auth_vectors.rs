use sqlx::sqlite::SqlitePool;
use sqlx::{Row, Sqlite, Transaction};

use auth_vector::types::XResHash;

use crate::data::error::DauthError;
use crate::data::vector::AuthVectorRes;
use crate::database::utilities::DauthDataUtilities;

/// Creates the auth vector table if it does not exist already.
#[tracing::instrument(skip(pool), name = "database::auth_vectors")]
pub async fn init_table(pool: &SqlitePool) -> Result<(), DauthError> {
    tracing::info!("Initializing table");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS auth_vector_table (
            user_id TEXT NOT NULL,
            seqnum INT NOT NULL,
            xres_star_hash BLOB NOT NULL,
            xres_hash BLOB NOT NULL,
            autn BLOB NOT NULL,
            rand BLOB NOT NULL,
            sent INTEGER NOT NULL,
            PRIMARY KEY (user_id, seqnum)
        );",
    )
    .execute(pool)
    .await?;
    sqlx::query(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_auth_vector_id_xres_star_hash
        ON auth_vector_table (user_id, xres_star_hash);",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_auth_vector_id_xres_hash
        ON auth_vector_table (user_id, xres_hash);",
    )
    .execute(pool)
    .await?;

    Ok(())
}

/* Queries */

/// Inserts a vector with the given data.
/// Returns an error if (id, seqnum) is not unique.
#[tracing::instrument(skip(transaction), name = "database::auth_vectors")]
pub async fn add(
    transaction: &mut Transaction<'_, Sqlite>,
    id: &str,
    seqnum: i64,
    xres_star_hash: &[u8],
    xres_hash: &XResHash,
    autn: &[u8],
    rand: &[u8],
) -> Result<(), DauthError> {
    tracing::debug!("Adding auth vector");

    sqlx::query(
        "INSERT INTO auth_vector_table
        VALUES ($1,$2,$3,$4,$5,$6,FALSE)",
    )
    .bind(id)
    .bind(seqnum)
    .bind(xres_star_hash)
    .bind(xres_hash.as_slice())
    .bind(autn)
    .bind(rand)
    .execute(transaction)
    .await?;

    Ok(())
}

/// Returns the first for a given id, sorted by rank (seqnum).
#[tracing::instrument(skip(transaction), name = "database::auth_vectors")]
pub async fn get_first(
    transaction: &mut Transaction<'_, Sqlite>,
    id: &str,
) -> Result<AuthVectorRes, DauthError> {
    tracing::debug!("Getting first auth vector");

    Ok(sqlx::query(
        "SELECT * FROM auth_vector_table
        WHERE user_id=$1
        ORDER BY seqnum
        LIMIT 1;",
    )
    .bind(id)
    .fetch_one(transaction)
    .await?
    .to_auth_vector()?)
}

/// Returns the auth vector with the corresponding xres_star_hash.
/// Not currently used.
#[allow(dead_code)]
#[tracing::instrument(skip(transaction), name = "database::auth_vectors")]
pub async fn get_by_xres_star_hash(
    transaction: &mut Transaction<'_, Sqlite>,
    xres_star_hash: &[u8],
) -> Result<AuthVectorRes, DauthError> {
    tracing::debug!("Adding auth vector by xres* hash");

    Ok(sqlx::query(
        "SELECT * FROM auth_vector_table
        WHERE xres_star_hash=$1
        LIMIT 1;",
    )
    .bind(xres_star_hash)
    .fetch_one(transaction)
    .await?
    .to_auth_vector()?)
}

/// Marks the vector with the (user_id, seqnum) pair as having been previously
/// sent.
#[tracing::instrument(skip(transaction), name = "database::auth_vectors")]
pub async fn mark_sent(
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &str,
    seqnum: i64,
) -> Result<(), DauthError> {
    tracing::debug!("Marking auth vector as sent");

    sqlx::query(
        "UPDATE auth_vector_table
        SET sent=TRUE
        WHERE (user_id,seqnum)=($1,$2)",
    )
    .bind(user_id)
    .bind(seqnum)
    .execute(transaction)
    .await?;
    Ok(())
}

/// Removes the vector with the (user_id, xres_star_hash) pair.
#[tracing::instrument(skip(transaction), name = "database::auth_vectors")]
pub async fn remove(
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &str,
    xres_star_hash: &[u8],
) -> Result<i32, DauthError> {
    tracing::debug!("Removing auth vector");

    let presence_count: i32 = sqlx::query(
        "SELECT count(*) as count FROM auth_vector_table
        WHERE (user_id,xres_star_hash)=($1,$2);",
    )
    .bind(user_id)
    .bind(xres_star_hash)
    .fetch_one::<&mut Transaction<'_, Sqlite>>(transaction)
    .await?
    .try_get::<i32, &str>("count")?
    .try_into()
    .unwrap();

    sqlx::query(
        "DELETE FROM auth_vector_table
        WHERE (user_id,xres_star_hash)=($1,$2)",
    )
    .bind(user_id)
    .bind(xres_star_hash)
    .execute::<&mut Transaction<'_, Sqlite>>(transaction)
    .await?;
    Ok(presence_count)
}

/// Removes all vectors belonging to an id.
#[tracing::instrument(skip(transaction), name = "database::auth_vectors")]
pub async fn remove_all(
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &str,
) -> Result<(), DauthError> {
    tracing::debug!("Removing all auth vectors for user");

    sqlx::query(
        "DELETE FROM auth_vector_table
        WHERE user_id=$1",
    )
    .bind(user_id)
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

    use auth_vector::types::{AUTN_LENGTH, RAND_LENGTH, XRES_HASH_LENGTH, XRES_STAR_HASH_LENGTH};

    use crate::database::{auth_vectors, general};

    fn gen_name() -> String {
        let s: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();

        format!("sqlite_{}.db", s)
    }

    async fn init() -> (SqlitePool, TempDir) {
        let dir = tempdir().unwrap();
        let path = String::from(dir.path().join(gen_name()).to_str().unwrap());
        println!("Building temporary db: {}", path);

        let pool = general::build_pool(&path).await.unwrap();
        auth_vectors::init_table(&pool).await.unwrap();

        (pool, dir)
    }

    /// Test that db and table creation will work
    #[tokio::test]
    async fn test_db_init() {
        init().await;
    }

    /// Test that db fails on non-existent result
    #[tokio::test]
    #[should_panic]
    async fn test_empty_get() {
        let (pool, _dir) = init().await;

        let mut transaction = pool.begin().await.unwrap();

        auth_vectors::get_first(&mut transaction, "test_id_0")
            .await
            .unwrap();

        transaction.commit().await.unwrap();
    }

    /// Test that db can insert
    #[tokio::test]
    async fn test_get() {
        let (pool, _dir) = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                auth_vectors::add(
                    &mut transaction,
                    &format!("test_id_{}", section),
                    row,
                    &[row as u8; XRES_STAR_HASH_LENGTH],
                    &[row as u8; XRES_HASH_LENGTH],
                    &[0_u8; AUTN_LENGTH],
                    &[0_u8; RAND_LENGTH],
                )
                .await
                .unwrap();
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

        auth_vectors::add(
            &mut transaction,
            "test_id_1",
            1,
            &[0_u8; XRES_STAR_HASH_LENGTH],
            &[0_u8; XRES_HASH_LENGTH],
            &[0_u8; AUTN_LENGTH],
            &[0_u8; RAND_LENGTH],
        )
        .await
        .unwrap();

        auth_vectors::add(
            &mut transaction,
            "test_id_1",
            1,
            &[0_u8; XRES_STAR_HASH_LENGTH],
            &[0_u8; XRES_HASH_LENGTH],
            &[0_u8; AUTN_LENGTH],
            &[0_u8; RAND_LENGTH],
        )
        .await
        .unwrap();

        transaction.commit().await.unwrap();
    }

    /// Test that db can delete after inserts
    #[tokio::test]
    async fn test_get_first() {
        let (pool, _dir) = init().await;

        let mut transaction = pool.begin().await.unwrap();

        auth_vectors::add(
            &mut transaction,
            "test_id_1",
            2,
            &[2_u8; XRES_STAR_HASH_LENGTH],
            &[2_u8; XRES_HASH_LENGTH],
            &[0_u8; AUTN_LENGTH],
            &[0_u8; RAND_LENGTH],
        )
        .await
        .unwrap();

        auth_vectors::add(
            &mut transaction,
            "test_id_1",
            0,
            &[0_u8; XRES_STAR_HASH_LENGTH],
            &[0_u8; XRES_HASH_LENGTH],
            &[0_u8; AUTN_LENGTH],
            &[0_u8; RAND_LENGTH],
        )
        .await
        .unwrap();

        auth_vectors::add(
            &mut transaction,
            "test_id_1",
            1,
            &[1_u8; XRES_STAR_HASH_LENGTH],
            &[1_u8; XRES_HASH_LENGTH],
            &[0_u8; AUTN_LENGTH],
            &[0_u8; RAND_LENGTH],
        )
        .await
        .unwrap();

        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();

        let res = auth_vectors::get_first(&mut transaction, "test_id_1")
            .await
            .unwrap();

        assert_eq!("test_id_1", res.user_id);
        assert_eq!(0, res.seqnum);

        transaction.commit().await.unwrap();
    }

    #[tokio::test]
    async fn test_get_by_hash() {
        let (pool, _dir) = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let good_hash = [0_u8; XRES_STAR_HASH_LENGTH];
        let mut bad_hash = [0_u8; XRES_STAR_HASH_LENGTH];
        bad_hash[0] = 1;

        auth_vectors::add(
            &mut transaction,
            "test_id_1",
            0,
            &good_hash,
            &good_hash,
            &[0_u8; AUTN_LENGTH],
            &[0_u8; RAND_LENGTH],
        )
        .await
        .unwrap();

        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();

        assert!(
            auth_vectors::get_by_xres_star_hash(&mut transaction, &bad_hash)
                .await
                .is_err()
        );

        let res = auth_vectors::get_by_xres_star_hash(&mut transaction, &good_hash)
            .await
            .unwrap();

        assert_eq!("test_id_1", res.user_id);
        assert_eq!(0, res.seqnum);

        transaction.commit().await.unwrap();
    }

    /// Test that db can delete after inserts
    #[tokio::test]
    async fn test_remove() {
        let (pool, _dir) = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                auth_vectors::add(
                    &mut transaction,
                    &format!("test_id_{}", section),
                    row,
                    &[row as u8; XRES_STAR_HASH_LENGTH],
                    &[row as u8; XRES_HASH_LENGTH],
                    &[0_u8; AUTN_LENGTH],
                    &[0_u8; RAND_LENGTH],
                )
                .await
                .unwrap();
            }
        }

        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                let count = auth_vectors::remove(
                    &mut transaction,
                    &format!("test_id_{}", section),
                    &[row as u8; XRES_STAR_HASH_LENGTH],
                )
                .await
                .unwrap();
                assert_eq!(count, 1);
            }
        }

        transaction.commit().await.unwrap();
    }

    /// Test that db can delete after inserts
    #[tokio::test]
    async fn test_remove_all() {
        let (pool, _dir) = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                auth_vectors::add(
                    &mut transaction,
                    &format!("test_id_{}", section),
                    row,
                    &[row as u8; XRES_STAR_HASH_LENGTH],
                    &[row as u8; XRES_HASH_LENGTH],
                    &[0_u8; AUTN_LENGTH],
                    &[0_u8; RAND_LENGTH],
                )
                .await
                .unwrap();
            }
        }

        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();

        let num_sections = 10;

        for section in 0..num_sections {
            auth_vectors::remove_all(&mut transaction, &format!("test_id_{}", section))
                .await
                .unwrap();
        }

        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();

        let num_sections = 10;

        for section in 0..num_sections {
            assert!(
                auth_vectors::get_first(&mut transaction, &format!("test_id_{}", section))
                    .await
                    .is_err()
            )
        }

        transaction.commit().await.unwrap();
    }

    /// Test that db can delete after inserts
    #[tokio::test]
    async fn test_get_first_with_delete() {
        let (pool, _dir) = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                auth_vectors::add(
                    &mut transaction,
                    &format!("test_id_{}", section),
                    row,
                    &[row as u8; XRES_STAR_HASH_LENGTH],
                    &[row as u8; XRES_HASH_LENGTH],
                    &[0_u8; AUTN_LENGTH],
                    &[0_u8; RAND_LENGTH],
                )
                .await
                .unwrap();
            }
        }

        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                let res =
                    auth_vectors::get_first(&mut transaction, &format!("test_id_{}", section))
                        .await
                        .unwrap();

                auth_vectors::remove(
                    &mut transaction,
                    &format!("test_id_{}", section),
                    &[row as u8; XRES_STAR_HASH_LENGTH],
                )
                .await
                .unwrap();

                assert_eq!(format!("test_id_{}", section), res.user_id);
                assert_eq!(row, res.seqnum);
            }
        }

        transaction.commit().await.unwrap();
    }
}
