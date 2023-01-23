use std::sync::Arc;

use crate::data::{context::DauthContext, error::DauthError, keys, vector::AuthVectorRes};
use crate::database;

pub async fn enroll_backup_commit(
    context: Arc<DauthContext>,
    user_id: &str,
    auth_vectors: Vec<AuthVectorRes>,
    key_shares: Vec<keys::CombinedKeyShare>,
) -> Result<(), DauthError> {
    tracing::info!("Storing auth vectors: {:?}", auth_vectors);

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

    tracing::info!("Handling multiple key store: {:?}", key_shares);

    let mut transaction = context.local_context.database_pool.begin().await?;
    for share in key_shares {
        database::key_shares::add(&mut transaction, user_id, &share).await?;
    }
    transaction.commit().await?;

    Ok(())
}
