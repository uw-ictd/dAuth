use std::rc::Rc;
use std::sync::Arc;

use crate::data::auth_vector::{AuthVectorRequest, AuthVectorResult};
use crate::data::context::DauthContext;
use crate::local;
use crate::remote;

/// Handles all RPC calls to the dAuth service.
pub struct DauthHandler {
    pub context: Arc<DauthContext>,
}

impl DauthHandler {
    /// Local (home) core request for a vector
    pub fn auth_vector_get_local(&self) {
        println!("--- RPC call for local vector request");
        println!("rpc::DauthHandler::auth_vector_generate_local");
        local::manager::auth_vector_get(self.context.clone(), Rc::new(AuthVectorRequest {}));
        println!("--- RPC call complete");
    }

    /// Remote core request for a vector
    pub fn auth_vector_get_remote(&self) {
        println!("--- RPC call for remote vector request");
        println!("rpc::DauthHandler::auth_vector_generate_remote");
        remote::manager::auth_vector_get_remote(self.context.clone(), Rc::new(AuthVectorRequest {}));
        println!("--- RPC call complete");
    }

    /// Remote alert that a vector has been used
    pub fn auth_vector_used_remote(&self) {
        println!("--- RPC call for remote vector used");
        println!("rpc::DauthHandler::auth_vector_used_remote");
        match remote::manager::auth_vector_used_remote(self.context.clone(), Rc::new(AuthVectorResult {})) {
            Ok(()) => (),
            Err(e) => println!("Error reporting used vector: {}", e)
        };
        println!("--- RPC call complete");
    }
}
