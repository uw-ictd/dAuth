use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::types::{Ck, Ik, Autn};

pub const KAUSF_LENGTH: usize = 32;
pub const KSEAF_LENGTH: usize = 32;
const FC_KAUSF: u8 = 0x6A;
const FC_KSEAF: u8 = 0x6C;

pub type Kausf = [u8; KAUSF_LENGTH];
pub type Kseaf = [u8; KSEAF_LENGTH];

pub fn gen_kausf(mcc: &str, mnc: &str, ck: &Ck, ik: &Ik, autn: &Autn) -> Kausf {
    let mut key = Vec::new();
    key.extend(ck);
    key.extend(ik);

    let mut data = vec![FC_KAUSF];

    data.extend(get_snn(mcc, mnc).as_bytes());
    let snn_len = get_snn(mcc, mnc).as_bytes().len() as u16;
    let snn_len = snn_len.to_be_bytes();
    data.extend(snn_len);

    let sqn_xor_ak = &autn[..6];
    data.extend(sqn_xor_ak);
    let autn_len = sqn_xor_ak.len() as u16;
    let autn_len = autn_len.to_be_bytes();
    data.extend(autn_len);

    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(&key).expect("HMAC can take key of any size");
    mac.update(&data);

    mac.finalize().into_bytes()[..KAUSF_LENGTH]
        .try_into()
        .expect("All data should have correct size")
}

pub fn gen_kseaf(mcc: &str, mnc: &str, kausf: &Kausf) -> Kseaf {
    let mut data = vec![FC_KSEAF];
    data.extend(get_snn(mcc, mnc).as_bytes());
    let snn_len = get_snn(mcc, mnc).as_bytes().len() as u16;
    let snn_len = snn_len.to_be_bytes();
    data.extend(snn_len);

    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(kausf).expect("HMAC can take key of any size");
    mac.update(&data);

    mac.finalize().into_bytes()[..32]
        .try_into()
        .expect("All data should have correct size")
}

fn get_snn(mcc: &str, mnc: &str) -> String {
    if mnc.len() == 2 {
        format!(
            "5G:mnc0{}.mcc{}.3gppnetwork.org",
            mnc,
            mcc,
        )
    } else {
        format!(
            "5G:mnc{}.mcc{}.3gppnetwork.org",
            mnc,
            mcc,
        )
    }
}
