use std::{rc::Rc, sync::Arc};

use crate::data::{auth_vector::{AuthVectorRequest, AuthVectorResult}, context::DauthContext};
use crate::local;
use crate::rpc::clients;

pub fn auth_vector_get_remote(context: Arc<DauthContext>, av_request: Rc<AuthVectorRequest>)
-> Option<Rc<AuthVectorResult>> {
    println!("remote::manager::auth_vector_get_remote");
    local::manager::auth_vector_get(context, av_request)
}

pub fn auth_vector_used_remote(context: Arc<DauthContext>, av_result: Rc<AuthVectorResult>)
-> Result<(), &'static str> {
    println!("remote::manager::auth_vector_used_remote");
    local::manager::auth_vector_used(context, av_result)
}

pub fn auth_vector_send_request(context: Arc<DauthContext>, av_request: Rc<AuthVectorRequest>)
-> Option<Rc<AuthVectorResult>> {
    println!("remote::manager::auth_vector_send_request");
    clients::request_auth_vector_remote(context, av_request)
}

pub fn auth_vector_report_used(context: Arc<DauthContext>, av_result: Rc<AuthVectorResult>)
-> Result<(), &'static str> {
    println!("remote::manager::auth_vector_report_used");
    clients::broadcast_auth_vector_used(context, av_result)
}
