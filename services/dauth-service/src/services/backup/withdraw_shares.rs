use std::sync::Arc;

use crate::data::{context::DauthContext, error::DauthError};
use crate::database;

/// Removes all key shares.
/// On failure, removes none.
pub async fn withdraw_shares(
    context: Arc<DauthContext>,
    xres_star_hashs: Vec<&auth_vector::types::XResStarHash>,
) -> Result<(), DauthError> {
    tracing::info!("Handling key shares remove: {:?}", xres_star_hashs,);

    let mut transaction = context.local_context.database_pool.begin().await?;
    for xres_star_hash in xres_star_hashs {
        database::key_shares::remove(&mut transaction, xres_star_hash).await?;
    }
    transaction.commit().await?;
    Ok(())
}
