use auth_vector::types::Id;
use sqlx::sqlite::{SqlitePool, SqliteRow};
use sqlx::{Sqlite, Transaction};

use crate::data::error::DauthError;

/// Creates the table if it does not exist already.
pub async fn init_table(pool: &SqlitePool) -> Result<(), DauthError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user_info_table (
            user_info_id TEXT NOT NULL,
            user_info_k BLOB NOT NULL,
            user_info_opc BLOB NOT NULL,
            user_info_sqn_max INT NOT NULL,
            user_info_sqn_slice INT NOT NULL,
            PRIMARY KEY (user_info_id, user_info_sqn_slice)
        );",
    )
    .execute(pool)
    .await?;
    Ok(())
}

/* Queries */

/// Get user info if exists.
pub async fn get(
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &Id,
    sqn_slice: u32,
) -> Result<SqliteRow, DauthError> {
    Ok(sqlx::query(
        "SELECT * FROM user_info_table
        WHERE (user_info_id,user_info_sqn_slice)=($1,$2);",
    )
    .bind(user_id)
    .bind(sqn_slice)
    .fetch_one(transaction)
    .await?)
}

/// Insert user info and replace if exists.
pub async fn upsert(
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &Id,
    k: &[u8],
    opc: &[u8],
    sqn_max: u64,
    sqn_slice: u32,
) -> Result<(), DauthError> {
    sqlx::query(
        "REPLACE INTO user_info_table
        VALUES ($1,$2,$3,$4,$5);",
    )
    .bind(user_id)
    .bind(k)
    .bind(opc)
    .bind(sqn_max as i64)
    .bind(sqn_slice)
    .execute(transaction)
    .await?;

    Ok(())
}

/// Remove user info if exists.
pub async fn remove(
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
    use tempfile::{tempdir, TempDir};

    use auth_vector::constants::{K_LENGTH, OPC_LENGTH, SQN_LENGTH};

    use crate::database::{general, user_infos};

    fn gen_name() -> String {
        let s: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();

        format!("sqlite_{}.db", s)
    }

    async fn init() -> (SqlitePool, TempDir) {
        let dir = tempdir().unwrap();
        let path = String::from(dir.path().join(gen_name()).to_str().unwrap());
        println!("Building temporary db: {}", path);

        let pool = general::build_pool(&path).await.unwrap();
        user_infos::init_table(&pool).await.unwrap();

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
                user_infos::upsert(
                    &mut transaction,
                    &format!("user_info_{}", section * num_rows + row),
                    &[section * num_rows + row; K_LENGTH],
                    &[section * num_rows + row; OPC_LENGTH],
                    (section * num_rows + row) as u64,
                    0,
                )
                .await
                .unwrap();
            }
        }
        transaction.commit().await.unwrap();
    }

    /// Test that insert works
    #[tokio::test]
    async fn test_get() {
        let (pool, _dir) = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                user_infos::upsert(
                    &mut transaction,
                    &format!("user_info_{}", section * num_rows + row),
                    &[section * num_rows + row; K_LENGTH],
                    &[section * num_rows + row; OPC_LENGTH],
                    (section * num_rows + row) as u64,
                    0,
                )
                .await
                .unwrap();
            }
        }
        transaction.commit().await.unwrap();
        let mut transaction = pool.begin().await.unwrap();

        for section in 0..num_sections {
            for row in 0..num_rows {
                let res = user_infos::get(
                    &mut transaction,
                    &format!("user_info_{}", section * num_rows + row),
                    0,
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
                    (section * num_rows + row) as u64,
                    res.get_unchecked::<i64, &str>("user_info_sqn_max") as u64
                );
            }
        }
        transaction.commit().await.unwrap();
    }

    /// Test that delete works
    #[tokio::test]
    async fn test_remove() {
        let (pool, _dir) = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                user_infos::upsert(
                    &mut transaction,
                    &format!("user_info_{}", section * num_rows + row),
                    &[section * num_rows + row; K_LENGTH],
                    &[section * num_rows + row; OPC_LENGTH],
                    (section * num_rows + row) as u64,
                    0,
                )
                .await
                .unwrap();
            }
        }
        transaction.commit().await.unwrap();
        let mut transaction = pool.begin().await.unwrap();

        for section in 0..num_sections {
            for row in 0..num_rows {
                user_infos::remove(
                    &mut transaction,
                    &format!("user_info_{}", section * num_rows + row),
                )
                .await
                .unwrap();
            }
        }

        for section in 0..num_sections {
            for row in 0..num_rows {
                assert!(user_infos::get(
                    &mut transaction,
                    &format!("user_info_{}", section * num_rows + row),
                    0,
                )
                .await
                .is_err());
            }
        }

        transaction.commit().await.unwrap();
    }

    /// Test that updates works
    #[tokio::test]
    async fn test_update() {
        let (pool, _dir) = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                user_infos::upsert(
                    &mut transaction,
                    &format!("user_info_{}", section * num_rows + row),
                    &[section * num_rows + row; K_LENGTH],
                    &[section * num_rows + row; OPC_LENGTH],
                    1,
                    0,
                )
                .await
                .unwrap();
            }
        }
        transaction.commit().await.unwrap();
        let mut transaction = pool.begin().await.unwrap();

        for section in 0..num_sections {
            for row in 0..num_rows {
                let res = user_infos::get(
                    &mut transaction,
                    &format!("user_info_{}", section * num_rows + row),
                    0,
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
                    1,
                    res.get_unchecked::<i64, &str>("user_info_sqn_max") as u64
                );
            }
        }
        transaction.commit().await.unwrap();
        let mut transaction = pool.begin().await.unwrap();

        for section in 0..num_sections {
            for row in 0..num_rows {
                user_infos::upsert(
                    &mut transaction,
                    &format!("user_info_{}", section * num_rows + row),
                    &[section * num_rows + row + 1; K_LENGTH],
                    &[section * num_rows + row + 2; OPC_LENGTH],
                    2,
                    0,
                )
                .await
                .unwrap();
            }
        }
        transaction.commit().await.unwrap();
        let mut transaction = pool.begin().await.unwrap();

        for section in 0..num_sections {
            for row in 0..num_rows {
                let res = user_infos::get(
                    &mut transaction,
                    &format!("user_info_{}", section * num_rows + row),
                    0,
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
                    1,
                    res.get_unchecked::<i64, &str>("user_info_sqn_max") as u64
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
                    2,
                    res.get_unchecked::<i64, &str>("user_info_sqn_max") as u64
                );
            }
        }
        transaction.commit().await.unwrap();
    }
}
