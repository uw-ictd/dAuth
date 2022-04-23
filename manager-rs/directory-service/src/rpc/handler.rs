use std::sync::Arc;

use crate::manager;
use crate::data::context::DirectoryContext;
use crate::rpc::directory_service::directory_server::Directory;
use crate::rpc::directory_service::{
    LookupUserReq, LookupUserResp, LooukupNetworkReq, LooukupNetworkResp, RegisterReq,
    RegisterResp, UpsertUserReq, UpsertUserResp,
};

/// Handles all RPC calls to the dAuth service.
pub struct DirectoryHandler {
    pub context: Arc<DirectoryContext>,
}

#[tonic::async_trait]
impl Directory for DirectoryHandler {
    async fn register(
        &self,
        request: tonic::Request<RegisterReq>,
    ) -> Result<tonic::Response<RegisterResp>, tonic::Status> {
        tracing::info!("New request: {:?}", request);
        
        let content = request.into_inner();
        
        match manager::register(self.context.clone(), &content.network_id, &content.address, &content.public_key).await {
            Ok(()) => Ok(tonic::Response::new(RegisterResp {})),
            Err(e) => {
                tracing::warn!("Request failed: {:?}", e);
                Err(tonic::Status::new(tonic::Code::Aborted, e.to_string()))
            }
        }
    }

    async fn lookup_network(
        &self,
        request: tonic::Request<LooukupNetworkReq>,
    ) -> Result<tonic::Response<LooukupNetworkResp>, tonic::Status> {
        tracing::info!("New request: {:?}", request);
        
        let content = request.into_inner();
        
        match manager::lookup_network(self.context.clone(), &content.network_id).await {
            Ok((address, public_key)) => Ok(tonic::Response::new(LooukupNetworkResp { address, public_key })),
            Err(e) => {
                tracing::warn!("Request failed: {:?}", e);
                Err(tonic::Status::new(tonic::Code::Aborted, e.to_string()))
            }
        }
    }

    async fn lookup_user(
        &self,
        request: tonic::Request<LookupUserReq>,
    ) -> Result<tonic::Response<LookupUserResp>, tonic::Status> {
        tracing::info!("New request: {:?}", request);
        
        let content = request.into_inner();
        
        match manager::lookup_user(self.context.clone(), &content.user_id).await {
            Ok((home_network_id, backup_network_ids)) => Ok(tonic::Response::new(LookupUserResp { home_network_id, backup_network_ids })),
            Err(e) => {
                tracing::warn!("Request failed: {:?}", e);
                Err(tonic::Status::new(tonic::Code::Aborted, e.to_string()))
            }
        }
    }

    async fn upsert_user(
        &self,
        request: tonic::Request<UpsertUserReq>,
    ) -> Result<tonic::Response<UpsertUserResp>, tonic::Status> {
        tracing::info!("New request: {:?}", request);
        
        let content = request.into_inner();
        
        match manager::upsert_user(self.context.clone(), &content.user_id, &content.home_network_id, &content.backup_network_ids).await {
            Ok(()) => Ok(tonic::Response::new(UpsertUserResp {})),
            Err(e) => {
                tracing::warn!("Request failed: {:?}", e);
                Err(tonic::Status::new(tonic::Code::Aborted, e.to_string()))
            }
        }
    }
}
