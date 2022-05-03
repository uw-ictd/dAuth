use std::sync::Arc;

use crate::data::context::DauthContext;
use crate::data::error::DauthError;
use crate::rpc::clients::directory;

/// Attempts to register if not already registered.
pub async fn run_task(context: Arc<DauthContext>) -> Result<(), DauthError> {
    let mut is_registered = context.tasks_context.is_registered.lock().await;

    if !*is_registered {
        directory::register(context.clone()).await?;

        *is_registered = true;
    }

    Ok(())
}
