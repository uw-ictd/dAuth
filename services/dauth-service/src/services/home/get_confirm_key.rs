use std::sync::Arc;

use crate::common;
use crate::data::{combined_res::ResKind, context::DauthContext, error::DauthError, keys};

#[tracing::instrument(skip(context), name = "home::get_confirm_key")]
pub async fn get_confirm_key(
    context: Arc<DauthContext>,
    combined_res: ResKind,
) -> Result<keys::KeyKind, DauthError> {
    tracing::info!("Getting confirm key for remote authentication");

    match combined_res {
        ResKind::ResStar(res_star) => Ok(keys::KeyKind::Kseaf(
            common::confirm_keys::get_confirm_key_res_star(context.clone(), res_star).await?,
        )),
        ResKind::Res(res) => Ok(keys::KeyKind::Kasme(
            common::confirm_keys::get_confirm_key_res(context.clone(), res).await?,
        )),
    }
}
