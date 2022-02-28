use std::collections::HashMap;
use ed25519_dalek::Keypair;
use sqlx::SqlitePool;
use tokio::runtime::Handle;
use tonic::transport::Channel;

use auth_vector::types::Id;

use crate::data::user_info::UserInfo;
use crate::rpc::dauth::remote::home_network_client::HomeNetworkClient;

/// Maintains the context for all components of
/// the dAuth service. All state exists here.
#[derive(Debug)]
pub struct DauthContext {
    pub local_context: LocalContext,
    pub remote_context: RemoteContext,
    pub rpc_context: RpcContext,
    pub database_context: DatabaseContext,
}

#[derive(Debug)]
pub struct LocalContext {
    pub user_info_database: tokio::sync::Mutex<HashMap<Id, UserInfo>>,
    pub local_user_id_min: Id,
    pub local_user_id_max: Id,
    pub signing_keys: Keypair,
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

#[derive(Debug)]
pub struct DatabaseContext {
    pub database_path: String,
    pub pool: SqlitePool,
}
