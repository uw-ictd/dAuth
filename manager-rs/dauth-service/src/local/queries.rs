use sqlx::sqlite::{SqlitePool, SqliteRow};
use sqlx::{Sqlite, Transaction};

use crate::data::{database::*, error::DauthError};

pub async fn init_vector(pool: &SqlitePool) -> Result<(), DauthError> {
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

pub async fn init_kseaf(pool: &SqlitePool) -> Result<(), DauthError> {
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
