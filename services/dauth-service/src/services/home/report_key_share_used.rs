use std::sync::Arc;

use crate::data::{
    combined_res::ResKind, combined_res::XResHashKind, context::DauthContext, error::DauthError,
};
use crate::database;

/// Handles a key share that was generated by this network and used
/// by a backup network.
pub async fn report_key_share_used(
    context: Arc<DauthContext>,
    response: &ResKind,
    xresponse_hash: &XResHashKind,
    backup_network_id: &str,
) -> Result<(), DauthError> {
    tracing::info!("Key share reported used by {}", backup_network_id);

    let mut transaction = context.local_context.database_pool.begin().await?;

    let state = match xresponse_hash {
        XResHashKind::XResStarHash(xres_star_hash) => {
            database::key_share_state::get_by_xres_star_hash(
                &mut transaction,
                xres_star_hash,
                backup_network_id,
            )
            .await?
        }
        XResHashKind::XResHash(xres_hash) => {
            database::key_share_state::get_by_xres_hash(
                &mut transaction,
                xres_hash,
                backup_network_id,
            )
            .await?
        }
    };

    if state.is_none() {
        tracing::warn!("Key share use reported with no corresponding share state");
        return Ok(());
    }

    let state = state.unwrap();
    tracing::info!(
        "Key share reported used by {} mapped to user {}",
        backup_network_id,
        state.user_id
    );

    match xresponse_hash {
        XResHashKind::XResStarHash(xres_star_hash) => {
            if let ResKind::ResStar(res_star) = response {
                validate_xres_star_hash(xres_star_hash, res_star, &state.rand)?;
                database::key_share_state::remove_by_xres_star_hash(
                    &mut transaction,
                    xres_star_hash,
                    backup_network_id,
                )
                .await?;
            } else {
                return Err(DauthError::DataError(
                    "Provided xres* with no res*".to_string(),
                ));
            }
        }
        XResHashKind::XResHash(xres_hash) => {
            if let ResKind::Res(res) = response {
                validate_xres_hash(xres_hash, res, &state.rand)?;
                database::key_share_state::remove_by_xres_hash(
                    &mut transaction,
                    xres_hash,
                    backup_network_id,
                )
                .await?;
            } else {
                return Err(DauthError::DataError(
                    "Provided xres with no res".to_string(),
                ));
            }
        }
    };

    transaction.commit().await?;

    Ok(())
}

/// Confirms res* is a valid preimage of xres* hash.
fn validate_xres_star_hash(
    xres_star_hash: &auth_vector::types::XResStarHash,
    res_star: &auth_vector::types::ResStar,
    rand: &auth_vector::types::Rand,
) -> Result<(), DauthError> {
    if xres_star_hash != &auth_vector::types::gen_xres_star_hash(rand, res_star) {
        Err(DauthError::DataError(
            "Provided res* does not hash to provided xres* hash".to_string(),
        ))
    } else {
        Ok(())
    }
}

/// Confirms res* is a valid preimage of xres* hash.
fn validate_xres_hash(
    xres_hash: &auth_vector::types::XResHash,
    res: &auth_vector::types::Res,
    rand: &auth_vector::types::Rand,
) -> Result<(), DauthError> {
    if xres_hash != &auth_vector::types::gen_xres_hash(rand, res) {
        Err(DauthError::DataError(
            "Provided res* does not hash to provided xres* hash".to_string(),
        ))
    } else {
        Ok(())
    }
}
