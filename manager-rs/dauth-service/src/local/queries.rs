use auth_vector::types::Id;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions, SqliteRow};
use sqlx::{Sqlite, Transaction};

use crate::data::error::DauthError;

/// Constructs the sqlite pool for running queries.
pub async fn build_pool(database_path: &str) -> Result<SqlitePool, DauthError> {
    Ok(SqlitePoolOptions::new()
        .max_connections(10)
        .connect_with(
            SqliteConnectOptions::new()
                .create_if_missing(true)
                .filename(database_path),
        )
        .await?)
}

/// Creates the auth vector table if it does not exist already.
pub async fn init_auth_vector_table(pool: &SqlitePool) -> Result<(), DauthError> {
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

/// Creates the kseaf table if it does not exist already.
pub async fn init_kseaf_table(pool: &SqlitePool) -> Result<(), DauthError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS kseaf_table (
            kseaf_uuid BLOB PRIMARY KEY,
            kseaf_data BLOB NOT NULL
        );",
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// Creates the auth vector table if it does not exist already.
pub async fn init_user_info_vector_table(pool: &SqlitePool) -> Result<(), DauthError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user_info_table (
            user_info_id TEST PRIMARY KEY,
            user_info_k BLOB NOT NULL,
            user_info_opc BLOB NOT NULL,
            user_info_sqn_max BLOB NOT NULL
        );",
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// Returns the first for a given id, sorted by rank (seqnum).
pub async fn get_first_vector(
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

/// Inserts a vector with the given data.
/// Returns an error if (id, seqnum) is not unique.
pub async fn insert_vector(
    transaction: &mut Transaction<'_, Sqlite>,
    id: &str,
    seqnum: i64,
    xres: &[u8],
    autn: &[u8],
    rand: &[u8],
) -> Result<(), DauthError> {
    sqlx::query(
        "INSERT INTO auth_vector_table
        VALUES ($1,$2,$3,$4,$5)",
    )
    .bind(id)
    .bind(seqnum)
    .bind(xres)
    .bind(autn)
    .bind(rand)
    .execute(transaction)
    .await?;

    Ok(())
}

/// Removes the vector with the (id, seqnum) pair.
pub async fn remove_vector(
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

/// Inserts a kseaf with a given uuid and value.
pub async fn insert_kseaf(
    transaction: &mut Transaction<'_, Sqlite>,
    uuid: &[u8],
    value: &[u8],
) -> Result<(), DauthError> {
    sqlx::query(
        "INSERT INTO kseaf_table
        VALUES ($1,$2)",
    )
    .bind(uuid)
    .bind(value)
    .execute(transaction)
    .await?;

    Ok(())
}

/// Deletes a kseaf vaule if found.
pub async fn delete_kseaf(
    transaction: &mut Transaction<'_, Sqlite>,
    uuid: &[u8],
) -> Result<(), DauthError> {
    sqlx::query(
        "DELETE FROM kseaf_table
        WHERE kseaf_uuid=$1",
    )
    .bind(uuid)
    .execute(transaction)
    .await?;

    Ok(())
}

/// Returns a kseaf value if found.
pub async fn get_kseaf(
    transaction: &mut Transaction<'_, Sqlite>,
    uuid: &[u8],
) -> Result<SqliteRow, DauthError> {
    Ok(sqlx::query(
        "SELECT * FROM kseaf_table
        WHERE kseaf_uuid=$1;",
    )
    .bind(uuid)
    .fetch_one(transaction)
    .await?)
}

/// Insert user info and replace if exists.
pub async fn user_info_upsert(
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &Id,
    k: &[u8],
    opc: &[u8],
    sqn_max: &[u8],
) -> Result<(), DauthError> {
    sqlx::query(
        "REPLACE INTO user_info_table
        VALUES ($1,$2,$3,$4);",
    )
    .bind(user_id)
    .bind(k)
    .bind(opc)
    .bind(sqn_max)
    .execute(transaction)
    .await?;

    Ok(())
}

/// Get user info if exists.
pub async fn user_info_get(
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &Id,
) -> Result<SqliteRow, DauthError> {
    Ok(sqlx::query(
        "SELECT * FROM user_info_table
        WHERE user_info_id=$1;",
    )
    .bind(user_id)
    .fetch_one(transaction)
    .await?)
}

/// Remove user info if exists.
pub async fn user_info_remove(
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &Id,
) -> Result<(), DauthError> {
    sqlx::query(
        "DELETE FROM user_info_table
        WHERE user_info_id=$1",
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
    use tempfile::tempdir;

    use auth_vector::constants::{
        AUTN_LENGTH, KSEAF_LENGTH, K_LENGTH, OPC_LENGTH, RAND_LENGTH, RES_STAR_HASH_LENGTH,
        RES_STAR_LENGTH, SQN_LENGTH,
    };

    use crate::local::queries;

    fn gen_name() -> String {
        let s: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();

        format!("sqlite_{}.db", s)
    }

    async fn init() -> SqlitePool {
        let dir = tempdir().unwrap();
        let path = String::from(dir.path().join(gen_name()).to_str().unwrap());
        println!("Building temporary db: {}", path);

        let pool = queries::build_pool(&path).await.unwrap();
        queries::init_auth_vector_table(&pool).await.unwrap();
        queries::init_kseaf_table(&pool).await.unwrap();
        queries::init_user_info_vector_table(&pool).await.unwrap();

        pool
    }

    /// Test that db and table creation will work
    #[tokio::test]
    async fn test_db_init() {
        init().await;
    }

    /// Test that db can insert
    #[tokio::test]
    async fn test_av_insert() {
        let pool = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                queries::insert_vector(
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
    async fn test_av_insert_dupicate_fail() {
        let pool = init().await;
        let mut transaction = pool.begin().await.unwrap();

        queries::insert_vector(
            &mut transaction,
            "test_id_1",
            1,
            &[0_u8; RES_STAR_HASH_LENGTH],
            &[0_u8; AUTN_LENGTH],
            &[0_u8; RAND_LENGTH],
        )
        .await
        .unwrap();

        queries::insert_vector(
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
    async fn test_av_get_first() {
        let pool = init().await;

        let mut transaction = pool.begin().await.unwrap();

        queries::insert_vector(
            &mut transaction,
            "test_id_1",
            2,
            &[0_u8; RES_STAR_HASH_LENGTH],
            &[0_u8; AUTN_LENGTH],
            &[0_u8; RAND_LENGTH],
        )
        .await
        .unwrap();

        queries::insert_vector(
            &mut transaction,
            "test_id_1",
            0,
            &[0_u8; RES_STAR_HASH_LENGTH],
            &[0_u8; AUTN_LENGTH],
            &[0_u8; RAND_LENGTH],
        )
        .await
        .unwrap();

        queries::insert_vector(
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

        let res = queries::get_first_vector(&mut transaction, "test_id_1")
            .await
            .unwrap();

        assert_eq!("test_id_1", res.get_unchecked::<&str, &str>("user_id"));
        assert_eq!(0, res.get_unchecked::<i64, &str>("seqnum"));

        transaction.commit().await.unwrap();
    }

    /// Test that db can delete after inserts
    #[tokio::test]
    async fn test_av_delete() {
        let pool = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                queries::insert_vector(
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
                queries::remove_vector(&mut transaction, &format!("test_id_{}", section), row)
                    .await
                    .unwrap();
            }
        }

        transaction.commit().await.unwrap();
    }

    /// Test that db can delete after inserts
    #[tokio::test]
    async fn test_av_get_first_with_delete() {
        let pool = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                queries::insert_vector(
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
                    queries::get_first_vector(&mut transaction, &format!("test_id_{}", section))
                        .await
                        .unwrap();

                queries::remove_vector(&mut transaction, &format!("test_id_{}", section), row)
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

    /// Test that insert works
    #[tokio::test]
    async fn test_kseaf_insert() {
        let pool = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                queries::insert_kseaf(
                    &mut transaction,
                    &[section * num_rows + row; RES_STAR_LENGTH],
                    &[section * num_rows + row; KSEAF_LENGTH],
                )
                .await
                .unwrap();
            }
        }
        transaction.commit().await.unwrap();
    }

    /// Tests that get works
    #[tokio::test]
    async fn test_kseaf_get() {
        let pool = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                queries::insert_kseaf(
                    &mut transaction,
                    &[section * num_rows + row; RES_STAR_LENGTH],
                    &[section * num_rows + row; KSEAF_LENGTH],
                )
                .await
                .unwrap();
            }
        }

        transaction.commit().await.unwrap();
        let mut transaction = pool.begin().await.unwrap();

        for section in 0..num_sections {
            for row in 0..num_rows {
                let res = queries::get_kseaf(
                    &mut transaction,
                    &[section * num_rows + row; RES_STAR_LENGTH],
                )
                .await
                .unwrap();

                assert_eq!(
                    &[section * num_rows + row; KSEAF_LENGTH],
                    res.get_unchecked::<&[u8], &str>("kseaf_data")
                );
            }
        }
        transaction.commit().await.unwrap();
    }

    /// Test that deletes work
    #[tokio::test]
    async fn test_kseaf_delete() {
        let pool = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                queries::insert_kseaf(
                    &mut transaction,
                    &[section * num_rows + row; RES_STAR_LENGTH],
                    &[section * num_rows + row; KSEAF_LENGTH],
                )
                .await
                .unwrap();
            }
        }

        transaction.commit().await.unwrap();
        let mut transaction = pool.begin().await.unwrap();

        for section in 0..num_sections {
            for row in 0..num_rows {
                queries::delete_kseaf(
                    &mut transaction,
                    &[section * num_rows + row; RES_STAR_LENGTH],
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
                assert!(queries::get_kseaf(
                    &mut transaction,
                    &[section * num_rows + row; RES_STAR_LENGTH],
                )
                .await
                .is_err());
            }
        }
        transaction.commit().await.unwrap();
    }

    /// Test that gets work before a delete
    #[tokio::test]
    async fn test_kseaf_get_with_delete() {
        let pool = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                queries::insert_kseaf(
                    &mut transaction,
                    &[section * num_rows + row; RES_STAR_LENGTH],
                    &[section * num_rows + row; KSEAF_LENGTH],
                )
                .await
                .unwrap();
            }
        }

        transaction.commit().await.unwrap();
        let mut transaction = pool.begin().await.unwrap();

        for section in 0..num_sections {
            for row in 0..num_rows {
                let res = queries::get_kseaf(
                    &mut transaction,
                    &[section * num_rows + row; RES_STAR_LENGTH],
                )
                .await
                .unwrap();

                assert_eq!(
                    &[section * num_rows + row; KSEAF_LENGTH],
                    res.get_unchecked::<&[u8], &str>("kseaf_data")
                );

                queries::delete_kseaf(
                    &mut transaction,
                    res.get_unchecked::<&[u8], &str>("kseaf_uuid"),
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
                assert!(queries::get_kseaf(
                    &mut transaction,
                    &[section * num_rows + row; RES_STAR_LENGTH],
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
    async fn test_kseaf_insert_dupicate_fail() {
        let pool = init().await;
        let mut transaction = pool.begin().await.unwrap();

        queries::insert_kseaf(
            &mut transaction,
            &[0_u8; RES_STAR_LENGTH],
            &[1_u8; KSEAF_LENGTH],
        )
        .await
        .unwrap();

        queries::insert_kseaf(
            &mut transaction,
            &[0_u8; RES_STAR_LENGTH],
            &[1_u8; KSEAF_LENGTH],
        )
        .await
        .unwrap();

        transaction.commit().await.unwrap();
    }

    /// Test that insert works
    #[tokio::test]
    async fn test_user_info_insert() {
        let pool = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                queries::user_info_upsert(
                    &mut transaction,
                    &format!("user_info_{}", section * num_rows + row),
                    &[section * num_rows + row; K_LENGTH],
                    &[section * num_rows + row; OPC_LENGTH],
                    &[section * num_rows + row; SQN_LENGTH],
                )
                .await
                .unwrap();
            }
        }
        transaction.commit().await.unwrap();
    }

    /// Test that insert works
    #[tokio::test]
    async fn test_user_info_get() {
        let pool = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                queries::user_info_upsert(
                    &mut transaction,
                    &format!("user_info_{}", section * num_rows + row),
                    &[section * num_rows + row; K_LENGTH],
                    &[section * num_rows + row; OPC_LENGTH],
                    &[section * num_rows + row; SQN_LENGTH],
                )
                .await
                .unwrap();
            }
        }
        transaction.commit().await.unwrap();
        let mut transaction = pool.begin().await.unwrap();

        for section in 0..num_sections {
            for row in 0..num_rows {
                let res = queries::user_info_get(
                    &mut transaction,
                    &format!("user_info_{}", section * num_rows + row),
                )
                .await
                .unwrap();

                assert_eq!(
                    format!("user_info_{}", section * num_rows + row),
                    res.get_unchecked::<&str, &str>("user_info_id")
                );
                assert_eq!(
                    &[section * num_rows + row; K_LENGTH],
                    res.get_unchecked::<&[u8], &str>("user_info_k")
                );
                assert_eq!(
                    &[section * num_rows + row; OPC_LENGTH],
                    res.get_unchecked::<&[u8], &str>("user_info_opc")
                );
                assert_eq!(
                    &[section * num_rows + row; SQN_LENGTH],
                    res.get_unchecked::<&[u8], &str>("user_info_sqn_max")
                );
            }
        }
        transaction.commit().await.unwrap();
    }

    /// Test that delete works
    #[tokio::test]
    async fn test_user_info_remove() {
        let pool = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                queries::user_info_upsert(
                    &mut transaction,
                    &format!("user_info_{}", section * num_rows + row),
                    &[section * num_rows + row; K_LENGTH],
                    &[section * num_rows + row; OPC_LENGTH],
                    &[section * num_rows + row; SQN_LENGTH],
                )
                .await
                .unwrap();
            }
        }
        transaction.commit().await.unwrap();
        let mut transaction = pool.begin().await.unwrap();

        for section in 0..num_sections {
            for row in 0..num_rows {
                queries::user_info_remove(
                    &mut transaction,
                    &format!("user_info_{}", section * num_rows + row),
                )
                .await
                .unwrap();
            }
        }

        for section in 0..num_sections {
            for row in 0..num_rows {
                assert!(queries::user_info_get(
                    &mut transaction,
                    &format!("user_info_{}", section * num_rows + row),
                )
                .await
                .is_err());
            }
        }

        transaction.commit().await.unwrap();
    }

    /// Test that updates works
    #[tokio::test]
    async fn test_user_info_update() {
        let pool = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                queries::user_info_upsert(
                    &mut transaction,
                    &format!("user_info_{}", section * num_rows + row),
                    &[section * num_rows + row; K_LENGTH],
                    &[section * num_rows + row; OPC_LENGTH],
                    &[section * num_rows + row; SQN_LENGTH],
                )
                .await
                .unwrap();
            }
        }
        transaction.commit().await.unwrap();
        let mut transaction = pool.begin().await.unwrap();

        for section in 0..num_sections {
            for row in 0..num_rows {
                let res = queries::user_info_get(
                    &mut transaction,
                    &format!("user_info_{}", section * num_rows + row),
                )
                .await
                .unwrap();

                assert_eq!(
                    format!("user_info_{}", section * num_rows + row),
                    res.get_unchecked::<&str, &str>("user_info_id")
                );
                assert_eq!(
                    &[section * num_rows + row; K_LENGTH],
                    res.get_unchecked::<&[u8], &str>("user_info_k")
                );
                assert_eq!(
                    &[section * num_rows + row; OPC_LENGTH],
                    res.get_unchecked::<&[u8], &str>("user_info_opc")
                );
                assert_eq!(
                    &[section * num_rows + row; SQN_LENGTH],
                    res.get_unchecked::<&[u8], &str>("user_info_sqn_max")
                );
            }
        }
        transaction.commit().await.unwrap();
        let mut transaction = pool.begin().await.unwrap();

        for section in 0..num_sections {
            for row in 0..num_rows {
                queries::user_info_upsert(
                    &mut transaction,
                    &format!("user_info_{}", section * num_rows + row),
                    &[section * num_rows + row + 1; K_LENGTH],
                    &[section * num_rows + row + 2; OPC_LENGTH],
                    &[section * num_rows + row + 3; SQN_LENGTH],
                )
                .await
                .unwrap();
            }
        }
        transaction.commit().await.unwrap();
        let mut transaction = pool.begin().await.unwrap();

        for section in 0..num_sections {
            for row in 0..num_rows {
                let res = queries::user_info_get(
                    &mut transaction,
                    &format!("user_info_{}", section * num_rows + row),
                )
                .await
                .unwrap();

                assert_eq!(
                    format!("user_info_{}", section * num_rows + row),
                    res.get_unchecked::<&str, &str>("user_info_id")
                );

                // old values
                assert_ne!(
                    &[section * num_rows + row; K_LENGTH],
                    res.get_unchecked::<&[u8], &str>("user_info_k")
                );
                assert_ne!(
                    &[section * num_rows + row; OPC_LENGTH],
                    res.get_unchecked::<&[u8], &str>("user_info_opc")
                );
                assert_ne!(
                    &[section * num_rows + row; SQN_LENGTH],
                    res.get_unchecked::<&[u8], &str>("user_info_sqn_max")
                );

                // new values
                assert_eq!(
                    &[section * num_rows + row + 1; K_LENGTH],
                    res.get_unchecked::<&[u8], &str>("user_info_k")
                );
                assert_eq!(
                    &[section * num_rows + row + 2; OPC_LENGTH],
                    res.get_unchecked::<&[u8], &str>("user_info_opc")
                );
                assert_eq!(
                    &[section * num_rows + row + 3; SQN_LENGTH],
                    res.get_unchecked::<&[u8], &str>("user_info_sqn_max")
                );
            }
        }
        transaction.commit().await.unwrap();
    }
}
