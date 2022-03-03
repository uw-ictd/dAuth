use std::sync::Arc;

use sqlx::sqlite::SqlitePool;
use sqlx::Row;

use auth_vector::types::{Id, Kseaf, ResStar};

use crate::data::{
    context::DauthContext,
    error::DauthError,
    user_info::UserInfo,
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

    let mut transaction = context.local_context.database_pool.begin().await?;

    let row = queries::get_first_vector(&mut transaction, &av_request.user_id).await?;
    queries::remove_vector(
        &mut transaction,
        row.try_get::<&str, &str>("user_id")?,
        row.try_get::<i64, &str>("seqnum")?,
    )
    .await?;

    transaction.commit().await?;

    Ok(AuthVectorRes {
        user_id: String::from(row.try_get::<&str, &str>("user_id")?),
        seqnum: row.try_get::<i64, &str>("seqnum")?,
        xres_star_hash: row.try_get::<&[u8], &str>("xres_star_hash")?.try_into()?,
        autn: row.try_get::<&[u8], &str>("autn")?.try_into()?,
        rand: row.try_get::<&[u8], &str>("rand")?.try_into()?,
    })
}

/// Deletes a vector if found.
pub async fn auth_vector_delete(
    context: Arc<DauthContext>,
    av_result: &AuthVectorRes,
) -> Result<(), DauthError> {
    tracing::info!("Database delete: {:?}", av_result);

    let mut transaction = context.local_context.database_pool.begin().await?;
    queries::remove_vector(&mut transaction, &av_result.user_id, av_result.seqnum).await?;
    transaction.commit().await?;

    Ok(())
}

/// Removes and returns a kseaf value.
pub async fn kseaf_get(
    context: Arc<DauthContext>,
    xres_star: &ResStar,
) -> Result<Kseaf, DauthError> {
    tracing::info!("Kseaf get: {:?}", xres_star);

    let mut transaction = context.local_context.database_pool.begin().await?;
    let row = queries::get_kseaf(&mut transaction, xres_star).await?;
    queries::delete_kseaf(&mut transaction, xres_star).await?;
    transaction.commit().await?;

    Ok(row.try_get::<&[u8], &str>("kseaf_data")?.try_into()?)
}

/// Adds a kseaf value with the given xres_star_hash.
pub async fn kseaf_put(
    context: Arc<DauthContext>,
    xres_star: &ResStar,
    kseaf: &Kseaf,
) -> Result<(), DauthError> {
    tracing::info!("Kseaf put: {:?} - {:?}", xres_star, kseaf);

    let mut transaction = context.local_context.database_pool.begin().await?;
    queries::insert_kseaf(&mut transaction, xres_star, kseaf).await?;
    transaction.commit().await?;

    Ok(())
}

pub async fn user_info_add(
    context: Arc<DauthContext>,
    user_id: &Id,
    user_info: &UserInfo,
) -> Result<(), DauthError> {
    tracing::info!("User info add: {:?} - {:?}", user_id, user_info);

    let mut transaction = context.local_context.database_pool.begin().await?;
    queries::user_info_upsert(
        &mut transaction,
        user_id,
        &user_info.k,
        &user_info.opc,
        &user_info.sqn_max,
    )
    .await?;
    transaction.commit().await?;

    Ok(())
}

pub async fn user_info_get(
    context: Arc<DauthContext>,
    user_id: &Id,
) -> Result<UserInfo, DauthError> {
    tracing::info!("User info get: {:?}", user_id);

    let mut transaction = context.local_context.database_pool.begin().await?;
    let row = queries::user_info_get(&mut transaction, user_id).await?;
    transaction.commit().await?;

    Ok(UserInfo {
        k: row.try_get::<&[u8], &str>("user_info_k")?.try_into()?,
        opc: row.try_get::<&[u8], &str>("user_info_opc")?.try_into()?,
        sqn_max: row
            .try_get::<&[u8], &str>("user_info_sqn_max")?
            .try_into()?,
    })
}

pub async fn user_info_remove(context: Arc<DauthContext>, user_id: &Id) -> Result<(), DauthError> {
    tracing::info!("User info remove: {:?}", user_id);

    let mut transaction = context.local_context.database_pool.begin().await?;
    queries::user_info_remove(&mut transaction, user_id).await?;
    transaction.commit().await?;

    Ok(())
}
