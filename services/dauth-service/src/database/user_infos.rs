use auth_vector::types::Id;
use sqlx::sqlite::SqlitePool;
use sqlx::{Sqlite, Transaction};

use crate::data::error::DauthError;
use crate::data::user_info::UserInfo;
use crate::database::utilities::DauthDataUtilities;

/// Creates the table if it does not exist already.
/// Contains the set of user infos owned by this network.
#[tracing::instrument(skip(pool), name = "database::user_infos")]
pub async fn init_table(pool: &SqlitePool) -> Result<(), DauthError> {
    tracing::info!("Initialzing table");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user_info_table (
            id TEXT NOT NULL,
            k BLOB NOT NULL,
            opc BLOB NOT NULL,
            sqn_max INT NOT NULL,
            sqn_slice INT NOT NULL,
            PRIMARY KEY (id, sqn_slice)
        );",
    )
    .execute(pool)
    .await?;
    Ok(())
}

/* Queries */

/// Get user info if exists.
#[tracing::instrument(skip(transaction), name = "database::user_infos")]
pub async fn get(
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &Id,
    sqn_slice: i64,
) -> Result<UserInfo, DauthError> {
    tracing::debug!("Getting user info");

    Ok(sqlx::query(
        "SELECT * FROM user_info_table
        WHERE (id,sqn_slice)=($1,$2);",
    )
    .bind(user_id)
    .bind(sqn_slice)
    .fetch_one(transaction)
    .await?
    .to_user_info()?)
}

#[derive(Debug, Clone, Copy, sqlx::FromRow)]
struct SqnMaxRow {
    pub sqn_max: i64,
}
/// Insert user info and replace if exists.
#[tracing::instrument(skip(transaction), name = "database::user_infos")]
pub async fn upsert(
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &Id,
    k: &[u8],
    opc: &[u8],
    sqn_max: i64,
    sqn_slice: i64,
) -> Result<(), DauthError> {
    tracing::debug!("Upserting user info");

    // HACK Don't ever decrease a user's sequence number unless the slice config
    // changes...
    let mut sqn_to_insert = sqn_max;
    let user_max_sqn: Option<SqnMaxRow> = sqlx::query_as(
        "SELECT sqn_max FROM user_info_table
            WHERE id=$1 AND sqn_slice=$2;
            ",
    )
    .bind(user_id)
    .bind(sqn_slice)
    .fetch_optional::<&mut Transaction<'_, Sqlite>>(transaction)
    .await?;

    let user_max_sqn = user_max_sqn.unwrap_or(SqnMaxRow { sqn_max: 0 });
    let user_max_sqn = user_max_sqn.sqn_max;
    if (user_max_sqn % 32) == (sqn_max % 32) {
        sqn_to_insert = std::cmp::max(user_max_sqn, sqn_max);
    }

    sqlx::query(
        "REPLACE INTO user_info_table
        VALUES ($1,$2,$3,$4,$5);",
    )
    .bind(user_id)
    .bind(k)
    .bind(opc)
    .bind(sqn_to_insert)
    .bind(sqn_slice)
    .execute::<&mut Transaction<'_, Sqlite>>(transaction)
    .await?;

    Ok(())
}

/// Remove user info if exists.
/// Not currently used.
#[allow(dead_code)]
#[tracing::instrument(skip(transaction), name = "database::user_infos")]
pub async fn remove(
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &Id,
) -> Result<(), DauthError> {
    tracing::debug!("Removing user info");

    sqlx::query(
        "DELETE FROM user_info_table
        WHERE id=$1",
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

    use auth_vector::types::{K_LENGTH, OPC_LENGTH};

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
                    (section * num_rows + row) as i64,
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
                    (section * num_rows + row) as i64,
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

                assert_eq!(format!("user_info_{}", section * num_rows + row), res.id);
                assert_eq!([section * num_rows + row; K_LENGTH], res.k);
                assert_eq!([section * num_rows + row; OPC_LENGTH], res.opc);
                assert_eq!((section * num_rows + row) as i64, res.sqn);
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
                    (section * num_rows + row) as i64,
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

                assert_eq!(format!("user_info_{}", section * num_rows + row), res.id);
                assert_eq!([section * num_rows + row; K_LENGTH], res.k);
                assert_eq!([section * num_rows + row; OPC_LENGTH], res.opc);
                assert_eq!(1, res.sqn);
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

                assert_eq!(format!("user_info_{}", section * num_rows + row), res.id);

                // old values
                assert_ne!([section * num_rows + row; K_LENGTH], res.k);
                assert_ne!([section * num_rows + row; OPC_LENGTH], res.opc);
                assert_ne!(1, res.sqn);

                // new values
                assert_eq!([section * num_rows + row + 1; K_LENGTH], res.k);
                assert_eq!([section * num_rows + row + 2; OPC_LENGTH], res.opc);
                assert_eq!(2, res.sqn);
            }
        }
        transaction.commit().await.unwrap();
    }
}
