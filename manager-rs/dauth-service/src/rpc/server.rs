use std::sync::Arc;

use crate::data::context::DauthContext;
use crate::rpc::handler::DauthHandler;

use tonic::{transport::Server, Request, Response, Status};

pub mod d_auth {
    tonic::include_proto!("d_auth");
}

use d_auth::local_authentication_server::{LocalAuthentication, LocalAuthenticationServer};

pub async fn start_server(context: Arc<DauthContext>) {
    println!("Starting server");
    // Testing until rpc/proto is ready
    let handler = DauthHandler {
        context: context.clone(),
    };
    handler.auth_vector_get_local();
    handler.auth_vector_get_remote();
    handler.auth_vector_used_remote();

    println!("Server closing");
}
