mod autn;
mod keys_5g;
mod keys_eps;
mod res;

pub use autn::*;
pub use keys_5g::*;
pub use keys_eps::*;
pub use res::*;

use thiserror::Error;

pub const ID_LENGTH: usize = 7;
pub const K_LENGTH: usize = 16;
pub const OPC_LENGTH: usize = 16;
pub const RAND_LENGTH: usize = 16;
pub const CK_LENGTH: usize = 16;
pub const IK_LENGTH: usize = 16;
pub const SQN_LENGTH: usize = 6;

/// General error type for dAuth service failures
#[derive(Error, Debug)]
pub enum AuthVectorConversionError {
    #[error("Conversion outside limits of destination type")]
    BoundsError(),

    #[error("Conversion outside limits of destination type")]
    SliceConversionError(#[from] std::array::TryFromSliceError),

    #[error("Could not convert hex input")]
    HexConversionError(#[from] hex::FromHexError),

    #[error("Could not parse integer input")]
    IntConversionError(#[from] std::num::ParseIntError),
}

pub type Id = String;
pub type K = [u8; K_LENGTH];
pub type Opc = [u8; OPC_LENGTH];
pub type Ck = [u8; CK_LENGTH];
pub type Ik = [u8; IK_LENGTH];
pub type Ak = [u8; 6];

#[derive(Debug, Clone, Copy)]
pub struct Sqn {
    data: [u8; SQN_LENGTH],
}

impl Sqn {
    pub fn as_bytes(&self) -> &[u8; SQN_LENGTH] {
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
        if value.len() != SQN_LENGTH {
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rand {
    data: [u8; RAND_LENGTH],
}

impl TryFrom<&[u8]> for Rand {
    type Error = AuthVectorConversionError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != RAND_LENGTH {
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
        let data_array: [u8; RAND_LENGTH] = r.gen();
        Rand { data: data_array }
    }

    pub fn as_array(self) -> [u8; RAND_LENGTH] {
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
