use std::sync::Arc;

use auth_vector::{self, data::AuthVectorData};
use sqlx::{Sqlite, Transaction};

use crate::data::{
    context::DauthContext,
    error::DauthError,
};
use crate::database;
use crate::database::utilities::DauthDataUtilities;

/// Builds an auth vector and updates the user state.
/// Returns the auth vector and seqnum values.
pub async fn build_auth_vector(
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
