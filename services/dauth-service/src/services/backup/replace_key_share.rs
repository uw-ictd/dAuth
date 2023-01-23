use std::sync::Arc;

use crate::data::{
    context::DauthContext, error::DauthError, keys,
};
use crate::database;

/// Replace the old key share if found.
/// Adds the new key share.
pub async fn replace_key_share(
    context: Arc<DauthContext>,
    old_xres_star_hash: &auth_vector::types::XResStarHash,
    new_key_share: &keys::CombinedKeyShare,
) -> Result<(), DauthError> {
    tracing::info!(
        "Replacing key share: {:?} => {:?}",
        old_xres_star_hash,
        new_key_share,
    );

    let mut transaction = context.local_context.database_pool.begin().await?;

    let user_id = database::key_shares::get_user_id(&mut transaction, old_xres_star_hash).await?;
    database::key_shares::remove(&mut transaction, old_xres_star_hash).await?;
    database::key_shares::add(&mut transaction, &user_id, new_key_share).await?;

    transaction.commit().await?;

    Ok(())
}
