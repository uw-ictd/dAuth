use std::sync::Arc;

use crate::data::combined_res::XResHashKind;
use crate::data::{context::DauthContext, error::DauthError, keys};
use crate::database;

/// Returns a key share value corresponding to the xres hash or
/// xres* hash, depending on the authentication type (5G or 4G/EPS).
/// The key share should correspond to an existing auth vector that
/// was previously sent.
#[tracing::instrument(skip(context), name = "backup::get_key_share")]
pub async fn get_key_share(
    context: Arc<DauthContext>,
    combined_hash: &XResHashKind,
    signed_request_bytes: &Vec<u8>,
) -> Result<keys::CombinedKeyShare, DauthError> {
    tracing::info!("Getting backup key share");

    // TODO(matt9j) Should get the rand as part of the key share to validate the hashes...
    let mut transaction = context.local_context.database_pool.begin().await?;

    let key_share = match combined_hash {
        XResHashKind::XResStarHash(xres_star_hash) => {
            database::key_shares::get_from_xres_star_hash(&mut transaction, xres_star_hash).await?
        }
        XResHashKind::XResHash(xres_hash) => {
            database::key_shares::get_from_xres_hash(&mut transaction, xres_hash).await?
        }
    };

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
