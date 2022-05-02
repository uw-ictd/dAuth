mod data;
mod database;
mod manager;
mod rpc;
mod startup;
mod tasks;

use structopt::StructOpt;
use tracing::Level;
use tracing_subscriber;

use crate::data::opt::DauthOpt;
use crate::rpc::server;

#[tokio::main]
async fn main() {
    // TODO(nickfh7) Add configuring for logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let dauth_opt = DauthOpt::from_args();
    let context = startup::build_context(dauth_opt)
        .await
        .expect("Failed to generate context");

    tasks::start_task_manager(context.clone())
        .await
        .expect("Failed to start task manager");

    server::start_server(context.clone()).await;
}
