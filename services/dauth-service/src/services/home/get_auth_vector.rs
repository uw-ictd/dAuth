use std::sync::Arc;

use crate::common;
use crate::data::{context::DauthContext, error::DauthError, vector::AuthVectorRes};

/// Generates an auth vector that will be verified locally.
/// Stores the kseaf directly, without key shares.
#[tracing::instrument(skip(context), name = "home::get_auth_vector")]
pub async fn get_auth_vector(
    context: Arc<DauthContext>,
    user_id: &str,
    serving_network_id: &str,
) -> Result<AuthVectorRes, DauthError> {
    tracing::info!("Generating new vector for requesting network");

    // TODO: Additional checking to verify requesting network

    common::auth_vectors::generate_local_vector(context, user_id).await
}
