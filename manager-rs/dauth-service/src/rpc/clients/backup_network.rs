use std::sync::Arc;

use auth_vector::types::{HresStar, Kseaf};

use crate::data::context::DauthContext;
use crate::data::error::DauthError;
use crate::data::vector::AuthVectorRes;

/// Request a network to become a backup network.
pub async fn enroll_backup_prepare(
    context: Arc<DauthContext>,
    user_id: &str,
    backup_network_id: &str,
    address: &str,
) -> Result<(), DauthError> {
    todo!()
}

/// Send the set of initial vectors and key shares after
/// a network has agreed to be a backup.
pub async fn enroll_backup_commit(
    context: Arc<DauthContext>,
    vectors: Vec<AuthVectorRes>,
    key_shares: Vec<(HresStar, Kseaf)>,
    address: &str,
) -> Result<(), DauthError> {
    todo!()
}

/// Get an auth vector from one of a user's backup networks.
pub async fn get_auth_vector(
    context: Arc<DauthContext>,
    user_id: &str,
    address: &str,
) -> Result<AuthVectorRes, DauthError> {
    todo!()
}

/// Get a key share from one of a user's backup networks.
pub async fn get_key_share(
    context: Arc<DauthContext>,
    xres_star_hash: HresStar,
    res_star: Kseaf,
    address: &str,
) -> Result<Kseaf, DauthError> {
    todo!()
}

/// Withdraws backup status from a backup network.
pub async fn withdraw_backup(
    context: Arc<DauthContext>,
    user_id: &str,
    backup_network_id: &str,
    address: &str,
) -> Result<(), DauthError> {
    todo!()
}

/// Withdraws all matching shares from a backup network.
pub async fn withdraw_shares(
    context: Arc<DauthContext>,
    xres_star_hashs: Vec<HresStar>,
    address: &str,
) -> Result<(), DauthError> {
    todo!()
}

/// Sends a flood vector to a backup network.
pub async fn flood_vector(
    context: Arc<DauthContext>,
    user_id: &str,
    vector: AuthVectorRes,
    address: &str,
) -> Result<AuthVectorRes, DauthError> {
    todo!()
}
