use sqlx::sqlite::{SqlitePool, SqliteRow};
use sqlx::Error as SqlxError;
use sqlx::{Sqlite, Transaction};

use crate::data::error::DirectoryError;

/// Creates the backup networks table if it does not exist already.
/// Contains all networks that are used as a backup for this network
pub async fn init_table(pool: &SqlitePool) -> Result<(), DirectoryError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS backups_directory_table (
            user_id TEXT NOT NULL,
            backup_network_id TEXT NOT NULL,
            PRIMARY KEY (user_id, backup_network_id)
        );",
    )
    .execute(pool)
    .await?;
    Ok(())
}

/* Queries */

/// Adds a user with a backup network.
pub async fn add(
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &str,
    backup_network_id: &str,
) -> Result<(), SqlxError> {
    sqlx::query(
        "INSERT INTO backups_directory_table
        VALUES ($1,$2)",
    )
    .bind(user_id)
    .bind(backup_network_id)
    .execute(transaction)
    .await?;

    Ok(())
}

/// Gets all of the backup networks for a user.
pub async fn get(
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &str,
) -> Result<Vec<SqliteRow>, SqlxError> {
    Ok(sqlx::query(
        "SELECT * FROM backups_directory_table
        WHERE user_id=$1;",
    )
    .bind(user_id)
    .fetch_all(transaction)
    .await?)
}

/// Removes all backup networks for a user.
pub async fn remove(
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &str,
) -> Result<(), SqlxError> {
    sqlx::query(
        "DELETE FROM backups_directory_table
        WHERE user_id=$1",
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

    use crate::database::{backups, general};

    fn gen_name() -> String {
        let s: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();

        format!("sqlite_{}.db", s)
    }

    async fn init() -> (SqlitePool, TempDir) {
        let dir = tempdir().unwrap();
        let path = String::from(dir.path().join(gen_name()).to_str().unwrap());
        println!("Building temporary db: {}", path);

        let pool = general::build_pool(&path).await.unwrap();
        backups::init_table(&pool).await.unwrap();

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
            backups::add(
                &mut transaction,
                &format!("test_user_id_{}", row),
                &format!("test_network_id_{}", row),
            )
            .await
            .unwrap();
        }
        transaction.commit().await.unwrap();
    }

    #[tokio::test]
    async fn test_get() {
        let (pool, _dir) = init().await;

        let num_rows = 10;

        let mut transaction = pool.begin().await.unwrap();
        for row in 0..num_rows {
            backups::add(
                &mut transaction,
                &format!("test_user_id_0"),
                &format!("test_network_id_{}", row),
            )
            .await
            .unwrap();
        }
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        let res = backups::get(&mut transaction, &format!("test_user_id_0"))
            .await
            .unwrap();

        let mut xres = Vec::new();

        for i in 0..num_rows {
            xres.push(format!("test_network_id_{}", i));
        }

        for row in res {
            xres.retain(|x| x != row.get_unchecked::<&str, &str>("backup_network_id"));
        }

        assert!(xres.is_empty());

        transaction.commit().await.unwrap();
    }
    
    #[tokio::test]
    async fn test_remove() {
        let (pool, _dir) = init().await;

        let num_rows = 10;

        let mut transaction = pool.begin().await.unwrap();
        for row in 0..num_rows {
            backups::add(
                &mut transaction,
                &format!("test_user_id_0"),
                &format!("test_network_id_{}", row),
            )
            .await
            .unwrap();
        }
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        let res = backups::get(&mut transaction, &format!("test_user_id_0"))
            .await
            .unwrap();

        assert!(!res.is_empty());

        backups::remove(&mut transaction, &format!("test_user_id_0"))
            .await
            .unwrap();

        let res = backups::get(&mut transaction, &format!("test_user_id_0"))
            .await
            .unwrap();

        assert!(res.is_empty());

        transaction.commit().await.unwrap();
    }
}
