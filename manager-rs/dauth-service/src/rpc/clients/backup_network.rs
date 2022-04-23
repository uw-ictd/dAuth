use std::sync::Arc;

use crate::data::context::DauthContext;
use crate::data::error::DauthError;
use crate::data::vector::AuthVectorRes;

/// Get an auth vector from one of a user's backup networks.
pub async fn get_auth_vector(
    context: Arc<DauthContext>,
    user_id: &str,
    address: &str,
) -> Result<AuthVectorRes, DauthError> {
    todo!()
}
