use std::sync::Arc;

use crate::data::{
    context::DauthContext,
    error::DauthError,
    keys,
    vector::AuthVectorRes,
};
use crate::database;

/// Store all auth vectors in the set.
/// Stores all or none on failure.
pub async fn store_backup_auth_vectors(
    context: Arc<DauthContext>,
    av_results: Vec<AuthVectorRes>,
) -> Result<(), DauthError> {
    tracing::info!("Storing auth vectors: {:?}", av_results);

    let mut transaction = context.local_context.database_pool.begin().await?;

    for av_result in av_results {
        database::auth_vectors::add(
            &mut transaction,
            &av_result.user_id,
            av_result.seqnum,
            &av_result.xres_star_hash,
            &av_result.xres_hash,
            &av_result.autn,
            &av_result.rand.as_array(),
        )
        .await?;
    }

    transaction.commit().await?;
    Ok(())
}

/// Stores a collection of key shares.
pub async fn store_key_shares(
    context: Arc<DauthContext>,
    user_id: &str,
    key_shares: Vec<keys::CombinedKeyShare>,
) -> Result<(), DauthError> {
    tracing::info!("Handling multiple key store: {:?}", key_shares);

    let mut transaction = context.local_context.database_pool.begin().await?;

    for share in key_shares {
        database::key_shares::add(&mut transaction, user_id, &share).await?;
    }
    transaction.commit().await?;
    Ok(())
}
