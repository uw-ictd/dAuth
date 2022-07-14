use crate::constants;

use thiserror::Error;
use hmac::{Hmac};
use sha2::{Digest, Sha256};

/// General error type for dAuth service failures
#[derive(Error, Debug)]
pub enum AuthVectorConversionError {
    #[error("Conversion outside limits of destination type")]
    BoundsError(),

    #[error("Conversion outside limits of destination type")]
    SliceConversionError(#[from] std::array::TryFromSliceError),
}

pub type Id = String;
pub type K = [u8; constants::K_LENGTH];
pub type Opc = [u8; constants::OPC_LENGTH];
pub type Mac = [u8; constants::MAC_LENGTH];
pub type ResStar = [u8; constants::RES_STAR_LENGTH];
pub type Ck = [u8; constants::CK_LENGTH];
pub type Ik = [u8; constants::IK_LENGTH];
pub type Kausf = [u8; constants::KAUSF_LENGTH];
pub type Kseaf = [u8; constants::KSEAF_LENGTH];

pub type Xres = [u8; constants::XRES_LENGTH];
pub type XresHash = [u8; constants::XRES_HASH_LENGTH];

pub type ResStarHash = [u8; constants::RES_STAR_HASH_LENGTH];
pub type Autn = [u8; constants::AUTN_LENGTH];

#[derive(Debug, Clone, Copy)]
pub struct Sqn {
    data: [u8; constants::SQN_LENGTH],
}

impl Sqn {
    pub fn as_bytes(&self) -> &[u8; constants::SQN_LENGTH] {
        &self.data
    }
}

impl Into<i64> for Sqn {
    /// Converts sqn_bytes to seqnum int
    fn into(self) -> i64 {
        i64::from_be_bytes([
            0,
            0,
            self.data[0],
            self.data[1],
            self.data[2],
            self.data[3],
            self.data[4],
            self.data[5],
        ])
    }
}

impl TryFrom<i64> for Sqn {
    type Error = AuthVectorConversionError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value < 0 || value >= (0x01 << 48) {
            return Err(AuthVectorConversionError::BoundsError());
        }

        // Convert the sqn i64 to a 48 bit bytestring of the low-magnitude bits.
        value.to_be_bytes()[2..].try_into()
    }
}

impl TryFrom<&[u8]> for Sqn {
    type Error = AuthVectorConversionError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != constants::SQN_LENGTH {
            return Err(AuthVectorConversionError::BoundsError());
        }

        Ok(Sqn {
            data: value.try_into()?,
        })
    }
}

impl TryFrom<&Vec<u8>> for Sqn {
    type Error = AuthVectorConversionError;
    fn try_from(value: &Vec<u8>) -> Result<Self, Self::Error> {
        value.as_slice().try_into()
    }
}

impl TryFrom<Vec<u8>> for Sqn {
    type Error = AuthVectorConversionError;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        value.as_slice().try_into()
    }
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub struct Rand {
    data: [u8; constants::RAND_LENGTH],
}

impl TryFrom<&[u8]> for Rand {
    type Error = AuthVectorConversionError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != constants::RAND_LENGTH {
            return Err(AuthVectorConversionError::BoundsError());
        }

        Ok(Rand {
            data: value.try_into()?,
        })
    }
}

impl TryFrom<&Vec<u8>> for Rand {
    type Error = AuthVectorConversionError;
    fn try_from(value: &Vec<u8>) -> Result<Self, Self::Error> {
        value.as_slice().try_into()
    }
}

impl TryFrom<Vec<u8>> for Rand {
    type Error = AuthVectorConversionError;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        value.as_slice().try_into()
    }
}

impl Rand {
    pub fn new<T: rand::Rng>(r: &mut T) -> Self {
        let data_array: [u8; constants::RAND_LENGTH] = r.gen();
        Rand { data: data_array }
    }

    pub fn as_array(self) -> [u8; constants::RAND_LENGTH] {
        self.data.clone()
    }

    pub fn to_vec(self) -> Vec<u8> {
        self.data.to_vec()
    }
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub struct Kasme {
    data: [u8; constants::KASME_LENGTH],
}

impl TryFrom<&[u8]> for Kasme {
    type Error = AuthVectorConversionError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != constants::KASME_LENGTH {
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

        let mut data = vec![constants::FC_KASME];
        let octet_1 = (constants::MCC_BYTES[1] << 4) & constants::MCC_BYTES[0];
        let octet_2 = (constants::MNC_BYTES[2] << 4) & constants::MCC_BYTES[2];
        let octet_3 = (constants::MNC_BYTES[1] << 4) & constants::MNC_BYTES[0];

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
            data: mac.finalize().into_bytes()[..constants::KASME_LENGTH]
            .try_into()
            .expect("All data should have correct size")
         }
    }

    pub fn as_array(self) -> [u8; constants::KASME_LENGTH] {
        self.data.clone()
    }

    pub fn to_vec(self) -> Vec<u8> {
        self.data.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqn_bytes_into_int() {
        let integer_rep: i64 = Sqn::try_from(vec![0, 0, 0, 0, 1, 0]).unwrap().into();

        assert_eq!(integer_rep, 256);
    }
}
