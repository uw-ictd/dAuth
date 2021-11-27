use std::sync::Arc;

use tonic::transport::Server;

use crate::data::context::DauthContext;
use crate::rpc::handler::DauthHandler;

use crate::rpc::dauth::common::local_authentication_server::LocalAuthenticationServer;
use crate::rpc::dauth::common::remote_authentication_server::RemoteAuthenticationServer;

// TODO(matt9j) Probably should return a result in case server start fails
#[tracing::instrument]
pub async fn start_server(context: Arc<DauthContext>) {
    tracing::info!("Hosting RPC server on {}", context.rpc_context.host_addr);

    Server::builder()
        .add_service(LocalAuthenticationServer::new(DauthHandler {
            context: context.clone(),
        }))
        .add_service(RemoteAuthenticationServer::new(DauthHandler {
            context: context.clone(),
        }))
        .serve(context.rpc_context.host_addr.parse().unwrap())
        .await
        .unwrap();
}
