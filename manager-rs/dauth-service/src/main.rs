mod data;
mod local;
mod remote;
mod rpc;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tracing::Level;
use tracing_subscriber;

use crate::data::context::{DauthContext, LocalContext, RemoteContext, RpcContext};
use crate::rpc::server;

#[tokio::main]
async fn main() {
    let context = Arc::new(DauthContext {
        local_context: LocalContext {
            database: Mutex::new(Box::new(HashMap::new())),
        },
        remote_context: RemoteContext {},
        rpc_context: RpcContext {
            host_addr: String::from("[::1]:50051"),
        },
    });

    // TODO(nickfh7) Add configuring for logging
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    server::start_server(context.clone()).await;
}
