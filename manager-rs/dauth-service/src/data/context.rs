use std::{
    collections::{HashMap, VecDeque},
    sync::Mutex,
};

use tonic::transport::Channel;

use crate::rpc::d_auth::remote_authentication_client::RemoteAuthenticationClient;
use crate::rpc::d_auth::AkaVectorResp;

/// Maintains the context for all components of
/// the dAuth service. All state exists here.
#[derive(Debug)]
pub struct DauthContext {
    pub local_context: LocalContext,
    pub remote_context: RemoteContext,
    pub rpc_context: RpcContext,
}

#[derive(Debug)]
pub struct LocalContext {
    pub database: Mutex<HashMap<Vec<u8>, VecDeque<AkaVectorResp>>>,
}

#[derive(Debug)]
pub struct RemoteContext {
    pub remote_addrs: Vec<String>,
}

#[derive(Debug)]
pub struct RpcContext {
    pub host_addr: String,
    pub client_stubs: Mutex<HashMap<String, RemoteAuthenticationClient<Channel>>>,
}
