use crate::types;

/// Contains all auth vector data
pub struct AuthVectorData {
    pub xres_star_hash: types::XResStarHash,
    pub xres_star: types::XResStar,
    pub autn: types::Autn,
    pub rand: types::Rand,
    pub kseaf: types::Kseaf,
    pub kasme: types::Kasme,
    pub xres_hash: types::XResHash,
    pub xres: types::XRes,
}
