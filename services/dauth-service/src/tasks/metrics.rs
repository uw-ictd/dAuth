use std::sync::Arc;
use std::time::Instant;

use crate::data::context::DauthContext;
use crate::data::error::DauthError;

/// Runs the metrics task.
pub async fn run_task(context: Arc<DauthContext>) -> Result<(), DauthError> {
    let mut last_report = context.tasks_context.metrics_last_report.lock().await;
    let metrics = context.metrics_context.get_metrics().await;

    if last_report.elapsed() > context.tasks_context.metrics_report_interval {
        tracing::info!("{:?}", metrics);
        *last_report = Instant::now();
    }

    Ok(())
}
