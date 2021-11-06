mod local;
mod remote;
mod rpc;
mod data;

use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::data::context::{DauthContext, LocalContext, RemoteContext, RpcContext};
use crate::rpc::server;

fn main() {
    let context = Arc::new(DauthContext {
        local_context: LocalContext {
            database: Mutex::new(Box::new(HashMap::new())),
        },
        remote_context: RemoteContext {},
        rpc_context: RpcContext { 
            host_addr: String::from("[..1]:50051"),
        },
    });

    server::start_server(context.clone());
}
