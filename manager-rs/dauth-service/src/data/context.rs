use std::{
    collections::{HashMap, VecDeque},
    sync::Mutex,
};

use tokio::runtime::Handle;
use tonic::transport::Channel;

use crate::data::user_info::UserInfo;
use crate::rpc::dauth::common::home_network_client::HomeNetworkClient;
use crate::rpc::dauth::common::AkaVectorResp;

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
    pub user_info_database: Mutex<HashMap<Vec<u8>, UserInfo>>,
    pub local_user_id_min: Vec<u8>,
    pub local_user_id_max: Vec<u8>,
}

#[derive(Debug)]
pub struct RemoteContext {
    pub remote_addrs: Vec<String>,
}

#[derive(Debug)]
pub struct RpcContext {
    pub runtime_handle: Handle,
    pub host_addr: String,
    pub client_stubs: tokio::sync::Mutex<HashMap<String, HomeNetworkClient<Channel>>>,
}
