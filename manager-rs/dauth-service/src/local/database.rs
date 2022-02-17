use std::sync::Arc;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use sqlx::Row;

use auth_vector::types::{Kseaf, ResStar};

use crate::data::{
    context::DauthContext,
    database::*,
    error::DauthError,
    vector::{AuthVectorReq, AuthVectorRes},
};

/// Builds the database connection pool.
/// Creates the database and tables if they don't exist.
pub async fn database_init(database_path: &str) -> Result<SqlitePool, DauthError> {
    let pool: SqlitePool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect_with(
            SqliteConnectOptions::new()
                .create_if_missing(true)
                .filename(database_path),
        )
        .await?;

    // TODO (nickfh7) add ranking, i.e. by seqnum
    sqlx::query(&format!(
        "CREATE TABLE IF NOT EXISTS {} (
            {} TEXT NOT NULL,
            {} BLOB NOT NULL,
            {} BLOB NOT NULL,
            {} BLOB NOT NULL
        );",
        AV_TABLE_NAME, AV_ID_FIELD, AV_XRES_FIELD, AV_AUTN_FIELD, AV_RAND_FIELD
    ))
    .execute(&pool)
    .await?;

    sqlx::query(&format!(
        "CREATE TABLE IF NOT EXISTS {} (
            {} INT PRIMARY KEY,
            {} BLOB NOT NULL
        );",
        KSEAF_TABLE_NAME, KSEAF_ID_FIELD, KSEAF_DATA_FIELD
    ))
    .execute(&pool)
    .await?;

    Ok(pool)
}

/// Removes and returns vector if at least one exists.
pub async fn auth_vector_next(
    context: Arc<DauthContext>,
    av_request: &AuthVectorReq,
) -> Result<AuthVectorRes, DauthError> {
    tracing::info!("Database next: {:?}", av_request);

    let mut transaction = context.database_context.pool.begin().await?;

    let row = sqlx::query(&format!(
        "SELECT rowid, * 
        FROM {}
        WHERE {}=$1 
        LIMIT 1;",
        AV_TABLE_NAME, AV_ID_FIELD
    ))
    .bind(&av_request.user_id)
    .fetch_one(&mut transaction)
    .await?;

    sqlx::query(&format!(
        "DELETE FROM {}
        WHERE rowid=$2",
        AV_TABLE_NAME
    ))
    .bind(row.try_get::<i64, &str>("rowid")?)
    .execute(&mut transaction)
    .await?;

    transaction.commit().await?;

    Ok(AuthVectorRes {
        user_id: String::from(row.try_get::<&str, &str>(AV_ID_FIELD)?),
        xres_star_hash: row.try_get::<&[u8], &str>(AV_XRES_FIELD)?.try_into()?,
        autn: row.try_get::<&[u8], &str>(AV_AUTN_FIELD)?.try_into()?,
        rand: row.try_get::<&[u8], &str>(AV_RAND_FIELD)?.try_into()?,
    })
}

/// Deletes a vector if found.
pub fn auth_vector_delete(
    _context: Arc<DauthContext>,
    av_result: &AuthVectorRes,
) -> Result<(), DauthError> {
    tracing::info!("Database delete: {:?}", av_result);
    todo!()
}

/// Removes and returns a kseaf value
pub fn kseaf_get(
    context: Arc<DauthContext>,
    xres_star_hash: &ResStar,
) -> Result<Kseaf, DauthError> {
    tracing::info!("Kseaf get: {:?}", xres_star_hash);

    let mut map = context.local_context.kseaf_map.lock().unwrap();

    match map.get(xres_star_hash) {
        Some(kseaf) => {
            let kseaf = kseaf.clone();
            map.remove(xres_star_hash);
            Ok(kseaf)
        }
        None => {
            tracing::error!("KSEAF not found with UUID: {:?}", xres_star_hash);
            Err(DauthError::NotFoundError(format!(
                "KSEAF not found with UUID: {:?}",
                xres_star_hash
            )))
        }
    }
}

/// Adds a kseaf value with the given xres_star_hash
pub fn kseaf_put(context: Arc<DauthContext>, xres_star: &ResStar, kseaf: &Kseaf) {
    tracing::info!("Kseaf put: {:?} - {:?}", xres_star, kseaf);
    context
        .local_context
        .kseaf_map
        .lock()
        .unwrap()
        .insert(xres_star.clone(), kseaf.clone());
}
