mod data;
mod database;
mod manager;
mod rpc;
mod startup;

use structopt::StructOpt;

use data::opt::DirectoryOpt;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let config = startup::build_config(DirectoryOpt::from_args().config_path)
        .expect("Failed to parse config");

    let context = startup::build_context(config)
        .await
        .expect("Failed to generate context");

    rpc::server::start_server(context).await;
}
