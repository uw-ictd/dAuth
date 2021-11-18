/// Contains all auth vector data
pub struct AuthVectorData {
    pub res: Vec<u8>,
    pub res_star: Vec<u8>,
    pub rand: Vec<u8>,
    pub sqn_xor_ak: Vec<u8>,
    pub mac_a: Vec<u8>
}
