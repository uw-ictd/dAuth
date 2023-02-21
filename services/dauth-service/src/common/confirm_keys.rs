use std::sync::Arc;

use crate::data::{context::DauthContext, error::DauthError};
use crate::database;

/// Gets the Kseaf value for the auth vector from this network.
/// Auth vector must have been generated by this network.
pub async fn get_confirm_key_res_star(
    context: Arc<DauthContext>,
    res_star: auth_vector::types::ResStar,
) -> Result<auth_vector::types::Kseaf, DauthError> {
    tracing::info!(?res_star, "Getting confirm key for res_star");

    let mut transaction = context.local_context.database_pool.begin().await?;

    let kseaf = database::kseafs::get(&mut transaction, &res_star).await?;
    database::kseafs::remove(&mut transaction, &res_star).await?;

    transaction.commit().await?;
    Ok(kseaf)
}

/// Gets the Kasmf value for the auth vector from this network.
/// Auth vector must have been generated by this network.
pub async fn get_confirm_key_res(
    context: Arc<DauthContext>,
    res: auth_vector::types::Res,
) -> Result<auth_vector::types::Kasme, DauthError> {
    tracing::info!(?res, "Getting confirm key for res");

    let mut transaction = context.local_context.database_pool.begin().await?;

    let kasme = database::kasmes::get(&mut transaction, &res).await?;
    database::kasmes::remove(&mut transaction, &res).await?;

    transaction.commit().await?;
    Ok(kasme)
}
