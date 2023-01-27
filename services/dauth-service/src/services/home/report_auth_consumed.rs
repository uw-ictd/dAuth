use std::sync::Arc;

use auth_vector::data::AuthVectorData;
use sqlx::{Sqlite, Transaction};

use crate::common;
use crate::data::{context::DauthContext, error::DauthError, keys, vector::AuthVectorRes};
use crate::database;
use crate::rpc::clients;

/// Handles a report that a vector was consumed by a backup network.
/// Returns a new vector to replace the used vector.
/// Sends new key shares to all other backup networks for the same user.
#[tracing::instrument(skip(context), name = "home::report_auth_consumed")]
pub async fn report_auth_consumed(
    context: Arc<DauthContext>,
    backup_network_id: &str,
    old_xres_star_hash: &auth_vector::types::XResStarHash,
) -> Result<Option<AuthVectorRes>, DauthError> {
    tracing::info!("Auth vector reported used by backup network");

    // A lot of operations are done on this transaction, but this is
    // the best way to ensure that changes are reverted on error.
    let mut transaction = context.local_context.database_pool.begin().await?;

    match replace_auth_vector_from_state(
        context.clone(),
        &mut transaction,
        backup_network_id,
        old_xres_star_hash,
    )
    .await?
    {
        Some((user_id, auth_vector_data, seqnum)) => {
            tracing::debug!(
                ?user_id,
                ?auth_vector_data,
                ?seqnum,
                "Replacement auth vector generated"
            );

            let (_, backup_networks) = clients::directory::lookup_user(&context, &user_id).await?;
            send_key_shares(
                context.clone(),
                &mut transaction,
                &backup_networks,
                &auth_vector_data,
                old_xres_star_hash,
            )
            .await?;

            tracing::debug!(
                ?user_id,
                ?backup_networks,
                "Replacement key shares prepared"
            );

            transaction.commit().await?;

            Ok(Some(AuthVectorRes {
                user_id,
                seqnum,
                rand: auth_vector_data.rand,
                autn: auth_vector_data.autn,
                xres_star_hash: auth_vector_data.xres_star_hash,
                xres_hash: auth_vector_data.xres_hash,
            }))
        }
        None => {
            tracing::warn!("No local state for vector, likely was already reported and replaced");
            Ok(None)
        }
    }
}

/// Attempts to build a new auth vector based on previous stored state.
/// If the state doesn't exist, this is considered a non-error and likely
/// occurs with repeated requests to report the same auth vector as used.
/// If the state does exists, a replacement auth vector is generated.
async fn replace_auth_vector_from_state(
    context: Arc<DauthContext>,
    transaction: &mut Transaction<'_, Sqlite>,
    backup_network_id: &str,
    old_xres_star_hash: &auth_vector::types::XResStarHash,
) -> Result<Option<(String, AuthVectorData, i64)>, DauthError> {
    match database::vector_state::get(transaction, old_xres_star_hash).await? {
        Some((owning_network_id, user_id)) => {
            tracing::debug!(
                ?owning_network_id,
                ?user_id,
                "Found existing state for vector"
            );

            if owning_network_id != backup_network_id {
                return Err(DauthError::DataError("Not the owning network".to_string()));
            }

            database::vector_state::remove(transaction, old_xres_star_hash).await?;

            let seqnum_slice =
                database::backup_networks::get_slice(transaction, &user_id, backup_network_id)
                    .await?;

            let (auth_vector_data, seqnum) = common::auth_vectors::build_auth_vector(
                context.clone(),
                transaction,
                &user_id,
                seqnum_slice,
            )
            .await?;

            database::vector_state::add(
                transaction,
                &auth_vector_data.xres_star_hash,
                &user_id,
                backup_network_id,
            )
            .await?;

            Ok(Some((user_id, auth_vector_data, seqnum)))
        }
        None => Ok(None),
    }
}

/// Builds a set of key shares for the provided backup networks.
/// Creates tasks to send shares (makes no network calls).
async fn send_key_shares(
    context: Arc<DauthContext>,
    transaction: &mut Transaction<'_, Sqlite>,
    backup_networks: &Vec<String>,
    auth_vector_data: &AuthVectorData,
    old_xres_star_hash: &auth_vector::types::XResStarHash,
) -> Result<(), DauthError> {
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

    for backup_network_id in backup_networks {
        let kseaf_share = kseaf_key_shares.pop().ok_or(DauthError::DataError(
            "Failed to generate all key shares".to_string(),
        ))?;
        let kasme_share = kasme_key_shares.pop().ok_or(DauthError::DataError(
            "Failed to generate all key shares".to_string(),
        ))?;
        database::tasks::replace_key_shares::add(
            transaction,
            &backup_network_id,
            &auth_vector_data.xres_star_hash,
            &auth_vector_data.xres_hash,
            old_xres_star_hash,
            &kseaf_share,
            &kasme_share,
        )
        .await?;
    }

    Ok(())
}
