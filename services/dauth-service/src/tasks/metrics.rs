use std::sync::Arc;

use crate::data::context::DauthContext;
use crate::data::error::DauthError;

/// Runs the metrics task.
pub async fn run_task(context: Arc<DauthContext>) -> Result<(), DauthError> {
    let metrics = context.metrics_context.get_metrics().await;

    tracing::info!("{:?}", metrics);

    Ok(())
}
