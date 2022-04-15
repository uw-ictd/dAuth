use sqlx::sqlite::{SqlitePool, SqliteRow};
use sqlx::{Sqlite, Transaction};

use crate::data::error::DauthError;

/// Creates the kseaf table if it does not exist already.
pub async fn init_table(pool: &SqlitePool) -> Result<(), DauthError> {
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

/* Queries */

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

/* Testing */

#[cfg(test)]
mod tests {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use sqlx::{Row, SqlitePool};
    use tempfile::tempdir;

    use auth_vector::constants::{
        KSEAF_LENGTH, RES_STAR_LENGTH,
    };

    use crate::local::queries::{kseafs, general};

    fn gen_name() -> String {
        let s: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();

        format!("sqlite_{}.db", s)
    }

    async fn init() -> SqlitePool {
        let dir = tempdir().unwrap();
        let path = String::from(dir.path().join(gen_name()).to_str().unwrap());
        println!("Building temporary db: {}", path);

        let pool = general::build_pool(&path).await.unwrap();
        kseafs::init_table(&pool).await.unwrap();

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
                kseafs::insert_kseaf(
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
    async fn test_get() {
        let pool = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                kseafs::insert_kseaf(
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
                let res = kseafs::get_kseaf(
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
    async fn test_remove() {
        let pool = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                kseafs::insert_kseaf(
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
                kseafs::delete_kseaf(
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
                assert!(kseafs::get_kseaf(
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
                kseafs::insert_kseaf(
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
                let res = kseafs::get_kseaf(
                    &mut transaction,
                    &[section * num_rows + row; RES_STAR_LENGTH],
                )
                .await
                .unwrap();

                assert_eq!(
                    &[section * num_rows + row; KSEAF_LENGTH],
                    res.get_unchecked::<&[u8], &str>("kseaf_data")
                );

                kseafs::delete_kseaf(
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
                assert!(kseafs::get_kseaf(
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

        kseafs::insert_kseaf(
            &mut transaction,
            &[0_u8; RES_STAR_LENGTH],
            &[1_u8; KSEAF_LENGTH],
        )
        .await
        .unwrap();

        kseafs::insert_kseaf(
            &mut transaction,
            &[0_u8; RES_STAR_LENGTH],
            &[1_u8; KSEAF_LENGTH],
        )
        .await
        .unwrap();

        transaction.commit().await.unwrap();
    }
}
