use std::sync::Arc;

mod data;
mod database;
mod manager;
mod rpc;

#[tokio::main]
async fn main() {
    let context = Arc::new(data::context::DirectoryContext {
        host_address: "127.0.0.1:8900".to_string(),
        database_pool: database::general::database_init("./directory_sqlite.db").await.expect("Failed to initialize database"),
    });

    rpc::server::start_server(context).await;
}
