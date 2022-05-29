use std::sync::Arc;

use sqlx::Row;

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
        tracing::info!("Found {} report key share(s) pending", reports.len());

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
    let (home_network_id, _) =
        clients::directory::lookup_user(context.clone(), &report.user_id).await?;

    let (address, _) =
        clients::directory::lookup_network(context.clone(), &home_network_id).await?;

    let mut transaction = context.local_context.database_pool.begin().await?;
    let old_seqnum = database::auth_vectors::get_by_hash(&mut transaction, &report.xres_star_hash)
        .await?
        .try_get::<i64, &str>("seqnum")?;
    database::auth_vectors::remove(&mut transaction, &report.user_id, old_seqnum).await?;

    let av_result = clients::home_network::report_auth_consumed(
        context.clone(),
        &report.xres_star_hash[..].try_into()?,
        &report.user_id,
        &report.signed_request_bytes,
        &address,
    )
    .await?;

    // commit only when we know the home network is alerted.
    transaction.commit().await?;

    core::auth_vectors::store_backup_auth_vector(context.clone(), &av_result).await?;

    Ok(report)
}
