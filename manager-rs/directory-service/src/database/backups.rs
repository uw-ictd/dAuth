use sqlx::sqlite::SqlitePool;
use sqlx::{Row, Sqlite, Transaction};

use crate::data::error::DirectoryError;

/// Creates the backup networks table if it does not exist already.
/// Contains all networks that are used as a backup for this network
pub async fn init_table(pool: &SqlitePool) -> Result<(), DirectoryError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS backups_directory_table (
            user_id TEXT NOT NULL
                REFERENCES users_directory_table(user_id),
            backup_network_id TEXT NOT NULL 
                REFERENCES networks_directory_table(network_id),
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
) -> Result<(), DirectoryError> {
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
) -> Result<Vec<String>, DirectoryError> {
    let rows = sqlx::query(
        "SELECT * FROM backups_directory_table
        WHERE user_id=$1;",
    )
    .bind(user_id)
    .fetch_all(transaction)
    .await?;

    let mut res = Vec::new();
    for row in rows {
        res.push(row.get_unchecked::<String, &str>("backup_network_id"))
    }
    Ok(res)
}

/// Removes all backup networks for a user.
pub async fn remove(
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &str,
) -> Result<(), DirectoryError> {
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
    use sqlx::SqlitePool;
    use tempfile::{tempdir, TempDir};

    use crate::database::{backups, general, networks, users};

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
        users::init_table(&pool).await.unwrap();
        networks::init_table(&pool).await.unwrap();

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
            networks::upsert(
                &mut transaction,
                &format!("test_network_id_{}", row),
                &format!("test_network_address_{}", row),
                &vec![0],
            )
            .await
            .unwrap();
            users::add(
                &mut transaction,
                &format!("test_user_id_{}", row),
                &format!("test_network_id_{}", row),
            )
            .await
            .unwrap();
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
        networks::upsert(
            &mut transaction,
            &format!("test_home_network_id"),
            &format!("test_home_address"),
            &vec![0],
        )
        .await
        .unwrap();
        users::add(
            &mut transaction,
            &format!("test_user_id_0"),
            &format!("test_home_network_id"),
        )
        .await
        .unwrap();
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        for row in 0..num_rows {
            networks::upsert(
                &mut transaction,
                &format!("test_network_id_{}", row),
                &format!("test_network_address_{}", row),
                &vec![0],
            )
            .await
            .unwrap();
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

        for i in 0..num_rows {
            assert!(res.contains(&format!("test_network_id_{}", i)));
        }

        transaction.commit().await.unwrap();
    }

    #[tokio::test]
    async fn test_remove() {
        let (pool, _dir) = init().await;
        let num_rows = 10;

        let mut transaction = pool.begin().await.unwrap();
        networks::upsert(
            &mut transaction,
            &format!("test_home_network_id"),
            &format!("test_home_address"),
            &vec![0],
        )
        .await
        .unwrap();
        users::add(
            &mut transaction,
            &format!("test_user_id_0"),
            &format!("test_home_network_id"),
        )
        .await
        .unwrap();
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        for row in 0..num_rows {
            networks::upsert(
                &mut transaction,
                &format!("test_network_id_{}", row),
                &format!("test_network_address_{}", row),
                &vec![0],
            )
            .await
            .unwrap();
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

    #[tokio::test]
    async fn test_add_without_foreign_key_fail() {
        let (pool, _dir) = init().await;

        let mut transaction = pool.begin().await.unwrap();
        assert!(backups::add(
            &mut transaction,
            &format!("test_user_id"),
            &format!("test_networ_id"),
        )
        .await
        .is_err());
    }
}
