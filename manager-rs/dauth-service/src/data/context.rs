use std::{
    collections::{HashMap, VecDeque},
    sync::Mutex,
};

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
    pub database: Mutex<Box<HashMap<Vec<u8>, VecDeque<AkaVectorResp>>>>,
}

#[derive(Debug)]
pub struct RemoteContext {}

#[derive(Debug)]
pub struct RpcContext {
    pub host_addr: String,
}
