use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions, SqliteRow};
use sqlx::{Sqlite, Transaction};

use crate::data::{database::*, error::DauthError};

pub async fn build_pool(database_path: &str) -> Result<SqlitePool, DauthError> {
    Ok(SqlitePoolOptions::new()
        .max_connections(10)
        .connect_with(
            SqliteConnectOptions::new()
                .create_if_missing(true)
                .filename(database_path),
        )
        .await?)
}

pub async fn init_auth_vector_table(pool: &SqlitePool) -> Result<(), DauthError> {
    sqlx::query(&format!(
        "CREATE TABLE IF NOT EXISTS {0} (
            {1} TEXT NOT NULL,
            {2} INT NOT NULL,
            {3} BLOB NOT NULL,
            {4} BLOB NOT NULL,
            {5} BLOB NOT NULL,
            PRIMARY KEY ({1}, {2})
        );",
        AV_TABLE_NAME, AV_ID_FIELD, AV_RANK_FIELD, AV_XRES_FIELD, AV_AUTN_FIELD, AV_RAND_FIELD
    ))
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn init_kseaf_table(pool: &SqlitePool) -> Result<(), DauthError> {
    sqlx::query(&format!(
        "CREATE TABLE IF NOT EXISTS {} (
            {} INT PRIMARY KEY,
            {} BLOB NOT NULL
        );",
        KSEAF_TABLE_NAME, KSEAF_ID_FIELD, KSEAF_DATA_FIELD
    ))
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_first_vector(
    transaction: &mut Transaction<'_, Sqlite>,
    id: &str,
) -> Result<SqliteRow, DauthError> {
    Ok(sqlx::query(&format!(
        "SELECT * 
        FROM {0}
        WHERE {1}=$1
        ORDER BY {2}
        LIMIT 1;",
        AV_TABLE_NAME, AV_ID_FIELD, AV_RANK_FIELD,
    ))
    .bind(id)
    .fetch_one(transaction)
    .await?)
}

pub async fn insert_vector(
    transaction: &mut Transaction<'_, Sqlite>,
    id: &str,
    seqnum: i64,
    xres: &[u8],
    autn: &[u8],
    rand: &[u8],
) -> Result<(), DauthError> {
    let res = sqlx::query(&format!(
        "INSERT INTO {0}
        VALUES ($1,$2,$3,$4,$5)",
        AV_TABLE_NAME
    ))
    .bind(id)
    .bind(seqnum)
    .bind(xres)
    .bind(autn)
    .bind(rand)
    .execute(transaction)
    .await?;

    Ok(())
}

pub async fn remove_vector(
    transaction: &mut Transaction<'_, Sqlite>,
    id: &str,
    seqnum: i64,
) -> Result<(), DauthError> {
    sqlx::query(&format!(
        "DELETE FROM {}
        WHERE ({},{})=($1,$2)",
        AV_TABLE_NAME, AV_ID_FIELD, AV_RANK_FIELD,
    ))
    .bind(id)
    .bind(seqnum)
    .execute(transaction)
    .await?;
    Ok(())
}

#[cfg(test)]
mod av_tests {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use sqlx::{SqlitePool, Row};
    use tempfile::tempdir;

    use auth_vector::constants::{AUTN_LENGTH, RAND_LENGTH, RES_STAR_HASH_LENGTH};

    use crate::local::queries;
    use crate::data::database::*;

    fn gen_name() -> String {
        let s: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();

        format!("sqlite_{}.db", s)
    }

    async fn init() -> SqlitePool {
        let dir = tempdir().unwrap();
        let path = String::from(dir.path().join(gen_name()).to_str().unwrap());
        println!("Building temporary db: {}", path);

        let pool = queries::build_pool(&path).await.unwrap();
        queries::init_auth_vector_table(&pool).await.unwrap();
        queries::init_kseaf_table(&pool).await.unwrap();

        pool
    }

    /// Test that db and table creation will work
    #[tokio::test]
    async fn test_db_init() {
        init().await;
    }

