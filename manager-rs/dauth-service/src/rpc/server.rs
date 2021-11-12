use std::sync::Arc;

use tonic::transport::Server;

use crate::data::context::DauthContext;
use crate::rpc::handler::DauthHandler;

use crate::rpc::d_auth::local_authentication_server::LocalAuthenticationServer;

// TODO(matt9j) Probably should return a result in case server start fails
#[tracing::instrument]
pub async fn start_server(context: Arc<DauthContext>) {
    // Testing until rpc/proto is ready
    let handler = DauthHandler {
        context: context.clone(),
    };

    tracing::info!("Hosting RPC server on {}", context.rpc_context.host_addr);

    // TODO(nickfh7) add other services
    Server::builder()
        .add_service(LocalAuthenticationServer::new(handler))
        .serve(context.rpc_context.host_addr.parse().unwrap())
        .await
        .unwrap();
}
