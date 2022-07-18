use sqlx::sqlite::SqlitePool;
use sqlx::{FromRow, Sqlite, Transaction};

use crate::data::error::DauthError;

#[derive(FromRow, Clone)]
pub struct ReportAuthVectorTask {
    pub xres_star_hash: Vec<u8>,
    pub user_id: String,
    pub signed_request_bytes: Vec<u8>,
    pub task_id: i64,
}

/// Creates the backup networks table if it does not exist already.
pub async fn init_table(pool: &SqlitePool) -> Result<(), DauthError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS report_auth_vectors_task_table (
            task_id INTEGER PRIMARY KEY,
            xres_star_hash BLOB,
            user_id TEXT NOT NULL,
            signed_request_bytes BLOB NOT NULL
        );",
    )
    .execute(pool)
    .await?;
    Ok(())
}

/* Queries */

/// Adds a pending auth vector used report.
pub async fn add(
    transaction: &mut Transaction<'_, Sqlite>,
    xres_star_hash: &[u8],
    user_id: &str,
    signed_request_bytes: &Vec<u8>,
) -> Result<(), DauthError> {
    sqlx::query(
        "INSERT INTO report_auth_vectors_task_table (xres_star_hash, user_id, signed_request_bytes)
        VALUES ($1,$2,$3)",
    )
    .bind(xres_star_hash)
    .bind(user_id)
    .bind(signed_request_bytes)
    .execute(transaction)
    .await?;

    Ok(())
}

/// Gets all pending auth vector used reports.
pub async fn get(
    transaction: &mut Transaction<'_, Sqlite>,
) -> Result<Vec<ReportAuthVectorTask>, DauthError> {
    let rows: Vec<ReportAuthVectorTask> =
        sqlx::query_as("SELECT * FROM report_auth_vectors_task_table")
            .fetch_all(transaction)
            .await?;

    Ok(rows)
}

/// Removes a specific auth vector used report.
pub async fn remove(
    transaction: &mut Transaction<'_, Sqlite>,
    row_id: i64,
) -> Result<(), DauthError> {
    sqlx::query(
        "DELETE FROM report_auth_vectors_task_table
        WHERE task_id=$1",
    )
    .bind(row_id)
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

    use crate::database::{general, tasks};

    fn gen_name() -> String {
        let s: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();

        format!("sqlite_{}.db", s)
    }

    async fn init() -> (SqlitePool, TempDir) {
        let dir = tempdir().unwrap();
        let path = String::from(dir.path().join(gen_name()).to_str().unwrap());
        println!("Building temporary db: {}", path);

        let pool = general::build_pool(&path).await.unwrap();
        tasks::report_auth_vectors::init_table(&pool).await.unwrap();

        (pool, dir)
    }

    #[tokio::test]
    async fn test_db_init() {
        init().await;
    }

    #[tokio::test]
    async fn test_add() {
        let (pool, _dir) = init().await;
        let num_rows = 10;

        let mut transaction = pool.begin().await.unwrap();
        for row in 0..num_rows {
            tasks::report_auth_vectors::add(
                &mut transaction,
                &vec![row as u8],
                &format!("test_user_id_{}", row),
                &vec![row as u8],
            )
            .await
            .unwrap()
        }
        transaction.commit().await.unwrap();
    }

    #[tokio::test]
    async fn test_get() {
        let (pool, _dir) = init().await;
        let num_rows = 10;

        let mut names = Vec::with_capacity(num_rows);

        let mut transaction = pool.begin().await.unwrap();
        for row in 0..num_rows {
            tasks::report_auth_vectors::add(
                &mut transaction,
                &vec![row as u8],
                &format!("test_user_id_{}", row),
                &vec![row as u8],
            )
            .await
            .unwrap();
            names.push(format!("test_user_id_{}", row));
        }
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        for res in tasks::report_auth_vectors::get(&mut transaction)
            .await
            .unwrap()
        {
            assert!(names.contains(&res.user_id));
        }
        transaction.commit().await.unwrap();
    }

    #[tokio::test]
    async fn test_remove() {
        let (pool, _dir) = init().await;
        let num_rows = 10;

        let mut names = Vec::with_capacity(num_rows);

        let mut transaction = pool.begin().await.unwrap();
        for row in 0..num_rows {
            tasks::report_auth_vectors::add(
                &mut transaction,
                &vec![row as u8],
                &format!("test_user_id_{}", row),
                &vec![row as u8],
            )
            .await
            .unwrap();
            names.push(format!("test_user_id_{}", row));
        }
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        let reports = tasks::report_auth_vectors::get(&mut transaction)
            .await
            .unwrap();
        for res in reports.iter() {
            assert!(names.contains(&res.user_id));
        }
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        for row in reports.iter() {
            tasks::report_auth_vectors::remove(&mut transaction, row.task_id)
                .await
                .unwrap();
        }
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        assert!(
            tasks::report_auth_vectors::get(&mut transaction)
                .await
                .unwrap()
                .len()
                == 0
        );
        transaction.commit().await.unwrap();
    }
}
