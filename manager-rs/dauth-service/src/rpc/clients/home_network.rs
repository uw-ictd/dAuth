use std::sync::Arc;

use auth_vector::types::{HresStar, Kseaf, ResStar};

use crate::data::context::DauthContext;
use crate::data::error::DauthError;
use crate::data::vector::AuthVectorRes;

/// Get an auth vector from a user's home network.
pub async fn get_auth_vector(
    context: Arc<DauthContext>,
    user_id: &str,
    address: &str,
) -> Result<AuthVectorRes, DauthError> {
    todo!()
}

/// Get the kseaf value at the end of an auth vector transaction.
pub async fn get_confirm_key(
    context: Arc<DauthContext>,
    res_star: &ResStar,
    xres_star_hash: &HresStar,
    address: &str,
) -> Result<Kseaf, DauthError> {
    todo!()
}
