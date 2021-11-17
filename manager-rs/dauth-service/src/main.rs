mod data;
mod local;
mod remote;
mod rpc;

use hex;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::runtime::Handle;
use tracing::Level;
use tracing_subscriber;

use crate::data::{
    context::{DauthContext, LocalContext, RemoteContext, RpcContext},
    user_info::UserInfo,
};
use crate::rpc::server;

#[tokio::main]
async fn main() {
    let context = Arc::new(DauthContext {
        local_context: LocalContext {
            database: Mutex::new(HashMap::new()),
            user_info_database: Mutex::new(HashMap::from([
                // Sample user
                (
                    hex::decode("0334176431A801").unwrap(),
                    UserInfo {
                        k: hex::decode("465B5CE8B199B49FAA5F0A2EE238A6BC").unwrap()[..]
                            .try_into()
                            .unwrap(),
                        opc: hex::decode("E8ED289DEBA952E4283B54E88E6183CA").unwrap()[..]
                            .try_into()
                            .unwrap(),
                        sqn_max: hex::decode("000000000021").unwrap()[..].try_into().unwrap(),
                    },
                ),
            ])),
        },
        remote_context: RemoteContext {
            remote_addrs: vec![String::from("[::1]:50051")],
        },
        rpc_context: RpcContext {
            runtime_handle: Handle::current(),
            host_addr: String::from("[::1]:50051"),
            client_stubs: tokio::sync::Mutex::new(HashMap::new()),
        },
    });

    // TODO(nickfh7) Add configuring for logging
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    server::start_server(context.clone()).await;
}
