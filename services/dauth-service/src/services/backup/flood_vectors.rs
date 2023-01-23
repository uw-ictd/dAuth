use std::sync::Arc;

use crate::data::{context::DauthContext, error::DauthError, vector::AuthVectorRes};
use crate::database;

// Store a new flood vector as a backup.
// Will be used before any normal auth vectors.
pub async fn flood_vectors(
    context: Arc<DauthContext>,
    av_result: &AuthVectorRes,
) -> Result<(), DauthError> {
    tracing::info!("Storing flood vector: {:?}", av_result);

    let mut transaction = context.local_context.database_pool.begin().await?;

    database::flood_vectors::add(
        &mut transaction,
        &av_result.user_id,
        av_result.seqnum,
        &av_result.xres_star_hash,
        &&av_result.xres_hash,
        &av_result.autn,
        &av_result.rand.as_array(),
    )
    .await?;

    transaction.commit().await?;
    Ok(())
}
