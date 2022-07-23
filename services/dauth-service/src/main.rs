mod core;
mod data;
mod database;
mod rpc;
mod startup;
mod tasks;

use structopt::StructOpt;
use tracing_subscriber::{filter, EnvFilter};

use crate::data::opt::DauthOpt;
use crate::rpc::server;

#[tokio::main]
async fn main() {
    let log_filter = EnvFilter::builder()
        .with_default_directive(filter::LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::fmt().with_env_filter(log_filter).init();

    let dauth_opt = DauthOpt::from_args();
    let context = startup::build_context(dauth_opt)
        .await
        .expect("Failed to generate context");

    tasks::task_manager::start(context.clone())
        .await
        .expect("Failed to start task manager");

    server::start_servers(context.clone()).await;
}
