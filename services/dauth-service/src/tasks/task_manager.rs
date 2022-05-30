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
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
    loop {
        tracing::trace!("Checking for tasks to run");

        // Register with directory before any register-dependent tasks
        if let Err(e) = tasks::register::run_task(context.clone()).await {
            tracing::warn!("Failed to run register task: {}", e);
        } else {
            let mut tasks = Vec::new();

            tasks.push(tokio::spawn(tasks::update_users::run_task(context.clone())));
            tasks.push(tokio::spawn(tasks::replace_key_shares::run_task(
                context.clone(),
            )));
            tasks.push(tokio::spawn(tasks::report_auth_vectors::run_task(
                context.clone(),
            )));
            tasks.push(tokio::spawn(tasks::report_key_shares::run_task(
                context.clone(),
            )));

            for task in tasks {
                match task.await {
                    Ok(task_res) => {
                        if let Err(e) = task_res {
                            tracing::warn!("Error while executing task: {}", e)
                        }
                    }
                    Err(je) => {
                        tracing::warn!("Error while joining: {}", je)
                    }
                }
            }
        }

        interval.tick().await; // Using delay, will always wait max time
    }
}
