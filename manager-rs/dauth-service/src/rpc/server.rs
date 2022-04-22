use std::sync::Arc;

use tonic::transport::Server;

use crate::data::context::DauthContext;
use crate::rpc::handlers::handler::DauthHandler;

use crate::rpc::dauth::local::local_authentication_server::LocalAuthenticationServer;
use crate::rpc::dauth::remote::home_network_server::HomeNetworkServer;

// TODO(matt9j) Probably should return a result in case server start fails
#[tracing::instrument]
pub async fn start_server(context: Arc<DauthContext>) {
    tracing::info!("Hosting RPC server on {}", context.rpc_context.host_addr);

    Server::builder()
        .add_service(LocalAuthenticationServer::new(DauthHandler {
            context: context.clone(),
        }))
        .add_service(HomeNetworkServer::new(DauthHandler {
            context: context.clone(),
        }))
        .serve(context.rpc_context.host_addr.parse().unwrap())
        .await
        .unwrap();
}
