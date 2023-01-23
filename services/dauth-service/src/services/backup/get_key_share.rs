use std::sync::Arc;

use crate::data::{context::DauthContext, error::DauthError, keys};
use crate::database;

/// Returns a key share value corresponding to the xres* hash.
pub async fn get_key_share_5g(
    context: Arc<DauthContext>,
    xres_star_hash: &auth_vector::types::XResStarHash,
    signed_request_bytes: &Vec<u8>,
) -> Result<keys::CombinedKeyShare, DauthError> {
    tracing::info!("Handling 5G key share get: {:?}", xres_star_hash,);

    // TODO(matt9j) Should get the rand as part of the key share to validate the hashes...
    let mut transaction = context.local_context.database_pool.begin().await?;

    let key_share =
        database::key_shares::get_from_xres_star_hash(&mut transaction, xres_star_hash).await?;

    let user_id = database::key_shares::get_user_id(&mut transaction, xres_star_hash).await?;

    // Remove the auth vectors at the point we have confirmed they were used.
    database::flood_vectors::remove(&mut transaction, &user_id, xres_star_hash).await?;
    database::auth_vectors::remove(&mut transaction, &user_id, xres_star_hash).await?;

    database::tasks::report_key_shares::add(
        &mut transaction,
        xres_star_hash,
        &user_id,
        signed_request_bytes,
    )
    .await?;

    transaction.commit().await?;

    Ok(key_share)
}

/// Returns a key share value corresponding to the xres hash.
pub async fn get_key_share_eps(
    context: Arc<DauthContext>,
    xres_hash: &auth_vector::types::XResHash,
    signed_request_bytes: &Vec<u8>,
) -> Result<keys::CombinedKeyShare, DauthError> {
    tracing::info!(?xres_hash, "Handling EPS key share get");

    // TODO(matt9j) Should get the rand as part of the key share to validate the hashes...
    let mut transaction = context.local_context.database_pool.begin().await?;

    let key_share = database::key_shares::get_from_xres_hash(&mut transaction, xres_hash).await?;

    let user_id =
        database::key_shares::get_user_id(&mut transaction, &key_share.xres_star_hash).await?;

    // Remove the auth vectors at the point we have confirmed they were used.
    database::flood_vectors::remove(&mut transaction, &user_id, &key_share.xres_star_hash).await?;
    database::auth_vectors::remove(&mut transaction, &user_id, &key_share.xres_star_hash).await?;

    database::tasks::report_key_shares::add(
        &mut transaction,
        &key_share.xres_star_hash,
        &user_id,
        signed_request_bytes,
    )
    .await?;

    transaction.commit().await?;

    Ok(key_share)
}
