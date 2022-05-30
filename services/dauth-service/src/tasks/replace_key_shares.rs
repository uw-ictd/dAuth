use std::sync::Arc;

use crate::data::context::DauthContext;
use crate::data::error::DauthError;
use crate::database;
use crate::database::tasks::replace_key_shares::ReplaceKeyShareTask;
use crate::rpc::clients;

/// Runs the replace key shares task.
/// If any replacements are queued up, waits for 10 seconds and then calls
/// the key share replacement rpc.
pub async fn run_task(context: Arc<DauthContext>) -> Result<(), DauthError> {
    let mut transaction = context.local_context.database_pool.begin().await.unwrap();
    let replaces = database::tasks::replace_key_shares::get(&mut transaction).await?;
    transaction.commit().await.unwrap();

    if replaces.is_empty() {
        tracing::info!("Nothing to do for replace key share task");
    } else {
        tracing::info!("Found {} replace key share(s) pending", replaces.len());

        let mut tasks = Vec::new();

        for replace in replaces {
            tasks.push(tokio::spawn(replace_key_share(context.clone(), replace)));
        }

        for task in tasks {
            match task.await {
                Ok(task_res) => match task_res {
                    Ok(replace) => {
                        let mut transaction = context.local_context.database_pool.begin().await?;
                        database::tasks::replace_key_shares::remove(
                            &mut transaction,
                            &replace.backup_network_id,
                            &replace.xres_star_hash,
                        )
                        .await?;
                        transaction.commit().await?;
                    }
                    Err(e) => {
                        tracing::warn!("Failed to execute replace task: {}", e)
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

async fn replace_key_share(
    context: Arc<DauthContext>,
    replace: ReplaceKeyShareTask,
) -> Result<ReplaceKeyShareTask, DauthError> {
    let mut replace_delay = tokio::time::interval(context.tasks_context.replace_key_share_delay);
    replace_delay.tick().await; // first tick does nothing
    replace_delay.tick().await;

    let (address, _) =
        clients::directory::lookup_network(context.clone(), &replace.backup_network_id).await?;

    clients::backup_network::replace_key_share(context, &replace, &address).await?;

    Ok(replace)
}
