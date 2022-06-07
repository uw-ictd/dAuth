use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::data::context::DauthContext;
use crate::data::error::DauthError;

/// Runs the metrics task.
pub async fn run_task(context: Arc<DauthContext>) -> Result<(), DauthError> {
    let mut last_report = context.tasks_context.metrics_last_report.lock().await;

    if last_report.elapsed() > context.tasks_context.metrics_report_interval
        && context.metrics_context.max_recorded_metrics > 0
    {
        let all_metrics = context.metrics_context.get_metrics().await;

        for (metric_id, metrics) in all_metrics {
            let mut metric_results =
                HashMap::with_capacity(context.metrics_context.max_recorded_metrics);
            let mut idle_times = Vec::with_capacity(context.metrics_context.max_recorded_metrics);
            let mut polling_times =
                Vec::with_capacity(context.metrics_context.max_recorded_metrics);

            for metric in metrics {
                idle_times.push(metric.total_idle_duration);
                polling_times.push(metric.total_poll_duration);
            }

            metric_results.insert("idle times", format!("{:?}", idle_times));
            metric_results.insert("polling times", format!("{:?}", polling_times));

            let (idle_len, polling_len) = (idle_times.len() as u32, polling_times.len() as u32);
            metric_results.insert(
                "idle average",
                format!("{:?}", idle_times.into_iter().sum::<Duration>() / idle_len),
            );
            metric_results.insert(
                "polling average",
                format!(
                    "{:?}",
                    polling_times.into_iter().sum::<Duration>() / polling_len
                ),
            );

            tracing::info!("Metrics for {}: {:?}", metric_id, metric_results);
        }

        *last_report = Instant::now();
    }

    Ok(())
}
