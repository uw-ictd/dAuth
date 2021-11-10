use std::sync::Arc;

use crate::data::context::DauthContext;
use crate::rpc::d_auth::{AkaVectorReq, AkaVectorResp};

/// Removes and returns vector if found.
pub fn auth_vector_next(
    context: Arc<DauthContext>,
    av_request: &AkaVectorReq,
) -> Option<AkaVectorResp> {
    tracing::info!("Database next: {:?}", av_request);

    match context.local_context.database.lock() {
        Ok(mut database) => match database.get_mut(&av_request.user_id) {
            Some(queue) => queue.pop_front(),
            None => None,
        },
        Err(e) => {
            tracing::error!("Failed getting mutex for database: {}", e);
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

    match context.local_context.database.lock() {
        Ok(mut database) => {
            // TODO(nickfh7) Look up vector
            // May need to add user id field to resp
            Ok(())
        }
        Err(e) => {
            tracing::error!("Failed getting mutex for database: {}", e);
            Err("Failed to get mutex")
        }
    }
}
