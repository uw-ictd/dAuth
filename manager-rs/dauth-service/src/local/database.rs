use std::sync::Arc;

use sqlx::sqlite::SqlitePool;
use sqlx::Row;

use auth_vector::types::{Kseaf, ResStar};

use crate::data::{
    context::DauthContext,
    database::*,
    error::DauthError,
    vector::{AuthVectorReq, AuthVectorRes},
};

use crate::local::queries;

/// Builds the database connection pool.
/// Creates the database and tables if they don't exist.
pub async fn database_init(database_path: &str) -> Result<SqlitePool, DauthError> {
    let pool: SqlitePool = queries::build_pool(database_path).await?;

    queries::init_auth_vector_table(&pool).await?;
    queries::init_kseaf_table(&pool).await?;

    Ok(pool)
}

/// Removes and returns vector if at least one exists.
pub async fn auth_vector_next(
    context: Arc<DauthContext>,
    av_request: &AuthVectorReq,
) -> Result<AuthVectorRes, DauthError> {
    tracing::info!("Database next: {:?}", av_request);

    let mut transaction = context.database_context.pool.begin().await?;

    let row = queries::get_first_vector(&mut transaction, &av_request.user_id).await?;
    queries::remove_vector(
        &mut transaction,
        row.try_get::<&str, &str>(AV_ID_FIELD)?,
        row.try_get::<i64, &str>(AV_RANK_FIELD)?,
    )
    .await?;

    transaction.commit().await?;

    Ok(AuthVectorRes {
        user_id: String::from(row.try_get::<&str, &str>(AV_ID_FIELD)?),
        seqnum: row.try_get::<i64, &str>(AV_RANK_FIELD)?,
        xres_star_hash: row.try_get::<&[u8], &str>(AV_XRES_FIELD)?.try_into()?,
        autn: row.try_get::<&[u8], &str>(AV_AUTN_FIELD)?.try_into()?,
        rand: row.try_get::<&[u8], &str>(AV_RAND_FIELD)?.try_into()?,
    })
}

/// Deletes a vector if found.
pub async fn auth_vector_delete(
    context: Arc<DauthContext>,
    av_result: &AuthVectorRes,
) -> Result<(), DauthError> {
    tracing::info!("Database delete: {:?}", av_result);

    let mut transaction = context.database_context.pool.begin().await?;
    queries::remove_vector(&mut transaction, &av_result.user_id, av_result.seqnum).await?;
    transaction.commit().await?;

    Ok(())
}

/// Removes and returns a kseaf value.
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

/// Adds a kseaf value with the given xres_star_hash.
pub fn kseaf_put(context: Arc<DauthContext>, xres_star: &ResStar, kseaf: &Kseaf) {
    tracing::info!("Kseaf put: {:?} - {:?}", xres_star, kseaf);
    context
        .local_context
        .kseaf_map
        .lock()
        .unwrap()
        .insert(xres_star.clone(), kseaf.clone());
}
