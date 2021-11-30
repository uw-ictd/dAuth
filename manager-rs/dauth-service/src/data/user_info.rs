/// Holds sensitive user info needed for auth vector generation
#[derive(Debug)]
pub struct UserInfo {
    pub k: Vec<u8>,
    pub opc: Vec<u8>,
    pub sqn_max: Vec<u8>,
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
        assert!(check_increment_result(vec![0, 0, 0], vec![0, 0, 0], 0));
        assert!(check_increment_result(vec![0, 0, 0], vec![0, 0, 1], 1));
        assert!(check_increment_result(vec![0, 0, 0], vec![0, 0, 2], 2));
        assert!(check_increment_result(vec![0, 0, 0], vec![0, 0, 255], 255));
        assert!(check_increment_result(vec![0, 0, 0], vec![0, 1, 0], 256));
        assert!(check_increment_result(
            vec![0, 0, 0],
            vec![1, 0, 0],
            256 * 256
        ));
        assert!(check_increment_result(vec![0, 1, 0], vec![0, 1, 1], 1));
        assert!(check_increment_result(vec![0, 1, 0], vec![0, 2, 0], 256));
        assert!(check_increment_result(
            vec![0, 0, 0],
            vec![0, 0, 0],
            256 * 256 * 256
        ));
    }

    fn check_increment_result(v1: Vec<u8>, v2: Vec<u8>, amount: u64) -> bool {
        let mut ui = UserInfo {
            k: vec![],
            opc: vec![],
            sqn_max: v1,
        };

        ui.increment_sqn(amount);

        ui.sqn_max == v2
    }
}
