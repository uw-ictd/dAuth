use std::sync::Arc;

use tonic::transport::Server;

use crate::rpc::directory_service::directory_server::DirectoryServer;
use crate::data::context::DirectoryContext;
use crate::rpc::handler::DirectoryHandler;

#[tracing::instrument]
pub async fn start_server(context: Arc<DirectoryContext>) {
    tracing::info!("Hosting directory server on {}", context.host_address);

    Server::builder()
        .add_service(DirectoryServer::new(DirectoryHandler {
            context: context.clone(),
        }))
        .serve(context.host_address.parse().unwrap())
        .await
        .unwrap();
}
