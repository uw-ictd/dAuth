mod common;
mod data;
mod database;
mod management;
mod rpc;
mod services;
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

    let config = startup::build_config_from_file(DauthOpt::from_args().config_path)
        .expect("Failed to read configuration file");
    let context = startup::build_context(config)
        .await
        .expect("Failed to generate context");

    tasks::task_manager::start(context.clone())
        .await
        .expect("Failed to start task manager");

    server::start_servers(context.clone()).await;
}
