use auth_vector::types::{Rand, XResStarHash};

#[derive(Clone, Debug)]
pub enum AuthSource {
    HomeNetwork,
    BackupNetwork,
}

#[derive(Clone, Debug)]
pub struct AuthState {
    pub rand: Rand,
    pub source: AuthSource,
    pub xres_star_hash: XResStarHash,
}
