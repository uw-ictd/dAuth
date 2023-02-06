use std::sync::Arc;

use crate::data::{
    context::DauthContext,
    error::DauthError,
    vector::{AuthVectorReq, AuthVectorRes},
};
use crate::database;
use crate::database::utilities::DauthDataUtilities;

/// Gets the next backup auth vector, checking for any available flood
/// vectors first. If there are no flood vectors, returns the auth vector
/// with the lowest seqnum.
#[tracing::instrument(skip(context), name = "backup::get_auth_vector")]
pub async fn get_auth_vector(
    context: Arc<DauthContext>,
    av_request: &AuthVectorReq,
    signed_request_bytes: &Vec<u8>,
) -> Result<AuthVectorRes, DauthError> {
    tracing::info!("Getting backup auth vector");

    let mut transaction = context.local_context.database_pool.begin().await?;

    // Check for a flood vector first
    let vector;
    if let Some(flood_row) =
        database::flood_vectors::get_first(&mut transaction, &av_request.user_id).await?
    {
        vector = flood_row.to_auth_vector()?;

        database::flood_vectors::mark_sent(&mut transaction, &vector.user_id, vector.seqnum)
            .await?;
        // database::flood_vectors::remove(&mut transaction, &vector.user_id, &vector.xres_star_hash).await?;

        tracing::info!("Flood vector found: {:?}", vector);
    } else {
        vector = database::auth_vectors::get_first(&mut transaction, &av_request.user_id)
            .await?
            .to_auth_vector()?;

        database::auth_vectors::mark_sent(&mut transaction, &vector.user_id, vector.seqnum).await?;
        // database::auth_vectors::remove(&mut transaction, &vector.user_id, &vector.xres_star_hash).await?;

        tracing::info!("Backup vector found: {:?}", vector);
    };

    database::tasks::report_auth_vectors::add(
        &mut transaction,
        &vector.xres_star_hash,
        &vector.user_id,
        signed_request_bytes,
    )
    .await?;

    transaction.commit().await?;

    Ok(vector)
}
