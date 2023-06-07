use sqlx::sqlite::SqlitePool;
use sqlx::Row;
use sqlx::{Sqlite, Transaction};

use crate::data::error::DauthError;

/// Creates the update user table if it does not exist already.
#[tracing::instrument(skip(pool), name = "database::tasks::update_users")]
pub async fn init_table(pool: &SqlitePool) -> Result<(), DauthError> {
    tracing::info!("Initialzing table");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS task_update_users_table (
            user_id TEXT NOT NULL,
            sqn_slice INT NOT NULL,
            backup_network_id INT NOT NULL,
            withdraw_first INT NOT NULL,
            PRIMARY KEY (user_id, sqn_slice),
            FOREIGN KEY (user_id, sqn_slice) 
                REFERENCES user_info_table(id, sqn_slice)
        );",
    )
    .execute(pool)
    .await?;
    Ok(())
}

/* Queries */

/// Adds a user id with a set of backup network ids.
/// Withrdaw first expects 1 (True) or 0 (False).
#[tracing::instrument(skip(transaction), name = "database::tasks::update_users")]
pub async fn add(
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &str,
    sqn_slice: i64,
    backup_network_id: &str,
    withdraw_first: i64,
) -> Result<(), DauthError> {
    tracing::debug!("Adding task");

    sqlx::query(
        "REPLACE INTO task_update_users_table
        VALUES ($1,$2,$3,$4)",
    )
    .bind(user_id)
    .bind(sqn_slice)
    .bind(backup_network_id)
    .bind(withdraw_first)
    .execute(&mut *transaction)
    .await?;

    Ok(())
}

/// Gets all user ids
#[tracing::instrument(skip(transaction), name = "database::tasks::update_users")]
pub async fn get_user_ids(
    transaction: &mut Transaction<'_, Sqlite>,
) -> Result<Vec<String>, DauthError> {
    tracing::debug!("Getting all user ids");

    let mut result = Vec::new();
    let rows = sqlx::query("SELECT DISTINCT user_id FROM task_update_users_table")
        .fetch_all(transaction)
        .await?;

    for row in rows {
        result.push(row.try_get::<String, &str>("user_id")?);
    }
    Ok(result)
}

/// Gets all backup network ids and sqn slices for a given user id.
#[tracing::instrument(skip(transaction), name = "database::tasks::update_users")]
pub async fn get_user_data(
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &str,
) -> Result<Vec<(String, i64)>, DauthError> {
    tracing::debug!("Getting user data");

    let mut result = Vec::new();
    let rows = sqlx::query(
        "SELECT * FROM task_update_users_table
        WHERE user_id=$1;",
    )
    .bind(user_id)
    .fetch_all(transaction)
    .await?;

    for row in rows {
        result.push((
            row.try_get::<String, &str>("backup_network_id")?,
            row.try_get::<i64, &str>("sqn_slice")?,
        ));
    }
    Ok(result)
}

/// Returns true if this user/backup task needs to trigger a withdraw first.
#[tracing::instrument(skip(transaction), name = "database::tasks::update_users")]
pub async fn withdraw_first(
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &str,
    backup_network_id: &str,
) -> Result<bool, DauthError> {
    tracing::debug!("Checking withdraw first");

    let row = sqlx::query(
        "SELECT * FROM task_update_users_table
        WHERE (user_id,backup_network_id)=($1,$2);",
    )
    .bind(user_id)
    .bind(backup_network_id)
    .fetch_one(transaction)
    .await?;

    Ok(row.try_get::<i64, &str>("withdraw_first")? != 0)
}

