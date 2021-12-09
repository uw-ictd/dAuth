use crate::types;

/// Contains all auth vector data
pub struct AuthVectorData {
    pub xres_star_hash: types::HresStar,
    pub autn: types::Autn,
    pub rand: types::Rand,
    pub kseaf: types::Kseaf,
}
