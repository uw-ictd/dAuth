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
                Vec::with_capacity(context.metrics_context.max_recorded_metrics);
            let mut idle_times = Vec::with_capacity(context.metrics_context.max_recorded_metrics);
            let mut polling_times =
                Vec::with_capacity(context.metrics_context.max_recorded_metrics);

            for metric in metrics {
                idle_times.push(metric.total_idle_duration);
                polling_times.push(metric.total_poll_duration);
            }

            let mut it_strings: Vec<String> = Vec::new();

            for time in idle_times.iter() {
                it_strings.push(format!("{:?}", time))
            }

            let mut pt_strings: Vec<String> = Vec::new();

            for time in polling_times.iter() {
                pt_strings.push(format!("{:?}", time))
            }

            let idle_times_string = format!("\"{}\": {:?}", "idle times", it_strings);
            let polling_times_string = format!("\"{}\": {:?}", "polling times", pt_strings);
            let (idle_len, polling_len) = (idle_times.len() as u32, polling_times.len() as u32);
            let (idle_avg, polling_avg) = (
                idle_times.into_iter().sum::<Duration>() / idle_len,
                polling_times.into_iter().sum::<Duration>() / polling_len,
            );

            metric_results.push(format!(
                "\"{}\": \"{:?}\"",
                "total average",
                idle_avg + polling_avg
            ));
            metric_results.push(format!("\"{}\": \"{:?}\"", "idle average", idle_avg));
            metric_results.push(format!("\"{}\": \"{:?}\"", "polling average", polling_avg));
            metric_results.push(idle_times_string);
            metric_results.push(polling_times_string);

            tracing::info!(
                "Metrics for {}: {{{:?}}}",
                metric_id,
                metric_results.join(", ")
            );
        }

        *last_report = Instant::now();
    }

    Ok(())
}
