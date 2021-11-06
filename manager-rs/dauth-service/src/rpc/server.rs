use std::sync::Arc;

use crate::data::context::DauthContext;
use crate::rpc::handler::DauthHandler;

pub fn start_server(context: Arc<DauthContext>) {
    println!("Starting server");
    // Testing until rpc/proto is ready
    let handler = DauthHandler { context: context.clone() };
    handler.auth_vector_get_local();
    handler.auth_vector_get_remote();
    handler.auth_vector_used_remote();

    println!("Server closing");
}