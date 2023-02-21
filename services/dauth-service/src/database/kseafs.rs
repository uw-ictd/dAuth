use auth_vector::types::Kseaf;
use sqlx::sqlite::SqlitePool;
use sqlx::{Sqlite, Transaction};

use crate::data::error::DauthError;
use crate::database::utilities::DauthDataUtilities;

/// Creates the kseaf table if it does not exist already.
#[tracing::instrument(skip(pool), name = "database::kseafs")]
pub async fn init_table(pool: &SqlitePool) -> Result<(), DauthError> {
    tracing::info!("Initialzing table");

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
#[tracing::instrument(skip(transaction), name = "database::kseafs")]
pub async fn add(
    transaction: &mut Transaction<'_, Sqlite>,
    uuid: &[u8],
    value: &[u8],
) -> Result<(), DauthError> {
    tracing::info!("Adding kseaf");

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

/// Returns a kseaf value if found.
#[tracing::instrument(skip(transaction), name = "database::kseafs")]
pub async fn get(
    transaction: &mut Transaction<'_, Sqlite>,
    uuid: &[u8],
) -> Result<Kseaf, DauthError> {
    tracing::info!("Getting kseaf");

    Ok(sqlx::query(
        "SELECT * FROM kseaf_table
        WHERE kseaf_uuid=$1;",
    )
    .bind(uuid)
    .fetch_one(transaction)
    .await?
    .to_kseaf()?)
}

/// Deletes a kseaf vaule if found.
#[tracing::instrument(skip(transaction), name = "database::kseafs")]
pub async fn remove(
    transaction: &mut Transaction<'_, Sqlite>,
    uuid: &[u8],
) -> Result<(), DauthError> {
    tracing::info!("Removing kseaf");

    sqlx::query(
        "DELETE FROM kseaf_table
        WHERE kseaf_uuid=$1",
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
    use sqlx::SqlitePool;
    use tempfile::{tempdir, TempDir};

    use auth_vector::types::{KSEAF_LENGTH, XRES_STAR_LENGTH};

    use crate::database::{general, kseafs};

    fn gen_name() -> String {
        let s: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();

        format!("sqlite_{}.db", s)
    }

    async fn init() -> (SqlitePool, TempDir) {
        let dir = tempdir().unwrap();
        let path = String::from(dir.path().join(gen_name()).to_str().unwrap());
        println!("Building temporary db: {}", path);

        let pool = general::build_pool(&path).await.unwrap();
        kseafs::init_table(&pool).await.unwrap();

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
                kseafs::add(
                    &mut transaction,
                    &[section * num_rows + row; XRES_STAR_LENGTH],
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
        let (pool, _dir) = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                kseafs::add(
                    &mut transaction,
                    &[section * num_rows + row; XRES_STAR_LENGTH],
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
                let res = kseafs::get(
                    &mut transaction,
                    &[section * num_rows + row; XRES_STAR_LENGTH],
                )
                .await
                .unwrap();

                assert_eq!([section * num_rows + row; KSEAF_LENGTH], res);
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
                kseafs::add(
                    &mut transaction,
                    &[section * num_rows + row; XRES_STAR_LENGTH],
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
                kseafs::remove(
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
                assert!(kseafs::get(
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
                kseafs::add(
                    &mut transaction,
                    &[section * num_rows + row; XRES_STAR_LENGTH],
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
                let res = kseafs::get(
                    &mut transaction,
                    &[section * num_rows + row; XRES_STAR_LENGTH],
                )
                .await
                .unwrap();

                assert_eq!([section * num_rows + row; KSEAF_LENGTH], res);

                kseafs::remove(&mut transaction, &res).await.unwrap();
            }
        }

        transaction.commit().await.unwrap();
        let mut transaction = pool.begin().await.unwrap();

        for section in 0..num_sections {
            for row in 0..num_rows {
                // should have been deleted
                assert!(kseafs::get(
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

        kseafs::add(
            &mut transaction,
            &[0_u8; XRES_STAR_LENGTH],
            &[1_u8; KSEAF_LENGTH],
        )
        .await
        .unwrap();

        kseafs::add(
            &mut transaction,
            &[0_u8; XRES_STAR_LENGTH],
            &[1_u8; KSEAF_LENGTH],
        )
        .await
        .unwrap();

        transaction.commit().await.unwrap();
    }
}
