use std::sync::Arc;

use tonic::transport::Server;

use crate::data::context::DauthContext;
use crate::rpc::handlers::backup_network::BackupNetworkHandler;
use crate::rpc::handlers::home_network::HomeNetworkHandler;
use crate::rpc::handlers::local_authentication::LocalAuthenticationHandler;

use crate::rpc::dauth::local::local_authentication_server::LocalAuthenticationServer;
use crate::rpc::dauth::remote::backup_network_server::BackupNetworkServer;
use crate::rpc::dauth::remote::home_network_server::HomeNetworkServer;

// TODO(matt9j) Probably should return a result in case server start fails
#[tracing::instrument]
pub async fn start_servers(context: Arc<DauthContext>) {
    tracing::info!(
        "Hosting remote-facing RPC server on {}",
        context.rpc_context.host_addr
    );
    let host_ip: std::net::SocketAddr = context.rpc_context.host_addr.parse().unwrap();
    let external_server_join_handle = tokio::spawn(
        Server::builder()
            .add_service(HomeNetworkServer::new(HomeNetworkHandler {
                context: context.clone(),
            }))
            .add_service(BackupNetworkServer::new(BackupNetworkHandler {
                context: context.clone(),
            }))
            .serve(host_ip),
    );

    tracing::info!(
        "Hosting local-auth RPC server on {}",
        context.rpc_context.local_auth_addr
    );
    let local_ip: std::net::SocketAddr = context.rpc_context.local_auth_addr.parse().unwrap();
    let local_server_join_handle = tokio::spawn(
        Server::builder()
            .add_service(LocalAuthenticationServer::new(LocalAuthenticationHandler {
                context: context.clone(),
            }))
            .serve(local_ip),
    );

    // Select will await on each arm, and then return when the first task is
    // complete. Select! cancells the other future, but this is okay since we
    // want to shutdown all endpoints if one of the endpoints fails.
    tokio::select! {
        _ = external_server_join_handle => {
            tracing::error!("Remote facing RPC server exited!")
        }
        _ = local_server_join_handle => {
            tracing::error!("Remote facing RPC server exited!")
        }
    };
}
