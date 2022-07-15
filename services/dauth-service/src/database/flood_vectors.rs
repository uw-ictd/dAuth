use sqlx::sqlite::{SqlitePool, SqliteRow};
use sqlx::Error as SqlxError;
use sqlx::{Row, Sqlite, Transaction};

use auth_vector::types::XResHash;

use crate::data::error::DauthError;

/// Creates the flood vector table if it does not exist already.
/// Meant to be used before the auth vector table.
pub async fn init_table(pool: &SqlitePool) -> Result<(), DauthError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS flood_vector_table (
            rank INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id TEXT NOT NULL,
            seqnum INT NOT NULL,
            xres_star_hash BLOB NOT NULL,
            xres_hash BLOB NOT NULL,
            autn BLOB NOT NULL,
            rand BLOB NOT NULL,
            sent INTEGER NOT NULL,
            UNIQUE(user_id, seqnum)
        );",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_flood_vector_id_xres_star_hash
        ON flood_vector_table (user_id, xres_star_hash);",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_flood_vector_id_xres_hash
        ON flood_vector_table (user_id, xres_hash);",
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Inserts a vector with the given data.
/// Returns an error if (id, seqnum) is not unique.
pub async fn add(
    transaction: &mut Transaction<'_, Sqlite>,
    id: &str,
    seqnum: i64,
    xres_star_hash: &[u8],
    xres_hash: &XResHash,
    autn: &[u8],
    rand: &[u8],
) -> Result<(), DauthError> {
    sqlx::query(
        "INSERT INTO flood_vector_table
        (user_id,seqnum,xres_star_hash,xres_hash,autn,rand, sent)
        VALUES ($1,$2,$3,$4,$5, $6, FALSE)",
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

/// Check first for higher priority flood vectors
/// Generally, it is normal for this to return Ok(None)
pub async fn get_first(
    transaction: &mut Transaction<'_, Sqlite>,
    id: &str,
) -> Result<Option<SqliteRow>, SqlxError> {
    let res = sqlx::query(
        "SELECT * FROM flood_vector_table
        WHERE user_id=$1
        ORDER BY rank
        LIMIT 1;",
    )
    .bind(id)
    .fetch_one(transaction)
    .await;

    match res {
        Err(SqlxError::RowNotFound) => Ok(None),
        _ => Ok(Some(res?)),
    }
}

/// Returns the auth vector with the corresponding xres_star_hash.
pub async fn get_by_xres_star_hash(
    transaction: &mut Transaction<'_, Sqlite>,
    xres_star_hash: &[u8],
) -> Result<SqliteRow, DauthError> {
    Ok(sqlx::query(
        "SELECT * FROM flood_vector_table
        WHERE xres_star_hash=$1
        LIMIT 1;",
    )
    .bind(xres_star_hash)
    .fetch_one(transaction)
    .await?)
}

/// Marks the vector with the (user_id, seqnum) pair as having been previously
/// sent.
pub async fn mark_sent(
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &str,
    seqnum: i64,
) -> Result<(), DauthError> {
    sqlx::query(
        "UPDATE flood_vector_table
        SET sent=TRUE
        WHERE (user_id,seqnum)=($1,$2)",
    )
    .bind(user_id)
    .bind(seqnum)
    .execute(transaction)
    .await?;
    Ok(())
}

/// Removes the vector with the (id, seqnum) pair.
pub async fn remove(
    transaction: &mut Transaction<'_, Sqlite>,
    id: &str,
    xres_star_hash: &[u8],
) -> Result<i32, DauthError> {
    let presence_count: i32 = sqlx::query(
        "SELECT count(*) as count FROM flood_vector_table
        WHERE (user_id,xres_star_hash)=($1,$2);",
    )
    .bind(id)
    .bind(xres_star_hash)
    .fetch_one::<&mut Transaction<'_, Sqlite>>(transaction)
    .await?
    .try_get::<i32, &str>("count")?
    .try_into()
    .unwrap();

    sqlx::query(
        "DELETE FROM flood_vector_table
        WHERE (user_id,xres_star_hash)=($1,$2)",
    )
    .bind(id)
    .bind(xres_star_hash)
    .execute(transaction)
    .await?;
    Ok(presence_count)
}

/// Removes all vectors belonging to an id.
pub async fn remove_all(
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &str,
) -> Result<(), DauthError> {
    sqlx::query(
        "DELETE FROM flood_vector_table
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
    use sqlx::{Row, SqlitePool};
    use tempfile::{tempdir, TempDir};

    use auth_vector::types::{AUTN_LENGTH, RAND_LENGTH, XRES_STAR_HASH_LENGTH, XRES_HASH_LENGTH};

    use crate::database::{flood_vectors, general};

    fn gen_name() -> String {
        let s: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();

        format!("sqlite_{}.db", s)
    }

    async fn init() -> (SqlitePool, TempDir) {
        let dir = tempdir().unwrap();
        let path = String::from(dir.path().join(gen_name()).to_str().unwrap());
        println!("Building temporary db: {}", path);

        let pool = general::build_pool(&path).await.unwrap();
        flood_vectors::init_table(&pool).await.unwrap();

        (pool, dir)
    }

    /// Test that db and table creation will work
    #[tokio::test]
    async fn test_db_init() {
        init().await;
    }

    /// Test that db fails on non-existent result
    #[tokio::test]
    async fn test_empty_get() {
        let (pool, _dir) = init().await;

        let mut transaction = pool.begin().await.unwrap();

        assert!(flood_vectors::get_first(&mut transaction, "test_id_0")
            .await
            .unwrap()
            .is_none());

        transaction.commit().await.unwrap();
    }

    /// Test that db can insert
    #[tokio::test]
    async fn test_add() {
        let (pool, _dir) = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                flood_vectors::add(
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

        flood_vectors::add(
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

        flood_vectors::add(
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

        flood_vectors::add(
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

        flood_vectors::add(
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

        flood_vectors::add(
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

        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();

        let res = flood_vectors::get_first(&mut transaction, "test_id_1")
            .await
            .unwrap()
            .unwrap();

        assert_eq!("test_id_1", res.get_unchecked::<&str, &str>("user_id"));
        assert_eq!(1, res.get_unchecked::<i64, &str>("seqnum"));

        transaction.commit().await.unwrap();
    }

    #[tokio::test]
    async fn test_get_by_hash() {
        let (pool, _dir) = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let good_hash = [0_u8; XRES_STAR_HASH_LENGTH];
        let mut bad_hash = [0_u8; XRES_STAR_HASH_LENGTH];
        bad_hash[0] = 1;

        flood_vectors::add(
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

        assert!(flood_vectors::get_by_xres_star_hash(&mut transaction, &bad_hash)
            .await
            .is_err());

        let res = flood_vectors::get_by_xres_star_hash(&mut transaction, &good_hash)
            .await
            .unwrap();

        assert_eq!("test_id_1", res.get_unchecked::<&str, &str>("user_id"));
        assert_eq!(0, res.get_unchecked::<i64, &str>("seqnum"));

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
                flood_vectors::add(
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
                let count = flood_vectors::remove(
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
                flood_vectors::add(
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
            flood_vectors::remove_all(&mut transaction, &format!("test_id_{}", section))
                .await
                .unwrap();
        }

        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();

        let num_sections = 10;

        for section in 0..num_sections {
            assert!(
                flood_vectors::get_first(&mut transaction, &format!("test_id_{}", section))
                    .await
                    .unwrap()
                    .is_none()
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
                flood_vectors::add(
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
                    flood_vectors::get_first(&mut transaction, &format!("test_id_{}", section))
                        .await
                        .unwrap()
                        .unwrap();

                flood_vectors::remove(
                    &mut transaction,
                    &format!("test_id_{}", section),
                    &[row as u8; XRES_STAR_HASH_LENGTH],
                )
                .await
                .unwrap();

                assert_eq!(
                    &format!("test_id_{}", section),
                    res.get_unchecked::<&str, &str>("user_id")
                );
                assert_eq!(row, res.get_unchecked::<i64, &str>("seqnum"));
            }
        }

        transaction.commit().await.unwrap();
    }
}
