use std::collections::HashMap;
use std::sync::Arc;
use tokio::task::JoinSet;

use crate::data::context::DauthContext;
use crate::data::error::DauthError;
use crate::database;
use crate::database::tasks::report_key_shares::ReportKeyShareTask;
use crate::rpc::clients;

/// Runs the report key shares task.
pub async fn run_task(context: Arc<DauthContext>) -> Result<(), DauthError> {
    let reports;
    {
        let mut transaction = context.local_context.database_pool.begin().await.unwrap();
        reports = database::tasks::report_key_shares::get(&mut transaction).await?;
        transaction.commit().await.unwrap();
    }

    if reports.is_empty() {
        tracing::debug!("Nothing to do for report key share task");
    } else {
        tracing::debug!("Found {} report key share(s) pending", reports.len());

        // Groupby Users
        let mut task_per_network: HashMap<String, Vec<ReportKeyShareTask>> = HashMap::new();

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
                        tracing::info!(?e, "Failed to execute report key share task");
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
    reports: Vec<ReportKeyShareTask>,
) -> Result<(), DauthError> {
    tracing::info!(?network_id, "Reporting key share(s) used to home network");
    let (home_net_address, _) = clients::directory::lookup_network(&context, &network_id).await?;

    let mut client = clients::home_network::get_client(context.clone(), &home_net_address).await?;

    // Run reports to a single network serially for now. This could be
    // parallelized in the future, or we could build an aggregate API.
    for report in reports {
        match clients::home_network::report_key_share_consumed(
            &context,
            &report.signed_request_bytes,
            &mut client,
        )
        .await {
            Ok(_) => (),
            Err(DauthError::ClientError(msg)) => {
                clients::home_network::mark_endpoint_offline(&context, &home_net_address).await;
                return Err(DauthError::ClientError(msg));
            },
            Err(e) => {
                return Err(e);
            }
        };

        let mut transaction = context.local_context.database_pool.begin().await?;
        database::tasks::report_key_shares::remove(&mut transaction, &report.xres_star_hash)
            .await?;
        transaction.commit().await?;
    }

    Ok(())
}
