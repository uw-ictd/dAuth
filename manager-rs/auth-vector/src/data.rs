/// Contains all auth vector data
pub struct AuthVectorData {
    pub xres_star_hash: Vec<u8>,
    pub autn: Vec<u8>,
    pub rand: Vec<u8>,
    pub kseaf: Vec<u8>,
}