    /// Test that db can insert
    #[tokio::test]
    async fn test_db_insert() {
        let pool = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                queries::insert_vector(
                    &mut transaction,
                    &format!("test_id_{}", section),
                    row,
                    &[0_u8; RES_STAR_HASH_LENGTH],
                    &[0_u8; AUTN_LENGTH],
                    &[0_u8; RAND_LENGTH],
                )
                .await
                .unwrap();
            }
        }

        transaction.commit().await.unwrap();
    }

    /// Test that duplicate inserts cause an error
    #[tokio::test]
    #[should_panic]
    async fn test_db_insert_dupicate_fail() {
        let pool = init().await;
        let mut transaction = pool.begin().await.unwrap();

        queries::insert_vector(
            &mut transaction,
            "test_id_1",
            1,
            &[0_u8; RES_STAR_HASH_LENGTH],
            &[0_u8; AUTN_LENGTH],
            &[0_u8; RAND_LENGTH],
        )
        .await
        .unwrap();
        
        queries::insert_vector(
            &mut transaction,
            "test_id_1",
            1,
            &[0_u8; RES_STAR_HASH_LENGTH],
            &[0_u8; AUTN_LENGTH],
            &[0_u8; RAND_LENGTH],
        )
        .await
        .unwrap();

        transaction.commit().await.unwrap();
    }

        /// Test that db can delete after inserts
        #[tokio::test]
        async fn test_db_get_first() {
            let pool = init().await;
    
            let mut transaction = pool.begin().await.unwrap();

            queries::insert_vector(
                &mut transaction,
                "test_id_1",
                2,
                &[0_u8; RES_STAR_HASH_LENGTH],
                &[0_u8; AUTN_LENGTH],
                &[0_u8; RAND_LENGTH],
            )
            .await
            .unwrap();

            queries::insert_vector(
                &mut transaction,
                "test_id_1",
                0,
                &[0_u8; RES_STAR_HASH_LENGTH],
                &[0_u8; AUTN_LENGTH],
                &[0_u8; RAND_LENGTH],
            )
            .await
            .unwrap();

            queries::insert_vector(
                &mut transaction,
                "test_id_1",
                1,
                &[0_u8; RES_STAR_HASH_LENGTH],
                &[0_u8; AUTN_LENGTH],
                &[0_u8; RAND_LENGTH],
            )
            .await
            .unwrap();
    
            transaction.commit().await.unwrap();

            let mut transaction = pool.begin().await.unwrap();
    
            let res = queries::get_first_vector(
                &mut transaction,
                "test_id_1",
            )
            .await
            .unwrap();
    
            assert_eq!("test_id_1", res.get_unchecked::<&str, &str>(AV_ID_FIELD));
            assert_eq!(0, res.get_unchecked::<i64, &str>(AV_RANK_FIELD));
    
            transaction.commit().await.unwrap();
        }

    /// Test that db can delete after inserts
    #[tokio::test]
    async fn test_db_delete() {
        let pool = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                queries::insert_vector(
                    &mut transaction,
                    &format!("test_id_{}", section),
                    row,
                    &[0_u8; RES_STAR_HASH_LENGTH],
                    &[0_u8; AUTN_LENGTH],
                    &[0_u8; RAND_LENGTH],
                )
                .await
                .unwrap();
            }
        }

        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                queries::remove_vector(
                    &mut transaction,
                    &format!("test_id_{}", section),
                    row,
                )
                .await
                .unwrap();
            }
        }

        transaction.commit().await.unwrap();
    }

    /// Test that db can delete after inserts
    #[tokio::test]
    async fn test_db_get_first_with_delete() {
        let pool = init().await;

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                queries::insert_vector(
                    &mut transaction,
                    &format!("test_id_{}", section),
                    row,
                    &[0_u8; RES_STAR_HASH_LENGTH],
                    &[0_u8; AUTN_LENGTH],
                    &[0_u8; RAND_LENGTH],
                )
                .await
                .unwrap();
            }
        }

        transaction.commit().await.unwrap();

        let mut transaction = pool.begin().await.unwrap();

        let num_rows = 10;
        let num_sections = 10;

        for section in 0..num_sections {
            for row in 0..num_rows {
                let res = queries::get_first_vector(
                    &mut transaction,
                    &format!("test_id_{}", section)
                )
                .await
                .unwrap();

                queries::remove_vector(
                    &mut transaction,
                    &format!("test_id_{}", section),
                    row,
                )
                .await
                .unwrap();

                assert_eq!(&format!("test_id_{}", section), res.get_unchecked::<&str, &str>(AV_ID_FIELD));
                assert_eq!(row, res.get_unchecked::<i64, &str>(AV_RANK_FIELD));
            }
        }

        transaction.commit().await.unwrap();
    }
}
