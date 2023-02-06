use std::sync::Arc;

use crate::data::{context::DauthContext, error::DauthError, keys};
use crate::database;

/// Adds a new key share, replacing the indicated old key share
/// if it hasn't already been used and removed.
#[tracing::instrument(skip(context), name = "backup::replace_key_share")]
pub async fn replace_key_share(
    context: Arc<DauthContext>,
    old_xres_star_hash: &auth_vector::types::XResStarHash,
    new_key_share: &keys::CombinedKeyShare,
) -> Result<(), DauthError> {
    tracing::info!("Replacing key share");

    let mut transaction = context.local_context.database_pool.begin().await?;

    let user_id = database::key_shares::get_user_id(&mut transaction, old_xres_star_hash).await?;
    database::key_shares::remove(&mut transaction, old_xres_star_hash).await?;
    database::key_shares::add(&mut transaction, &user_id, new_key_share).await?;

    transaction.commit().await?;

    Ok(())
}
