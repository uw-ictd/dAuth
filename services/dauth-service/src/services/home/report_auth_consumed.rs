use std::sync::Arc;

use auth_vector::{self, data::AuthVectorData};
use sqlx::{Sqlite, Transaction};

use crate::data::{context::DauthContext, error::DauthError, keys, vector::AuthVectorRes};
use crate::database;
use crate::database::utilities::DauthDataUtilities;
use crate::rpc::clients;

// Report that a vector given to a backup network has been used.
/// Returns a new vector to replace the used vector.
/// Sends new key shares to all other backup networks for the same user.
pub async fn backup_auth_vector_used(
    context: Arc<DauthContext>,
    backup_network_id: &str,
    xres_star_hash: &auth_vector::types::XResStarHash,
) -> Result<Option<AuthVectorRes>, DauthError> {
    tracing::info!(
        "Auth vector reported used on {:?}: {:?}",
        backup_network_id,
        xres_star_hash
    );

    let mut transaction = context.local_context.database_pool.begin().await?;

    let held_state = database::vector_state::get(&mut transaction, xres_star_hash).await?;
    if held_state.is_none() {
        tracing::info!("No local state for vector, likely was already reported and replaced");
        return Ok(None);
    }

    let (owning_network_id, user_id) = held_state.unwrap();

    if owning_network_id != backup_network_id {
        return Err(DauthError::DataError("Not the owning network".to_string()));
    }

    database::vector_state::remove(&mut transaction, xres_star_hash).await?;

    let seqnum_slice =
        database::backup_networks::get_slice(&mut transaction, &user_id, backup_network_id).await?;

    let (auth_vector_data, seqnum) =
        build_auth_vector(context.clone(), &mut transaction, &user_id, seqnum_slice).await?;

    database::vector_state::add(
        &mut transaction,
        &auth_vector_data.xres_star_hash,
        &user_id,
        backup_network_id,
    )
    .await?;

    let (_, backup_networks) = clients::directory::lookup_user(&context, &user_id).await?;

    let mut kseaf_key_shares = keys::create_shares_from_kseaf(
        &auth_vector_data.kseaf,
        backup_networks.len() as u8,
        std::cmp::min(
            context.backup_context.backup_key_threshold,
            backup_networks.len() as u8,
        ),
        &mut rand_0_8::thread_rng(),
    )?;

    let mut kasme_key_shares = keys::create_shares_from_kasme(
        &auth_vector_data.kasme,
        backup_networks.len() as u8,
        std::cmp::min(
            context.backup_context.backup_key_threshold,
            backup_networks.len() as u8,
        ),
        &mut rand_0_8::thread_rng(),
    )?;

    for id in backup_networks {
        let kseaf_share = kseaf_key_shares.pop().ok_or(DauthError::DataError(
            "Failed to generate all key shares".to_string(),
        ))?;
        let kasme_share = kasme_key_shares.pop().ok_or(DauthError::DataError(
            "Failed to generate all key shares".to_string(),
        ))?;
        database::tasks::replace_key_shares::add(
            &mut transaction,
            &id,
            &auth_vector_data.xres_star_hash,
            &auth_vector_data.xres_hash,
            xres_star_hash,
            &kseaf_share,
            &kasme_share,
        )
        .await?;
    }

    transaction.commit().await?;

    Ok(Some(AuthVectorRes {
        user_id: user_id.to_string(),
        seqnum,
        rand: auth_vector_data.rand,
        autn: auth_vector_data.autn,
        xres_star_hash: auth_vector_data.xres_star_hash,
        xres_hash: auth_vector_data.xres_hash,
    }))
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
