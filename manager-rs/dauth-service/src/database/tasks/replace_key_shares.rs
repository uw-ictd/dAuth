use sqlx::sqlite::SqlitePool;
use sqlx::{FromRow, Sqlite, Transaction};

use crate::data::error::DauthError;

#[derive(FromRow)]
pub struct GetResult {
    backup_network_id: String,
    xres_star_hash: Vec<u8>,
    key_share: Vec<u8>,
}

/// Creates the backup networks table if it does not exist already.
/// Contains all networks that are used as a backup for this network
pub async fn init_table(pool: &SqlitePool) -> Result<(), DauthError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS replace_key_share_task_table (
            backup_network_id TEXT NOT NULL,
            xres_star_hash BLOB NOT NULL,
            key_share BLOB NOT NULL,
            PRIMARY KEY (backup_network_id, xres_star_hash)
        );",
    )
    .execute(pool)
    .await?;
    Ok(())
}

/* Queries */

/// Adds a pending key share replace.
pub async fn add(
    transaction: &mut Transaction<'_, Sqlite>,
    backup_network_id: &str,
    xres_star_hash: &[u8],
    key_share: &[u8],
) -> Result<(), DauthError> {
    sqlx::query(
        "INSERT INTO replace_key_share_task_table
        VALUES ($1,$2,$3)",
    )
    .bind(backup_network_id)
    .bind(xres_star_hash)
    .bind(key_share)
    .execute(transaction)
    .await?;

    Ok(())
}

/// Gets all pending key share replaces.
pub async fn get(transaction: &mut Transaction<'_, Sqlite>) -> Result<Vec<GetResult>, DauthError> {
    let rows = sqlx::query("SELECT * FROM replace_key_share_task_table")
        .fetch_all(transaction)
        .await?;

    let mut res = Vec::with_capacity(rows.len());
    for row in rows {
        res.push(GetResult::from_row(&row)?)
    }
    Ok(res)
}

/// Removes a specific key share replace.
pub async fn remove(
    transaction: &mut Transaction<'_, Sqlite>,
    backup_network_id: &str,
    xres_star_hash: &[u8],
) -> Result<(), DauthError> {
    sqlx::query(
        "DELETE FROM replace_key_share_task_table
        WHERE (backup_network_id,xres_star_hash)=($1,$2)",
    )
    .bind(backup_network_id)
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
        tasks::replace_key_shares::init_table(&pool).await.unwrap();

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
            tasks::replace_key_shares::add(
                &mut transaction,
                &format!("test_backup_network_{}", row),
                &vec![row as u8],
                &vec![0u8],
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
            tasks::replace_key_shares::add(
                &mut transaction,
                &format!("test_backup_network_{}", row),
                &vec![row as u8],
                &vec![0u8],
            )
            .await
            .unwrap();
            names.push(format!("test_backup_network_{}", row));
        }
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        for res in tasks::replace_key_shares::get(&mut transaction)
            .await
            .unwrap()
        {
            assert!(names.contains(&res.backup_network_id));
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
            tasks::replace_key_shares::add(
                &mut transaction,
                &format!("test_backup_network_{}", row),
                &vec![row as u8],
                &vec![0u8],
            )
            .await
            .unwrap();
            names.push(format!("test_backup_network_{}", row));
        }
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        for res in tasks::replace_key_shares::get(&mut transaction)
            .await
            .unwrap()
        {
            assert!(names.contains(&res.backup_network_id));
        }
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        for row in 0..num_rows {
            tasks::replace_key_shares::remove(
                &mut transaction,
                &format!("test_backup_network_{}", row),
                &vec![row as u8],
            )
            .await
            .unwrap();
        }
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        assert!(
            tasks::replace_key_shares::get(&mut transaction)
                .await
                .unwrap()
                .len()
                == 0
        );
        transaction.commit().await.unwrap();
    }
}