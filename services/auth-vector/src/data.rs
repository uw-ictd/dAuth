use crate::types;

/// Contains all auth vector data
pub struct AuthVectorData {
    pub xres_star_hash: types::ResStarHash,
    pub xres_star: types::ResStar,
    pub autn: types::Autn,
    pub rand: types::Rand,
    pub kseaf: types::Kseaf,
    pub kasme: types::Kasme,
    pub xres_hash: types::XresHash,
    pub xres: types::Xres,
}
