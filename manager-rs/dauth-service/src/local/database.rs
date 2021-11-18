use std::sync::Arc;

use crate::data::{context::DauthContext, error::DauthError};
use crate::rpc::d_auth::{AkaVectorReq, AkaVectorResp};

/// Removes and returns vector if at least one exists.
pub fn auth_vector_next(
    context: Arc<DauthContext>,
    av_request: &AkaVectorReq,
) -> Result<AkaVectorResp, DauthError> {
    tracing::info!("Database next: {:?}", av_request);

    match context
        .local_context
        .database
        .lock()
        .unwrap()
        .get_mut(&av_request.user_id)
    {
        Some(queue) => match queue.pop_front() {
            Some(av_result) => {
                tracing::info!("Auth vector found: {:?}", av_result);
                Ok(av_result)
            }
            None => {
                tracing::info!("No vector found: {:?}", av_request);
                Err(DauthError::NotFound(format!("No vector found")))
            }
        },
        None => {
            tracing::error!("User not in database: {:?}", av_request);
            Err(DauthError::NotFound(format!("User not in database")))
        }
    }
}

/// Deletes a vector if found.
pub fn auth_vector_delete(
    context: Arc<DauthContext>,
    av_result: &AkaVectorResp,
) -> Result<(), DauthError> {
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
            tracing::error!("User not in database: {:?}", av_result);
            Err(DauthError::NotFound(format!("User not in database")))
        }
    }
}
