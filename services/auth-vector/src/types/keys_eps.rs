use crate::types::AuthVectorConversionError;
use crate::types::{Ck, Ik, Autn};

use hmac::{Hmac};
use sha2::{Sha256};

pub const KASME_LENGTH: usize = 32;
pub const FC_KASME: u8 = 0x10;


#[derive(Debug,Copy,Clone,PartialEq,Eq)]
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
    pub fn derive(ck: &Ck, ik: &Ik, autn: &Autn) -> Self {
        let mut key = Vec::new();
        key.extend(ck);
        key.extend(ik);

        let mut data = vec![FC_KASME];
        let octet_1 = (crate::constants::MCC_BYTES[1] << 4) & crate::constants::MCC_BYTES[0];
        let octet_2 = (crate::constants::MNC_BYTES[2] << 4) & crate::constants::MCC_BYTES[2];
        let octet_3 = (crate::constants::MNC_BYTES[1] << 4) & crate::constants::MNC_BYTES[0];

        data.extend(vec![octet_1, octet_2, octet_3].iter());
        let sn_id_len = 0x03 as u16;
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

        Kasme{
            data: mac.finalize().into_bytes()[..KASME_LENGTH]
            .try_into()
            .expect("All data should have correct size")
         }
    }

    pub fn as_array(self) -> [u8; KASME_LENGTH] {
        self.data.clone()
    }

    pub fn to_vec(self) -> Vec<u8> {
        self.data.to_vec()
    }
}
