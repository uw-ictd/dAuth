use crate::constants;

use thiserror::Error;

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

pub type HresStar = [u8; constants::RES_STAR_HASH_LENGTH];
pub type Rand = [u8; constants::RAND_LENGTH];
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
        i64::from_be_bytes([0, 0, self.data[0], self.data[1], self.data[2], self.data[3], self.data[4], self.data[5]])
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

        Ok(Sqn{data: value.try_into()?})
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqn_bytes_into_int() {
        let integer_rep: i64 = Sqn::try_from(vec![0,0,0,0,1,0]).unwrap().into();

        assert_eq!(integer_rep, 256);
    }
}
