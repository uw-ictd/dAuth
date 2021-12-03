/// Contains all auth vector data
pub struct AuthVectorData {
    pub res: Vec<u8>,
    pub res_star: Vec<u8>,
    pub autn: Vec<u8>,
    pub rand: Vec<u8>,
    pub kseaf: Vec<u8>,
}
