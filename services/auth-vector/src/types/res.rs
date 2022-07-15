use sha2::{Digest, Sha256};

use crate::types::{Rand};

pub const XRES_LENGTH: usize = 8;
pub const XRES_HASH_LENGTH: usize = 16;
pub const XRES_STAR_LENGTH: usize = 16;
pub const XRES_STAR_HASH_LENGTH: usize = 16;

pub type XRes = [u8; XRES_LENGTH];
pub type XResHash = [u8; XRES_HASH_LENGTH];
pub type XResStar = [u8; XRES_STAR_LENGTH];
pub type XResStarHash = [u8; XRES_STAR_HASH_LENGTH];

// Separate types for the res, received from the actual UE. It will be the
// "same" format as the XRes computed by the core network even though the type
// has a different semantic meaning in the 3gpp specs.
pub type Res = XRes;
pub type ResStar = XResStar;

pub fn gen_xres_star_hash(rand: &Rand, xres_star: &XResStar) -> XResStarHash {
    let mut data = Vec::new();
    data.extend(rand.as_array());
    data.extend(xres_star);

    let mut hasher = Sha256::new();
    hasher.update(data);

    hasher.finalize()[16..32]
        .try_into()
        .expect("All data should have correct size")
}

pub fn gen_xres_hash(rand: &Rand, xres_star: &XRes) -> XResHash {
    let mut data = Vec::new();
    data.extend(rand.as_array());
    data.extend(xres_star);

    let mut hasher = Sha256::new();
    hasher.update(data);

    hasher.finalize()[16..32]
        .try_into()
        .expect("All data should have correct size")
}
