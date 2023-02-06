use std::sync::Arc;

use crate::data::{context::DauthContext, error::DauthError};
use crate::database;

/// Removes all key shares of the corresponding xres* hash values.
/// On failure, none of the key shares will be removed.
#[tracing::instrument(skip(context), name = "backup::withdraw_shares")]
pub async fn withdraw_shares(
    context: Arc<DauthContext>,
    xres_star_hashs: Vec<&auth_vector::types::XResStarHash>,
) -> Result<(), DauthError> {
    tracing::info!("Withdrawing key shares");

    let mut transaction = context.local_context.database_pool.begin().await?;
    for xres_star_hash in xres_star_hashs {
        database::key_shares::remove(&mut transaction, xres_star_hash).await?;
    }
    transaction.commit().await?;
    Ok(())
}
