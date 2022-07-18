use crate::get_encoded_plmn;
use crate::types::AuthVectorConversionError;
use crate::types::{Autn, Ck, Ik};

use hmac::Hmac;
use sha2::Sha256;

pub const KASME_LENGTH: usize = 32;
pub const FC_KASME: u8 = 0x10;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Kasme {
    data: [u8; KASME_LENGTH],
}

impl TryFrom<&[u8]> for Kasme {
    type Error = AuthVectorConversionError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != KASME_LENGTH {
            return Err(AuthVectorConversionError::BoundsError());
        }

        Ok(Kasme {
            data: value.try_into()?,
        })
    }
}

impl TryFrom<&Vec<u8>> for Kasme {
    type Error = AuthVectorConversionError;
    fn try_from(value: &Vec<u8>) -> Result<Self, Self::Error> {
        value.as_slice().try_into()
    }
}

impl TryFrom<Vec<u8>> for Kasme {
    type Error = AuthVectorConversionError;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        value.as_slice().try_into()
    }
}

impl Kasme {
    pub fn derive(mcc: &str, mnc: &str, ck: &Ck, ik: &Ik, autn: &Autn) -> Self {
        let mut key = Vec::new();
        key.extend(ck);
        key.extend(ik);

        let mut data = vec![FC_KASME];

        let mut sn_id = get_encoded_plmn(mcc, mnc).unwrap();
        let sn_id_len = sn_id.len() as u16;
        data.append(&mut sn_id);
        data.extend(sn_id_len.to_be_bytes());

        let sqn_xor_ak = &autn[..6];
        data.extend(sqn_xor_ak);
        let autn_len = sqn_xor_ak.len() as u16;
        let autn_len = autn_len.to_be_bytes();
        data.extend(autn_len);

        use hmac::Mac;
        type HmacSha256 = Hmac<Sha256>;
        let mut mac = HmacSha256::new_from_slice(&key).expect("HMAC can take key of any size");
        mac.update(&data);

        Kasme {
            data: mac.finalize().into_bytes()[..KASME_LENGTH]
                .try_into()
                .expect("All data should have correct size"),
        }
    }

    pub fn as_array(self) -> [u8; KASME_LENGTH] {
        self.data.clone()
    }

    pub fn to_vec(self) -> Vec<u8> {
        self.data.to_vec()
    }
}
