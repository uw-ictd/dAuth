use std::collections::HashMap;
use std::sync::Arc;

use tokio::task::JoinSet;

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
        tracing::debug!("Found {} report auth used vector(s) pending", reports.len());

        // Groupby Users
        let mut task_per_network: HashMap<String, Vec<ReportAuthVectorTask>> = HashMap::new();

        for report in reports {
            let (home_network_id, _) =
                clients::directory::lookup_user(&context, &report.user_id).await?;

            match task_per_network.get_mut(&report.user_id) {
                None => {
                    // The first task for this network requires allocating a new vector.
                    task_per_network.insert(home_network_id, vec![report]);
                }
                Some(task_vector) => {
                    task_vector.push(report);
                }
            }
        }

        let mut tasks = JoinSet::new();

        for (network, reports) in task_per_network.into_iter() {
            tasks.spawn(report_to_network(context.clone(), network, reports));
        }

        while let Some(join_result) = tasks.join_one().await {
            match join_result {
                Ok(task_res) => match task_res {
                    Ok(_) => {
                        // No action for successful report.
                    }
                    Err(e) => {
                        tracing::warn!(?e, "Failed to execute report key share task");
                    }
                },
                Err(e) => {
                    tracing::error!(?e, "Error while joining");
                }
            }
        }
    }
    Ok(())
}

async fn report_to_network(
    context: Arc<DauthContext>,
    network_id: String,
    reports: Vec<ReportAuthVectorTask>,
) -> Result<(), DauthError> {
    tracing::info!(?network_id, "Reporting auth vector(s) used to home network");

    let (home_net_address, _) = clients::directory::lookup_network(&context, &network_id).await?;

    let mut client = clients::home_network::get_client(context.clone(), &home_net_address).await?;

    // Run reports to a single network serially for now. This could be
    // parallelized in the future, or we could build an aggregate API.
    for report in reports {
        let possible_av_result = clients::home_network::report_auth_consumed(
            &context,
            &report.xres_star_hash[..].try_into()?,
            &report.user_id,
            &report.signed_request_bytes,
            &mut client,
        )
        .await?;

        if let Some(av_result) = possible_av_result {
            core::auth_vectors::store_backup_auth_vector(context.clone(), &av_result).await?;
        }

        {
            let mut transaction = context.local_context.database_pool.begin().await?;
            database::tasks::report_auth_vectors::remove(&mut transaction, report.task_id).await?;
            transaction.commit().await?;
        }
    }

    Ok(())
}
