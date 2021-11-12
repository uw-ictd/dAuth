use std::net::SocketAddr;
use std::sync::Arc;

use tonic::transport::Server;

use crate::data::context::DauthContext;
use crate::rpc::handler::DauthHandler;

use crate::rpc::d_auth::local_authentication_server::LocalAuthenticationServer;
use crate::rpc::d_auth::remote_authentication_server::RemoteAuthenticationServer;

// TODO(matt9j) Probably should return a result in case server start fails
#[tracing::instrument]
pub async fn start_server(context: Arc<DauthContext>) {
    // Testing until rpc/proto is ready
    tracing::info!("Hosting RPC server on {}", context.rpc_context.host_addr);

    // TODO(nickfh7) add other services
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