/// Removes a user id and all its backup network ids.
#[tracing::instrument(skip(transaction), name = "database::tasks::update_users")]
pub async fn remove(
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &str,
) -> Result<(), DauthError> {
    tracing::debug!("Removing task");

    sqlx::query(
        "DELETE FROM task_update_users_table
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

    use crate::database::{general, tasks, user_infos};

    fn gen_name() -> String {
        let s: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();

        format!("sqlite_{}.db", s)
    }

    async fn init() -> (SqlitePool, TempDir) {
        let dir = tempdir().unwrap();
        let path = String::from(dir.path().join(gen_name()).to_str().unwrap());
        println!("Building temporary db: {}", path);

        let pool = general::build_pool(&path).await.unwrap();
        tasks::update_users::init_table(&pool).await.unwrap();
        user_infos::init_table(&pool).await.unwrap();

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
            for sqn_max in 0..3 {
                user_infos::upsert(
                    &mut transaction,
                    &format!("test_user_id_{}", row),
                    &[0u8, 3],
                    &[0u8, 3],
                    sqn_max,
                    sqn_max,
                )
                .await
                .unwrap();
            }

            tasks::update_users::add(
                &mut transaction,
                &format!("test_user_id_{}", row),
                0,
                "test_network_id_a",
                0,
            )
            .await
            .unwrap();
            tasks::update_users::add(
                &mut transaction,
                &format!("test_user_id_{}", row),
                1,
                "test_network_id_b",
                0,
            )
            .await
            .unwrap();
            tasks::update_users::add(
                &mut transaction,
                &format!("test_user_id_{}", row),
                2,
                "test_network_id_c",
                0,
            )
            .await
            .unwrap();
        }
        transaction.commit().await.unwrap();
    }

    #[tokio::test]
    async fn test_withdraw_first() {
        let (pool, _dir) = init().await;
        let num_rows = 10;

        let mut transaction = pool.begin().await.unwrap();
        for row in 0..num_rows {
            for sqn_max in 0..3 {
                user_infos::upsert(
                    &mut transaction,
                    &format!("test_user_id_{}", row),
                    &[0u8, 3],
                    &[0u8, 3],
                    sqn_max,
                    sqn_max,
                )
                .await
                .unwrap();
            }

            tasks::update_users::add(
                &mut transaction,
                &format!("test_user_id_{}", row),
                0,
                "test_network_id_a",
                0,
            )
            .await
            .unwrap();
            tasks::update_users::add(
                &mut transaction,
                &format!("test_user_id_{}", row),
                1,
                "test_network_id_b",
                1,
            )
            .await
            .unwrap();
        }
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        for row in 0..num_rows {
            assert!(!tasks::update_users::withdraw_first(
                &mut transaction,
                &format!("test_user_id_{}", row),
                "test_network_id_a",
            )
            .await
            .unwrap());
            assert!(tasks::update_users::withdraw_first(
                &mut transaction,
                &format!("test_user_id_{}", row),
                "test_network_id_b",
            )
            .await
            .unwrap());
        }
        transaction.commit().await.unwrap();
    }

    #[tokio::test]
    async fn test_get() {
        let (pool, _dir) = init().await;

        let mut transaction = pool.begin().await.unwrap();
        assert!(tasks::update_users::get_user_ids(&mut transaction)
            .await
            .unwrap()
            .is_empty());
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        for sqn_max in 0..3 {
            user_infos::upsert(
                &mut transaction,
                &"test_user_id".to_string(),
                &[0u8, 3],
                &[0u8, 3],
                sqn_max,
                sqn_max,
            )
            .await
            .unwrap();
        }

        tasks::update_users::add(&mut transaction, "test_user_id", 0, "test_network_id_a", 0)
            .await
            .unwrap();
        tasks::update_users::add(&mut transaction, "test_user_id", 1, "test_network_id_b", 0)
            .await
            .unwrap();
        tasks::update_users::add(&mut transaction, "test_user_id", 2, "test_network_id_c", 0)
            .await
            .unwrap();
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        let user_id = &tasks::update_users::get_user_ids(&mut transaction)
            .await
            .unwrap()[0];
        assert_eq!(user_id, "test_user_id");
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        tasks::update_users::get_user_data(&mut transaction, user_id)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_remove() {
        let (pool, _dir) = init().await;
        let num_rows = 10;

        let mut transaction = pool.begin().await.unwrap();
        for row in 0..num_rows {
            for sqn_max in 0..3 {
                user_infos::upsert(
                    &mut transaction,
                    &format!("test_user_id_{}", row),
                    &[0u8, 3],
                    &[0u8, 3],
                    sqn_max,
                    sqn_max,
                )
                .await
                .unwrap();
            }

            tasks::update_users::add(
                &mut transaction,
                &format!("test_user_id_{}", row),
                0,
                "test_network_id_a",
                0,
            )
            .await
            .unwrap();
            tasks::update_users::add(
                &mut transaction,
                &format!("test_user_id_{}", row),
                1,
                "test_network_id_b",
                0,
            )
            .await
            .unwrap();
            tasks::update_users::add(
                &mut transaction,
                &format!("test_user_id_{}", row),
                2,
                "test_network_id_c",
                0,
            )
            .await
            .unwrap();
        }
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        assert!(!tasks::update_users::get_user_ids(&mut transaction)
            .await
            .unwrap()
            .is_empty());
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        for user_id in tasks::update_users::get_user_ids(&mut transaction)
            .await
            .unwrap()
        {
            tasks::update_users::remove(&mut transaction, &user_id)
                .await
                .unwrap();
        }
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        assert!(tasks::update_users::get_user_ids(&mut transaction)
            .await
            .unwrap()
            .is_empty());
        transaction.commit().await.unwrap();
    }
}
