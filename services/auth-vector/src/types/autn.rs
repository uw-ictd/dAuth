use crate::types::{Sqn, Ak, Rand};
use crate::types::SQN_LENGTH;

pub const AMF_LENGTH: usize = 2;
pub const AMF: [u8; AMF_LENGTH] = [0x80, 0x00];

pub const AUTN_LENGTH: usize = SQN_LENGTH + AMF_LENGTH + MAC_LENGTH;
pub type Autn = [u8; AUTN_LENGTH];

const MAC_LENGTH: usize = 8;
type Mac = [u8; MAC_LENGTH];


pub fn build_autn(sqn: &Sqn, ak: &Ak, rand: &Rand, m: &mut milenage::Milenage) -> Autn {

    let sqn_xor_ak: Sqn = sqn
    .as_bytes()
    .iter()
    .zip(ak.iter())
    .map(|(a, b)| a ^ b)
    .collect::<Vec<u8>>()[..]
    .try_into()
    .expect("All data should have correct size");

    let mac: Mac = m.f1(&rand.as_array(), &sqn.as_bytes(), &AMF);

    let mut autn: Autn = [0; AUTN_LENGTH];

    autn[..SQN_LENGTH].copy_from_slice(sqn_xor_ak.as_bytes());
    autn[SQN_LENGTH..(SQN_LENGTH + AMF_LENGTH)]
        .copy_from_slice(&AMF[..]);
    autn[(SQN_LENGTH + AMF_LENGTH)..].copy_from_slice(&mac);

    autn
}
