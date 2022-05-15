use std::sync::Arc;

use crate::data::context::DauthContext;
use crate::data::error::DauthError;
use crate::database;
use crate::database::tasks::report_key_shares::ReportKeyShareTask;
use crate::rpc::clients;

/// Runs the report key shares task.
pub async fn run_task(context: Arc<DauthContext>) -> Result<(), DauthError> {
    let mut transaction = context.local_context.database_pool.begin().await.unwrap();
    let reports = database::tasks::report_key_shares::get(&mut transaction).await?;
    transaction.commit().await.unwrap();

    if reports.is_empty() {
        tracing::info!("Nothing to do for report key share task");
    } else {
        tracing::info!("Found {} report key share(s) pending", reports.len());

        let mut tasks = Vec::new();

        for report in reports {
            tasks.push(tokio::spawn(report_key_shares(context.clone(), report)));
        }

        for task in tasks {
            match task.await {
                Ok(task_res) => match task_res {
                    Ok(report) => {
                        let mut transaction = context.local_context.database_pool.begin().await?;
                        database::tasks::report_key_shares::remove(
                            &mut transaction,
                            &report.xres_star_hash,
                        )
                        .await?;
                        transaction.commit().await?;
                    }
                    Err(e) => {
                        tracing::warn!("Failed to execute report key share task: {}", e)
                    }
                },
                Err(je) => {
                    tracing::warn!("Error while joining: {}", je)
                }
            }
        }
    }
    Ok(())
}

async fn report_key_shares(
    context: Arc<DauthContext>,
    report: ReportKeyShareTask,
) -> Result<ReportKeyShareTask, DauthError> {
    let (home_network_id, _) =
        clients::directory::lookup_user(context.clone(), &report.user_id).await?;

    let (address, _) =
        clients::directory::lookup_network(context.clone(), &home_network_id).await?;

    clients::home_network::report_key_share_consumed(
        context,
        &report.signed_request_bytes,
        &address,
    )
    .await?;

    Ok(report)
}
