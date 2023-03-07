use std::sync::Arc;

use crate::data::{context::DauthContext, error::DauthError, keys, vector::AuthVectorRes};
use crate::database;

/// Confirms backup enrollment of this node for the provided user.
/// Optionally stores a set of auth vectors and key shares for the user.
#[tracing::instrument(
    skip(context, auth_vectors, key_shares),
    name = "backup::enroll_backup_commit"
)]
pub async fn enroll_backup_commit(
    context: Arc<DauthContext>,
    user_id: &str,
    auth_vectors: Vec<AuthVectorRes>,
    key_shares: Vec<keys::CombinedKeyShare>,
) -> Result<(), DauthError> {
    tracing::info!("Committing backup enrollment");

    tracing::debug!("Storing auth vectors: {:?}", auth_vectors);

    let mut transaction = context.local_context.database_pool.begin().await?;
    for av in auth_vectors {
        database::auth_vectors::add(
            &mut transaction,
            &av.user_id,
            av.seqnum,
            &av.xres_star_hash,
            &av.xres_hash,
            &av.autn,
            &av.rand.as_array(),
        )
        .await?;
    }
    transaction.commit().await?;

    tracing::debug!("Storing key shares: {:?}", key_shares);

    let mut transaction = context.local_context.database_pool.begin().await?;
    for share in key_shares {
        database::key_shares::add(&mut transaction, user_id, &share).await?;
    }
    transaction.commit().await?;

    Ok(())
}
