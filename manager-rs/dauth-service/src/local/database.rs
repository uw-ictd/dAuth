use std::{rc::Rc, sync::Arc};

use crate::data::{
    auth_vector::{AuthVectorRequest, AuthVectorResult},
    context::DauthContext,
};

pub fn auth_vector_lookup(
    context: Arc<DauthContext>,
    av_request: Rc<AuthVectorRequest>,
) -> Option<Rc<AuthVectorResult>> {
    println!("local::database::auth_vector_lookup");
    Some(Rc::new(AuthVectorResult {}))
}

pub fn auth_vector_delete(
    context: Arc<DauthContext>,
    av_result: Rc<AuthVectorResult>,
) -> Result<(), &'static str> {
    println!("local::database::auth_vector_delete");
    Ok(())
}
