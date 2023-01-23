use std::sync::Arc;

use auth_vector::{self, data::AuthVectorData};
use sqlx::{Sqlite, Transaction};

use crate::data::{context::DauthContext, error::DauthError, vector::AuthVectorRes};
use crate::database;
use crate::database::utilities::DauthDataUtilities;

/// Generates an auth vector that will be verified locally.
/// Stores the kseaf directly, without key shares.
pub async fn generate_local_vector(
    context: Arc<DauthContext>,
    user_id: &str,
    sqn_slice: i64,
) -> Result<AuthVectorRes, DauthError> {
    tracing::info!("Attempting to generate new vector locally");

    let mut transaction = context.local_context.database_pool.begin().await?;

    let (auth_vector_data, seqnum) =
        build_auth_vector(context.clone(), &mut transaction, &user_id, 0).await?;

    let av_response = AuthVectorRes {
        user_id: user_id.to_string(),
        seqnum,
        rand: auth_vector_data.rand,
        autn: auth_vector_data.autn,
        xres_star_hash: auth_vector_data.xres_star_hash,
        xres_hash: auth_vector_data.xres_hash,
    };

    database::kseafs::add(
        &mut transaction,
        &auth_vector_data.xres_star,
        &auth_vector_data.kseaf,
    )
    .await?;

    database::kasmes::add(
        &mut transaction,
        &auth_vector_data.xres,
        &auth_vector_data.kasme,
    )
    .await?;

    tracing::info!("Auth vector generated: {:?}", av_response);
    transaction.commit().await?;

    Ok(av_response)
}

/// Builds an auth vector and updates the user state.
/// Returns the auth vector and seqnum values.
async fn build_auth_vector(
    context: Arc<DauthContext>,
    transaction: &mut Transaction<'_, Sqlite>,
    user_id: &str,
    sqn_slice: i64,
) -> Result<(AuthVectorData, i64), DauthError> {
    let mut user_info = database::user_infos::get(transaction, &user_id.to_string(), sqn_slice)
        .await?
        .to_user_info()?;

    tracing::info!(?user_id, ?sqn_slice, "sqn"=?user_info.sqn, "Generating Vector for user");

    let auth_vector_data = auth_vector::generate_vector(
        &context.local_context.mcc,
        &context.local_context.mnc,
        &user_info.k,
        &user_info.opc,
        &user_info.sqn.try_into()?,
    );

    user_info.sqn += context.local_context.num_sqn_slices;

    database::user_infos::upsert(
        transaction,
        &user_id.to_string(),
        &user_info.k,
        &user_info.opc,
        user_info.sqn,
        sqn_slice,
    )
    .await?;

    Ok((auth_vector_data, user_info.sqn))
}
