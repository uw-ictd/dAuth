use std::sync::Arc;

use crate::data::context::DirectoryContext;
use crate::data::error::DirectoryError;
use crate::rpc::directory_service::directory_server::Directory;
use crate::rpc::directory_service::{RegisterReq, RegisterResp, LookupUserReq, LookupUserResp, LooukupNetworkReq, LooukupNetworkResp, UpsertUserReq, UpsertUserResp};

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
        todo!()
    }

    async fn lookup_network(
        &self,
        request: tonic::Request<LooukupNetworkReq>,
    ) -> Result<tonic::Response<LooukupNetworkResp>, tonic::Status> {
        todo!()
    }

    async fn lookup_user(
        &self,
        request: tonic::Request<LookupUserReq>,
    ) -> Result<tonic::Response<LookupUserResp>, tonic::Status> {
        todo!()
    }

    async fn upsert_user(
        &self,
        request: tonic::Request<UpsertUserReq>,
    ) -> Result<tonic::Response<UpsertUserResp>, tonic::Status> {
        todo!()
    }
}
