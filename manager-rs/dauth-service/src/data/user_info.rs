/// Holds sensitive user info needed for auth vector generation
#[derive(Debug)]
pub struct UserInfo {
    pub k: [u8; 16],
    pub opc: [u8; 16],
    pub sqn_max: [u8; 6],
}

impl UserInfo {
    pub fn increment_sqn(&mut self, mut amount: u64) {
        let mut i = self.sqn_max.len() - 1;

        while amount > 0 {
            amount += self.sqn_max[i] as u64;
            self.sqn_max[i] = amount as u8;
            amount /= 256;

            if i == 0 {
                break;
            }
            i -= 1;
        }
    }
}