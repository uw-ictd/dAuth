use std::sync::Arc;

use crate::data::context::DauthContext;
use crate::data::error::DauthError;

/// Starts a task manager that runs periodically.
/// Does not block.
pub async fn start_task_manager(context: Arc<DauthContext>) -> Result<(), DauthError> {
    tracing::info!("Starting task manager");

    tokio::spawn(async move { run_task_manager(context).await });

    Ok(())
}

async fn run_task_manager(context: Arc<DauthContext>) -> Result<(), DauthError> {
    tracing::info!(
        "Task manager running, starting normal tasks in {:?}s",
        context.tasks_context.startup_delay
    );
    tokio::time::interval(context.tasks_context.startup_delay)
        .tick()
        .await;

    let mut interval = tokio::time::interval(context.tasks_context.interval);
    loop {
        tracing::info!("Checking for tasks to run");
        // TODO: add tasks

        interval.tick().await;
    }
}
