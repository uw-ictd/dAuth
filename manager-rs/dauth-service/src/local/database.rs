use std::sync::Arc;

use sqlx::sqlite::SqlitePool;
use sqlx::Row;

use auth_vector::types::{Kseaf, ResStar, Id, Sqn, K};

use crate::data::{
    context::DauthContext,
    database::*,
    error::DauthError,
    vector::{AuthVectorReq, AuthVectorRes}, user_info::UserInfo,
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
pub async fn kseaf_get(
    context: Arc<DauthContext>,
    xres_star: &ResStar,
) -> Result<Kseaf, DauthError> {
    tracing::info!("Kseaf get: {:?}", xres_star);

    let mut transaction = context.database_context.pool.begin().await?;
    let row = queries::get_kseaf(&mut transaction, xres_star).await?;
    queries::delete_kseaf(&mut transaction, xres_star).await?;
    transaction.commit().await?;

    Ok(row.try_get::<&[u8], &str>(KSEAF_DATA_FIELD)?.try_into()?)
}

/// Adds a kseaf value with the given xres_star_hash.
pub async fn kseaf_put(context: Arc<DauthContext>, xres_star: &ResStar, kseaf: &Kseaf) -> Result<(), DauthError> {
    tracing::info!("Kseaf put: {:?} - {:?}", xres_star, kseaf);

    let mut transaction = context.database_context.pool.begin().await?;
    queries::insert_kseaf(&mut transaction, xres_star, kseaf).await?;
    transaction.commit().await?;

    Ok(())
}


pub async fn user_info_add(
    context: Arc<DauthContext>,
    user_id: Id,
    user_info: UserInfo,
) -> Result<(), DauthError> {
    tracing::info!("User info add: {:?} - {:?}", user_id, user_info);

    let mut transaction = context.database_context.pool.begin().await?;
    queries::user_info_add(&mut transaction, user_id, &user_info.k, &user_info.opc, &user_info.sqn_max).await?;
    transaction.commit().await?;


    Ok(())
}

pub async fn user_info_get(
    context: Arc<DauthContext>,
    user_id: Id,
) -> Result<UserInfo, DauthError> {
    tracing::info!("User info get: {:?}", user_id);

    let mut transaction = context.database_context.pool.begin().await?;
    let row = queries::user_info_get(&mut transaction, user_id).await?;
    transaction.commit().await?;

    Ok(UserInfo {
        k: row.try_get::<&[u8], &str>(USER_INFO_K_FIELD)?.try_into()?,
        opc: row.try_get::<&[u8], &str>(USER_INFO_OPC_FIELD)?.try_into()?,
        sqn_max: row.try_get::<&[u8], &str>(USER_INFO_SQN_FIELD)?.try_into()?,
    })
}

pub async fn user_info_remove(
    context: Arc<DauthContext>,
    user_id: Id,
) -> Result<(), DauthError> {
    tracing::info!("User info remove: {:?}", user_id);

    let mut transaction = context.database_context.pool.begin().await?;
    queries::user_info_remove(&mut transaction, user_id).await?;
    transaction.commit().await?;

    Ok(())
}
