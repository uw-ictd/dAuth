use std::sync::Arc;

use crate::data::context::DauthContext;
use crate::rpc::d_auth::{AkaVectorReq, AkaVectorResp};

/// Removes and returns vector if at least one exists.
pub fn auth_vector_next(
    context: Arc<DauthContext>,
    av_request: &AkaVectorReq,
) -> Option<AkaVectorResp> {
    tracing::info!("Database next: {:?}", av_request);

    match context
        .local_context
        .database
        .lock()
        .unwrap()
        .get_mut(&av_request.user_id)
    {
        Some(queue) => queue.pop_front(),
        None => {
            tracing::error!("User not in database (next): {:?}", av_request);
            None
        }
    }
}

/// Deletes a vector if found.
pub fn auth_vector_delete(
    context: Arc<DauthContext>,
    av_result: &AkaVectorResp,
) -> Result<(), &'static str> {
    tracing::info!("Database delete: {:?}", av_result);

    match context
        .local_context
        .database
        .lock()
        .unwrap()
        .get_mut(&av_result.user_id)
    {
        Some(queue) => {
            let num_elements = queue.len();
            queue.retain(|x| *x != *av_result);
            match num_elements - queue.len() {
                0 => tracing::info!("Nothing deleted"),
                1 => (),
                x => tracing::warn!("{} deleted", x),
            };
            Ok(())
        }
        None => {
            tracing::error!("Use not in database (delete): {:?}", av_result);
            Err("Failed to find user")
        }
    }
}
