use sqlx::sqlite::SqlitePool;
use sqlx::{FromRow, Sqlite, Transaction};

use crate::data::error::DauthError;
use crate::data::keys;

#[derive(Clone)]
pub struct ReplaceKeyShareTask {
    pub backup_network_id: String,
    pub xres_star_hash: Vec<u8>,
    pub old_xres_star_hash: Vec<u8>,
    pub key_share: keys::KseafShare,
}

impl TryFrom<ReplaceKeyShareTaskRow> for ReplaceKeyShareTask {
    type Error = DauthError;
    fn try_from(value: ReplaceKeyShareTaskRow) -> Result<Self, Self::Error> {
        Ok(ReplaceKeyShareTask {
            backup_network_id: value.backup_network_id,
            xres_star_hash: value.xres_star_hash,
            old_xres_star_hash: value.old_xres_star_hash,
            key_share: value.key_share.try_into()?,
        })
    }
}

#[derive(FromRow, Clone)]
pub struct ReplaceKeyShareTaskRow {
    pub backup_network_id: String,
    pub xres_star_hash: Vec<u8>,
    pub old_xres_star_hash: Vec<u8>,
    pub key_share: Vec<u8>,
}

/// Creates the backup networks table if it does not exist already.
pub async fn init_table(pool: &SqlitePool) -> Result<(), DauthError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS replace_key_share_task_table (
            backup_network_id TEXT NOT NULL,
            xres_star_hash BLOB NOT NULL,
            old_xres_star_hash BLOB NOT NULL,
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
    old_xres_star_hash: &[u8],
    key_share: &keys::KseafShare,
) -> Result<(), DauthError> {
    sqlx::query(
        "INSERT INTO replace_key_share_task_table
        VALUES ($1,$2,$3,$4)",
    )
    .bind(backup_network_id)
    .bind(xres_star_hash)
    .bind(old_xres_star_hash)
    .bind(key_share.share.as_slice())
    .execute(transaction)
    .await?;

    Ok(())
}

/// Gets all pending key share replaces.
pub async fn get(
    transaction: &mut Transaction<'_, Sqlite>,
) -> Result<Vec<ReplaceKeyShareTask>, DauthError> {
    let rows: Vec<ReplaceKeyShareTaskRow> =
        sqlx::query_as("SELECT * FROM replace_key_share_task_table")
            .fetch_all(transaction)
            .await?;

    let mut res: Vec<ReplaceKeyShareTask> = Vec::with_capacity(rows.len());
    for row in rows {
        res.push(row.try_into()?)
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

    use crate::data::keys;
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
                &vec![row as u8],
                &keys::KseafShare { share: [0xFF; 33] },
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
                &vec![row as u8],
                &keys::KseafShare { share: [0xEE; 33] },
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
                &vec![row as u8],
                &keys::KseafShare { share: [0xDD; 33] },
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
