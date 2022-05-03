use std::sync::Arc;

use crate::data::context::DauthContext;
use crate::data::error::DauthError;
use crate::tasks;

/// Starts a task manager that runs periodically.
/// Does not block.
pub async fn start(context: Arc<DauthContext>) -> Result<(), DauthError> {
    tracing::info!("Starting task manager");

    tokio::spawn(async move { run(context).await });

    Ok(())
}

async fn run(context: Arc<DauthContext>) -> Result<(), DauthError> {
    tracing::info!(
        "Task manager running, starting normal tasks in {:?}s",
        context.tasks_context.startup_delay
    );

    let mut startup_delay = tokio::time::interval(context.tasks_context.startup_delay);
    startup_delay.tick().await; // first tick does nothing
    startup_delay.tick().await;

    let mut interval = tokio::time::interval(context.tasks_context.interval);
    loop {
        interval.tick().await;
        tracing::info!("Checking for tasks to run");

        // Register with directory before any register-dependent tasks
        if let Err(e) = tasks::register::run_task(context.clone()).await {
            tracing::warn!("Failed to run register task: {}", e);
        } else {
            if let Err(e) = tasks::update_users::run_task(context.clone()).await {
                tracing::warn!("Failed to run update user task: {}", e)
            }
        }
    }
}
