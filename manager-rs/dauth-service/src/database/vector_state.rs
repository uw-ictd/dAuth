use sqlx::sqlite::SqlitePool;
use sqlx::{Row, Sqlite, Transaction};

use crate::data::error::DauthError;

/// Creates the vector state table if it does not exist already.
pub async fn init_table(pool: &SqlitePool) -> Result<(), DauthError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS vector_state_table (
            xres_star_hash BLOB PRIMARY KEY,
            user_id TEXT NOT NULL,
            backup_network_id TEXT NOT NULL
        );",
    )
    .execute(pool)
    .await?;
    Ok(())
}

/* Queries */

/// Adds the auth vector as owned by the backup network.
/// Use xres* hash as the reference for the auth vector.
pub async fn add(
    transaction: &mut Transaction<'_, Sqlite>,
    xres_star_hash: &[u8],
    user_id: &str,
    backup_network_id: &str,
) -> Result<(), DauthError> {
    sqlx::query(
        "INSERT INTO vector_state_table
        VALUES ($1,$2,$3)",
    )
    .bind(xres_star_hash)
    .bind(user_id)
    .bind(backup_network_id)
    .execute(transaction)
    .await?;

    Ok(())
}

/// Returns the owning network and user id of the auth vector.
pub async fn get(
    transaction: &mut Transaction<'_, Sqlite>,
    xres_star_hash: &[u8],
) -> Result<(String, String), DauthError> {
    let row = sqlx::query(
        "SELECT * FROM vector_state_table
        WHERE xres_star_hash=$1;",
    )
    .bind(xres_star_hash)
    .fetch_one(transaction)
    .await?;

    Ok((
        row.try_get::<String, &str>("backup_network_id")?,
        row.try_get::<String, &str>("user_id")?,
    ))
}

/// Returns the set of xres* hashes owned by the network for a given user.
pub async fn get_all_by_id(
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &str,
    backup_network_id: &str,
) -> Result<Vec<Vec<u8>>, DauthError> {
    let res = sqlx::query(
        "SELECT * FROM vector_state_table
        WHERE (user_id,backup_network_id)=($1,$2);",
    )
    .bind(user_id)
    .bind(backup_network_id)
    .fetch_all(transaction)
    .await?;

    let mut hashes = Vec::with_capacity(res.len());
    for row in res {
        hashes.push(row.try_get::<Vec<u8>, &str>("xres_star_hash")?)
    }
    Ok(hashes)
}

/// Deletes a vector reference if found.
pub async fn remove(
    transaction: &mut Transaction<'_, Sqlite>,
    xres_star_hash: &[u8],
) -> Result<(), DauthError> {
    sqlx::query(
        "DELETE FROM vector_state_table
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
    use sqlx::SqlitePool;
    use tempfile::{tempdir, TempDir};

    use crate::database::{general, vector_state};

    fn gen_name() -> String {
        let s: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();

        format!("sqlite_{}.db", s)
    }

    async fn init() -> (SqlitePool, TempDir) {
        let dir = tempdir().unwrap();
        let path = String::from(dir.path().join(gen_name()).to_str().unwrap());
        println!("Building temporary db: {}", path);

        let pool = general::build_pool(&path).await.unwrap();
        vector_state::init_table(&pool).await.unwrap();

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
            vector_state::add(
                &mut transaction,
                &[row as u8; 1],
                "test_user_id",
                &format!("test_backup_network_{}", row),
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
            vector_state::add(
                &mut transaction,
                &[row as u8; 1],
                "test_user_id",
                &format!("test_backup_network_{}", row),
            )
            .await
            .unwrap();
        }
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        for row in 0..num_rows {
            assert_eq!(
                vector_state::get(&mut transaction, &[row as u8; 1],)
                    .await
                    .unwrap()
                    .0,
                format!("test_backup_network_{}", row)
            );
        }
        transaction.commit().await.unwrap();
    }
    #[tokio::test]
    async fn test_get_all() {
        let (pool, _dir) = init().await;
        let num_rows = 10;

        let mut transaction = pool.begin().await.unwrap();
        let res =
            vector_state::get_all_by_id(&mut transaction, "test_user_id", "test_backup_network_id")
                .await
                .unwrap();
        assert_eq!(res.len(), 0);
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        for row in 0..num_rows {
            vector_state::add(
                &mut transaction,
                &[row as u8; 1],
                "test_user_id",
                "test_backup_network_id",
            )
            .await
            .unwrap();
        }
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        let res =
            vector_state::get_all_by_id(&mut transaction, "test_user_id", "test_backup_network_id")
                .await
                .unwrap();
        assert_eq!(res.len(), num_rows);
        transaction.commit().await.unwrap();
    }

    #[tokio::test]
    async fn test_remove() {
        let (pool, _dir) = init().await;
        let num_rows = 10;

        let mut transaction = pool.begin().await.unwrap();
        for row in 0..num_rows {
            vector_state::add(
                &mut transaction,
                &[row as u8; 1],
                "test_user_id",
                &format!("test_backup_network_{}", row),
            )
            .await
            .unwrap();
        }
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        for row in 0..num_rows {
            assert_eq!(
                vector_state::get(&mut transaction, &[row as u8; 1],)
                    .await
                    .unwrap()
                    .0,
                format!("test_backup_network_{}", row)
            );
        }
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        for row in 0..num_rows {
            vector_state::remove(&mut transaction, &[row as u8; 1])
                .await
                .unwrap();
        }
        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();
        for row in 0..num_rows {
            assert!(vector_state::get(&mut transaction, &[row as u8; 1],)
                .await
                .is_err());
        }
        transaction.commit().await.unwrap();
    }
}
