use sqlx::sqlite::{SqlitePool, SqliteRow};
use sqlx::{Sqlite, Transaction};

use auth_vector::types::Kasme;
use crate::data::error::DauthError;

/// Creates the kasme table if it does not exist already.
pub async fn init_table(pool: &SqlitePool) -> Result<(), DauthError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS kasme_table (
            kasme_uuid BLOB PRIMARY KEY,
            kasme_data BLOB NOT NULL
        );",
    )
    .execute(pool)
    .await?;
    Ok(())
}

/* Queries */

/// Inserts a kasme with a given uuid and value.
pub async fn add(
    transaction: &mut Transaction<'_, Sqlite>,
    uuid: &[u8],
    value: &Kasme,
) -> Result<(), DauthError> {
    sqlx::query(
        "INSERT INTO kasme_table
        VALUES ($1,$2)",
    )
    .bind(uuid)
    .bind(value.to_vec())
    .execute(transaction)
    .await?;

    Ok(())
}

/// Returns a kasme value if found.
pub async fn get(
    transaction: &mut Transaction<'_, Sqlite>,
    uuid: &[u8],
) -> Result<SqliteRow, DauthError> {
    Ok(sqlx::query(
        "SELECT * FROM kasme_table
        WHERE kasme_uuid=$1;",
    )
    .bind(uuid)
    .fetch_one(transaction)
    .await?)
}

/// Deletes a kasme vaule if found.
pub async fn remove(
    transaction: &mut Transaction<'_, Sqlite>,
    uuid: &[u8],
) -> Result<(), DauthError> {
    sqlx::query(
        "DELETE FROM kasme_table
        WHERE kasme_uuid=$1",
    )
    .bind(uuid)
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

    use auth_vector::types::{KASME_LENGTH, XRES_STAR_LENGTH};

    use crate::database::{general, kasmes};

    fn gen_name() -> String {
        let s: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();

        format!("sqlite_{}.db", s)
    }

    async fn init() -> (SqlitePool, TempDir) {
        let dir = tempdir().unwrap();
        let path = String::from(dir.path().join(gen_name()).to_str().unwrap());
        println!("Building temporary db: {}", path);

        let pool = general::build_pool(&path).await.unwrap();
        kasmes::init_table(&pool).await.unwrap();

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
                kasmes::add(
                    &mut transaction,
                    &[section * num_rows + row; XRES_STAR_LENGTH],
                    &vec![section * num_rows + row; KASME_LENGTH].try_into().unwrap(),
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
                kasmes::add(
                    &mut transaction,
                    &[section * num_rows + row; XRES_STAR_LENGTH],
                    &vec![section * num_rows + row; KASME_LENGTH].try_into().unwrap(),
                )
                .await
                .unwrap();
            }
        }

        transaction.commit().await.unwrap();
        let mut transaction = pool.begin().await.unwrap();

        for section in 0..num_sections {
            for row in 0..num_rows {
                let res = kasmes::get(
                    &mut transaction,
                    &[section * num_rows + row; XRES_STAR_LENGTH],
                )
                .await
                .unwrap();

                assert_eq!(
                    &[section * num_rows + row; KASME_LENGTH],
                    res.get_unchecked::<&[u8], &str>("kasme_data")
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
                kasmes::add(
                    &mut transaction,
                    &[section * num_rows + row; XRES_STAR_LENGTH],
                    &vec![section * num_rows + row; KASME_LENGTH].try_into().unwrap(),
                )
                .await
                .unwrap();
            }
        }

        transaction.commit().await.unwrap();
        let mut transaction = pool.begin().await.unwrap();

        for section in 0..num_sections {
            for row in 0..num_rows {
                kasmes::remove(
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
                assert!(kasmes::get(
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
                kasmes::add(
                    &mut transaction,
                    &[section * num_rows + row; XRES_STAR_LENGTH],
                    &vec![section * num_rows + row; KASME_LENGTH].try_into().unwrap(),
                )
                .await
                .unwrap();
            }
        }

        transaction.commit().await.unwrap();
        let mut transaction = pool.begin().await.unwrap();

        for section in 0..num_sections {
            for row in 0..num_rows {
                let res = kasmes::get(
                    &mut transaction,
                    &[section * num_rows + row; XRES_STAR_LENGTH],
                )
                .await
                .unwrap();

                assert_eq!(
                    &[section * num_rows + row; KASME_LENGTH],
                    res.get_unchecked::<&[u8], &str>("kasme_data")
                );

                kasmes::remove(
                    &mut transaction,
                    res.get_unchecked::<&[u8], &str>("kasme_uuid"),
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
                assert!(kasmes::get(
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

        kasmes::add(
            &mut transaction,
            &[0_u8; XRES_STAR_LENGTH],
            &vec![1_u8; KASME_LENGTH].try_into().unwrap(),
        )
        .await
        .unwrap();

        kasmes::add(
            &mut transaction,
            &[0_u8; XRES_STAR_LENGTH],
            &vec![1_u8; KASME_LENGTH].try_into().unwrap(),
        )
        .await
        .unwrap();

        transaction.commit().await.unwrap();
    }
}
