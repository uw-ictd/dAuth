use std::{rc::Rc, sync::Arc};

use crate::{data::{auth_vector::{AuthVectorRequest, AuthVectorResult}, context::DauthContext}, local};

pub fn request_auth_vector_remote(context: Arc<DauthContext>, av_request: Rc<AuthVectorRequest>)
-> Option<Rc<AuthVectorResult>> {
    println!("rpc::clients::request_auth_vector_remote");
    Some(Rc::new(AuthVectorResult {}))
}

pub fn broadcast_auth_vector_used(context: Arc<DauthContext>, av_result: Rc<AuthVectorResult>)
-> Result<(), &'static str> {
    println!("rpc::clients::broadcast_auth_vector_used");
    Ok(())
}
