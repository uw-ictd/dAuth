use auth_vector::types::{Opc, K};

/// Holds sensitive user info needed for auth vector generation
#[derive(Debug)]
pub struct UserInfo {
    pub k: K,
    pub opc: Opc,
    pub sqn: u64,
}
