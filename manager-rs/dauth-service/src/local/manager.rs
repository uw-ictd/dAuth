use std::{rc::Rc, sync::Arc};

use crate::data::{
    auth_vector::{AuthVectorRequest, AuthVectorResult},
    context::DauthContext,
};
use crate::local;
use crate::remote;

/// Attempts to find or possibly generate a new auth vector
/// Order of checks:
/// 1. Check local database (returns and deletes, if found)
/// 2. Generate if
pub fn auth_vector_get(
    context: Arc<DauthContext>,
    av_request: Rc<AuthVectorRequest>,
) -> Option<Rc<AuthVectorResult>> {
    println!("local::manager::auth_vector_get");
    if let Some(av_result) =
        local::database::auth_vector_lookup(context.clone(), av_request.clone())
    {
        match auth_vector_used(context.clone(), av_result.clone()) {
            _ => (),
        }
        match remote::manager::auth_vector_report_used(context.clone(), av_result.clone()) {
            _ => (),
        }
        Some(av_result)
    } else if let Some(av_result) =
        remote::manager::auth_vector_send_request(context.clone(), av_request.clone())
    {
        Some(av_result)
    } else {
        None
    }
}

pub fn auth_vector_used(
    context: Arc<DauthContext>,
    av_result: Rc<AuthVectorResult>,
) -> Result<(), &'static str> {
    println!("local::manager::auth_vector_used");
    local::database::auth_vector_delete(context, av_result)
}
