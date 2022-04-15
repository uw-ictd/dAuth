use sqlx::sqlite::{SqlitePool, SqliteRow};
use sqlx::{Sqlite, Transaction};

use crate::data::error::DauthError;

/// Creates the kseaf table if it does not exist already.
pub async fn init_table(pool: &SqlitePool) -> Result<(), DauthError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS key_share_table (
            xres_star_hash BLOB PRIMARY KEY,
            key_share BLOB NOT NULL
        );",
    )
    .execute(pool)
    .await?;
    Ok(())
}

/* Queries */

/// Inserts a key share
pub async fn add(
    transaction: &mut Transaction<'_, Sqlite>,
    xres_star_hash: &[u8],
    key_share: &[u8],
) -> Result<(), DauthError> {
    sqlx::query(
        "INSERT INTO key_share_table
        VALUES ($1,$2)",
    )
    .bind(xres_star_hash)
    .bind(key_share)
    .execute(transaction)
    .await?;

    Ok(())
}

/// Returns a key share if found.
pub async fn get(
    transaction: &mut Transaction<'_, Sqlite>,
    xres_star_hash: &[u8],
) -> Result<SqliteRow, DauthError> {
    Ok(sqlx::query(
        "SELECT * FROM key_share_table
        WHERE xres_star_hash=$1;",
    )
    .bind(xres_star_hash)
    .fetch_one(transaction)
    .await?)
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
    use sqlx::{Row, SqlitePool};
    use tempfile::tempdir;

    use auth_vector::constants::{
        KSEAF_LENGTH, RES_STAR_HASH_LENGTH, RES_STAR_LENGTH,
    };

    use crate::local::queries::{key_shares, general};

    fn gen_name() -> String {
        let s: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();

        format!("sqlite_{}.db", s)
    }

    async fn init() -> SqlitePool {
        let dir = tempdir().unwrap();
        let path = String::from(dir.path().join(gen_name()).to_str().unwrap());
        println!("Building temporary db: {}", path);

        let pool = general::build_pool(&path).await.unwrap();
        key_shares::init_table(&pool).await.unwrap();

        pool
    }

    /// Test that db and table creation will work
    #[tokio::test]
    async fn test_db_init() {
        init().await;
    }
    
    /// Test that insert works
    #[tokio::test]
    async fn test_add() {
        let pool = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                key_shares::add(
                    &mut transaction,
                    &[section * num_rows + row; RES_STAR_HASH_LENGTH],
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
    async fn test_get() {
        let pool = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                key_shares::add(
                    &mut transaction,
                    &[section * num_rows + row; RES_STAR_HASH_LENGTH],
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
                let res = key_shares::get(
                    &mut transaction,
                    &[section * num_rows + row; RES_STAR_HASH_LENGTH],
                )
                .await
                .unwrap();

                assert_eq!(
                    &[section * num_rows + row; KSEAF_LENGTH],
                    res.get_unchecked::<&[u8], &str>("key_share")
                );
            }
        }
        transaction.commit().await.unwrap();
    }

    /// Test that deletes work
    #[tokio::test]
    async fn test_remove() {
        let pool = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                key_shares::add(
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
                key_shares::remove(
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
                assert!(key_shares::get(
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
    async fn test_get_with_delete() {
        let pool = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                key_shares::add(
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
                let res = key_shares::get(
                    &mut transaction,
                    &[section * num_rows + row; RES_STAR_LENGTH],
                )
                .await
                .unwrap();

                assert_eq!(
                    &[section * num_rows + row; KSEAF_LENGTH],
                    res.get_unchecked::<&[u8], &str>("key_share")
                );

                key_shares::remove(
                    &mut transaction,
                    res.get_unchecked::<&[u8], &str>("xres_star_hash"),
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
                assert!(key_shares::get(
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
    async fn test_add_dupicate_fail() {
        let pool = init().await;
        let mut transaction = pool.begin().await.unwrap();

        key_shares::add(
            &mut transaction,
            &[0_u8; RES_STAR_LENGTH],
            &[1_u8; KSEAF_LENGTH],
        )
        .await
        .unwrap();

        key_shares::add(
            &mut transaction,
            &[0_u8; RES_STAR_LENGTH],
            &[1_u8; KSEAF_LENGTH],
        )
        .await
        .unwrap();

        transaction.commit().await.unwrap();
    }
}
