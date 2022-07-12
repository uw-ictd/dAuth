use std::sync::Arc;

use crate::core;
use crate::data::context::DauthContext;
use crate::data::error::DauthError;
use crate::database;
use crate::database::tasks::report_auth_vectors::ReportAuthVectorTask;
use crate::rpc::clients;

/// Runs the report auth vector task.
pub async fn run_task(context: Arc<DauthContext>) -> Result<(), DauthError> {
    let mut transaction = context.local_context.database_pool.begin().await.unwrap();
    let reports = database::tasks::report_auth_vectors::get(&mut transaction).await?;
    transaction.commit().await.unwrap();

    if reports.is_empty() {
        tracing::debug!("Nothing to do for report auth vector task");
    } else {
        tracing::info!("Found {} report auth used vector(s) pending", reports.len());

        let mut tasks = Vec::new();

        for report in reports {
            tasks.push(tokio::spawn(report_auth_vector(context.clone(), report)));
        }

        for task in tasks {
            match task.await {
                Ok(task_res) => match task_res {
                    Ok(report) => {
                        let mut transaction = context.local_context.database_pool.begin().await?;
                        database::tasks::report_auth_vectors::remove(
                            &mut transaction,
                            &report.xres_star_hash,
                        )
                        .await?;
                        transaction.commit().await?;
                    }
                    Err(e) => {
                        tracing::warn!("Failed to execute report auth vector task: {}", e)
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

async fn report_auth_vector(
    context: Arc<DauthContext>,
    report: ReportAuthVectorTask,
) -> Result<ReportAuthVectorTask, DauthError> {
    tracing::debug!("Looking up home network ID for user {}", &report.user_id);
    let (home_network_id, _) =
        clients::directory::lookup_user(context.clone(), &report.user_id).await?;

    tracing::debug!("Looking up address for home network id {}", &home_network_id);
    let (address, _) =
        clients::directory::lookup_network(context.clone(), &home_network_id).await?;

    tracing::debug!("Reporting auth consumed to home network");
    let av_result = clients::home_network::report_auth_consumed(
        context.clone(),
        &report.xres_star_hash[..].try_into()?,
        &report.user_id,
        &report.signed_request_bytes,
        &address,
    )
    .await?;

    core::auth_vectors::store_backup_auth_vector(context.clone(), &av_result).await?;

    Ok(report)
}
