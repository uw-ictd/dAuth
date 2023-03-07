use auth_vector::types::{Opc, K};

/// Holds sensitive user info needed for auth vector generation
#[derive(Debug)]
pub struct UserInfo {
    pub id: String,
    pub k: K,
    pub opc: Opc,
    pub sqn: i64,
}
