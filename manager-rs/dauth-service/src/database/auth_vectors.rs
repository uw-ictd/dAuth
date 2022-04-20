use sqlx::sqlite::{SqlitePool, SqliteRow};
use sqlx::{Sqlite, Transaction};

use crate::data::error::DauthError;

/// Creates the auth vector table if it does not exist already.
pub async fn init_table(pool: &SqlitePool) -> Result<(), DauthError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS auth_vector_table (
            user_id TEXT NOT NULL,
            seqnum INT NOT NULL,
            xres_star_hash BLOB NOT NULL,
            autn BLOB NOT NULL,
            rand BLOB NOT NULL,
            PRIMARY KEY (user_id, seqnum)
        );",
    )
    .execute(pool)
    .await?;
    Ok(())
}

/* Queries */

/// Inserts a vector with the given data.
/// Returns an error if (id, seqnum) is not unique.
pub async fn add(
    transaction: &mut Transaction<'_, Sqlite>,
    id: &str,
    seqnum: i64,
    xres_star_hash: &[u8],
    autn: &[u8],
    rand: &[u8],
) -> Result<(), DauthError> {
    sqlx::query(
        "INSERT INTO auth_vector_table
        VALUES ($1,$2,$3,$4,$5)",
    )
    .bind(id)
    .bind(seqnum)
    .bind(xres_star_hash)
    .bind(autn)
    .bind(rand)
    .execute(transaction)
    .await?;

    Ok(())
}

/// Returns the first for a given id, sorted by rank (seqnum).
pub async fn get_first(
    transaction: &mut Transaction<'_, Sqlite>,
    id: &str,
) -> Result<SqliteRow, DauthError> {
    Ok(sqlx::query(
        "SELECT * FROM auth_vector_table
        WHERE user_id=$1
        ORDER BY seqnum
        LIMIT 1;",
    )
    .bind(id)
    .fetch_one(transaction)
    .await?)
}

/// Returns the auth vector with the corresponding xres_star_hash.
pub async fn get_by_hash(
    transaction: &mut Transaction<'_, Sqlite>,
    xres_star_hash: &[u8],
) -> Result<SqliteRow, DauthError> {
    Ok(sqlx::query(
        "SELECT * FROM auth_vector_table
        WHERE xres_star_hash=$1
        LIMIT 1;",
    )
    .bind(xres_star_hash)
    .fetch_one(transaction)
    .await?)
}

/// Removes the vector with the (id, seqnum) pair.
pub async fn remove(
    transaction: &mut Transaction<'_, Sqlite>,
    id: &str,
    seqnum: i64,
) -> Result<(), DauthError> {
    sqlx::query(
        "DELETE FROM auth_vector_table
        WHERE (user_id,seqnum)=($1,$2)",
    )
    .bind(id)
    .bind(seqnum)
    .execute(transaction)
    .await?;
    Ok(())
}

/// Removes all vectors belonging to an id.
pub async fn remove_all(
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &str,
) -> Result<(), DauthError> {
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
    use sqlx::{Row, SqlitePool};
    use tempfile::{tempdir, TempDir};

    use auth_vector::constants::{AUTN_LENGTH, RAND_LENGTH, RES_STAR_HASH_LENGTH};

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
                    &[0_u8; RES_STAR_HASH_LENGTH],
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
            &[0_u8; RES_STAR_HASH_LENGTH],
            &[0_u8; AUTN_LENGTH],
            &[0_u8; RAND_LENGTH],
        )
        .await
        .unwrap();

        auth_vectors::add(
            &mut transaction,
            "test_id_1",
            1,
            &[0_u8; RES_STAR_HASH_LENGTH],
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
            &[0_u8; RES_STAR_HASH_LENGTH],
            &[0_u8; AUTN_LENGTH],
            &[0_u8; RAND_LENGTH],
        )
        .await
        .unwrap();

        auth_vectors::add(
            &mut transaction,
            "test_id_1",
            0,
            &[0_u8; RES_STAR_HASH_LENGTH],
            &[0_u8; AUTN_LENGTH],
            &[0_u8; RAND_LENGTH],
        )
        .await
        .unwrap();

        auth_vectors::add(
            &mut transaction,
            "test_id_1",
            1,
            &[0_u8; RES_STAR_HASH_LENGTH],
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

        assert_eq!("test_id_1", res.get_unchecked::<&str, &str>("user_id"));
        assert_eq!(0, res.get_unchecked::<i64, &str>("seqnum"));

        transaction.commit().await.unwrap();
    }

    #[tokio::test]
    async fn test_get_by_hash() {
        let (pool, _dir) = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let good_hash = [0_u8; RES_STAR_HASH_LENGTH];
        let mut bad_hash = [0_u8; RES_STAR_HASH_LENGTH];
        bad_hash[0] = 1;

        auth_vectors::add(
            &mut transaction,
            "test_id_1",
            0,
            &good_hash,
            &[0_u8; AUTN_LENGTH],
            &[0_u8; RAND_LENGTH],
        )
        .await
        .unwrap();

        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();

        assert!(auth_vectors::get_by_hash(&mut transaction, &bad_hash)
            .await.is_err());

        let res = auth_vectors::get_by_hash(&mut transaction, &good_hash)
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
                auth_vectors::add(
                    &mut transaction,
                    &format!("test_id_{}", section),
                    row,
                    &[0_u8; RES_STAR_HASH_LENGTH],
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
                auth_vectors::remove(&mut transaction, &format!("test_id_{}", section), row)
                    .await
                    .unwrap();
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
                    &[0_u8; RES_STAR_HASH_LENGTH],
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
                    &[0_u8; RES_STAR_HASH_LENGTH],
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

                auth_vectors::remove(&mut transaction, &format!("test_id_{}", section), row)
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
