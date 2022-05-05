use auth_vector::types::{Opc, Sqn, K};

/// Holds sensitive user info needed for auth vector generation
#[derive(Debug)]
pub struct UserInfo {
    pub k: K,
    pub opc: Opc,
    pub sqn_max: Sqn,
    pub sqn_slice: u32,
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

#[cfg(test)]
mod tests {
    use crate::data::user_info::UserInfo;

    #[test]
    fn test_increment() {
        assert!(check_increment_result(
            [0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0],
            0
        ));
        assert!(check_increment_result(
            [0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 1],
            1
        ));
        assert!(check_increment_result(
            [0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 2],
            2
        ));
        assert!(check_increment_result(
            [0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 255],
            255
        ));
        assert!(check_increment_result(
            [0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 1, 0],
            256
        ));
        assert!(check_increment_result(
            [0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 0, 0],
            256 * 256
        ));
        assert!(check_increment_result(
            [0, 0, 0, 0, 1, 0],
            [0, 0, 0, 0, 1, 1],
            1
        ));
        assert!(check_increment_result(
            [0, 0, 0, 0, 1, 0],
            [0, 0, 0, 0, 2, 0],
            256
        ));
        assert!(check_increment_result(
            [0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0],
            256 * 256 * 256 * 256 * 256 * 256
        ));
    }

    fn check_increment_result(
        v1: auth_vector::types::Sqn,
        v2: auth_vector::types::Sqn,
        amount: u64,
    ) -> bool {
        let mut ui = UserInfo {
            k: [0; auth_vector::constants::K_LENGTH],
            opc: [0; auth_vector::constants::OPC_LENGTH],
            sqn_max: v1,
            sqn_slice: 0,
        };

        ui.increment_sqn(amount);

        ui.sqn_max == v2
    }
}
