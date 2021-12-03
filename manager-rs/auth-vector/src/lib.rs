mod constants;
mod data;

use std::array::TryFromSliceError;

use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;

use milenage::Milenage;
use rand as r;

use crate::data::AuthVectorData;

/// Uses provided k, opc, and rand with milenage.
/// Returns tuple of auth vector data (xres, rand, sqn_xor_ak, mac_a)
pub fn generate_vector(
    k: &Vec<u8>,
    opc: &Vec<u8>,
    sqn: &Vec<u8>,
) -> Result<AuthVectorData, TryFromSliceError> {
    let rand: [u8; constants::RAND_LENGTH] = r::random();

    generate_vector_with_rand(k, opc, &Vec::from(rand), sqn)
}

/// Generate auth vector data with a provided rand
fn generate_vector_with_rand(
    k: &Vec<u8>,
    opc: &Vec<u8>,
    rand: &Vec<u8>,
    sqn: &Vec<u8>,
) -> Result<AuthVectorData, TryFromSliceError> {
    let k_a: [u8; constants::K_LENGTH] = k[..].try_into()?;
    let opc_a: [u8; constants::OPC_LENGTH] = opc[..].try_into()?;
    let rand_a: [u8; constants::RAND_LENGTH] = rand[..].try_into()?;
    let sqn_a: [u8; constants::SQN_LENGTH] = sqn[..].try_into()?;

    let mut m = Milenage::new_with_opc(k_a, opc_a);

    let (res, ck, ik, ak) = m.f2345(&rand_a);

    let res_star = m
        .compute_res_star(constants::MCC, constants::MNC, &rand_a, &res)
        .unwrap();

    let sqn_xor_ak: [u8; 6] = [
        sqn[0] ^ ak[0],
        sqn[1] ^ ak[1],
        sqn[2] ^ ak[2],
        sqn[3] ^ ak[3],
        sqn[4] ^ ak[4],
        sqn[5] ^ ak[5],
    ];

    let mac_a = m.f1(&rand_a, &sqn_a, &constants::AMF);

    let autn_a: [u8; constants::AUTN_LENGTH] =
        [&sqn_xor_ak[..], &constants::AMF[..], &mac_a[..]].concat()[..].try_into()?;
    let autn = Vec::from(autn_a);

    let kausf = gen_kausf(&Vec::from(ck), &Vec::from(ik), &autn);
    let kseaf = gen_kseaf(&kausf);

    Ok(AuthVectorData {
        res: Vec::from(res),
        res_star: Vec::from(res_star),
        autn: Vec::from(autn),
        rand: rand.clone(),
        kseaf: kseaf,
    })
}

fn gen_kausf(ck: &Vec<u8>, ik: &Vec<u8>, autn: &Vec<u8>) -> Vec<u8> {
    let mut key = ck.clone();
    key.extend(ik);

    let mut data = vec![constants::FC_KAUSF];
    data.extend(get_snn().as_bytes());
    data.extend(autn);

    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(&key).expect("HMAC can take key of any size");
    mac.update(&data);

    Vec::from(&mac.finalize().into_bytes()[..32])
}

fn gen_kseaf(kausf: &Vec<u8>) -> Vec<u8> {
    let mut data = vec![constants::FC_KSEAF];
    data.extend(get_snn().as_bytes());

    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(&kausf).expect("HMAC can take key of any size");
    mac.update(&data);

    Vec::from(&mac.finalize().into_bytes()[..32])
}

fn get_snn() -> String {
    if constants::MNC.len() == 2 {
        format!(
            "5G:mnc0{}.mcc{}.3gppnetwork.org",
            constants::MNC,
            constants::MCC,
        )
    } else {
        format!(
            "5G:mnc{}.mcc{}.3gppnetwork.org",
            constants::MNC,
            constants::MCC,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::{fmt::Write, num::ParseIntError};

    use hex;
    use milenage::Milenage;

    use crate::generate_vector_with_rand;

    #[test]
    fn test_generation() {
        // Used from successful ueransim 5G attach
        let k = hex::decode("465B5CE8B199B49FAA5F0A2EE238A6BC").unwrap();
        let opc = hex::decode("E8ED289DEBA952E4283B54E88E6183CA").unwrap();
        let rand = hex::decode("562d716dbd058b475cfecdbb48ed038f").unwrap();
        let sqn = hex::decode("000000000021").unwrap();

        let result = generate_vector_with_rand(&k, &opc, &rand, &sqn).unwrap();

        assert_eq!("fc9b23591b391885", hex::encode(result.res));
        assert_eq!(
            "60607d1246f9ab32569edf4c3cc18566",
            hex::encode(result.res_star)
        );
        assert_eq!("562d716dbd058b475cfecdbb48ed038f", hex::encode(result.rand));
        assert_eq!("67c325a93c6880006ed9f592d86b709c", hex::encode(result.autn));
        // assert_eq!(
        //     "0c25a657fc10c3a4c5cde321dd785032ef105d7392e5c9078412550af028cca9",
        //     hex::encode(result.kseaf)
        // ); // Need to confirm
    }

    /// INFORMATIONAL TESTS
    /// Intended only to show how milenage works
    #[test]
    fn test_sqn_from_ak() {
        // Used from successful ueransim 5G attach
        let k: [u8; 16] = decode_hex("465B5CE8B199B49FAA5F0A2EE238A6BC").unwrap()[..]
            .try_into()
            .unwrap();
        let opc: [u8; 16] = decode_hex("E8ED289DEBA952E4283B54E88E6183CA").unwrap()[..]
            .try_into()
            .unwrap();
        let rand: [u8; 16] = decode_hex("562d716dbd058b475cfecdbb48ed038f").unwrap()[..]
            .try_into()
            .unwrap();

        let sqn_xor_ak: [u8; 6] = decode_hex("67c325a93c68").unwrap()[..].try_into().unwrap();
        let amf: [u8; 2] = decode_hex("8000").unwrap()[..].try_into().unwrap();

        let mut m = Milenage::new_with_opc(k, opc);
        let (res, _ck, _ik, ak) = m.f2345(&rand);

        assert_eq!("67c325a93c49", encode_hex(&ak));
        assert_eq!("fc9b23591b391885", encode_hex(&res));
        // Res from pcap: 60607d1246f9ab32569edf4c3cc18566

        let sqn: [u8; 6] = [
            sqn_xor_ak[0] ^ ak[0],
            sqn_xor_ak[1] ^ ak[1],
            sqn_xor_ak[2] ^ ak[2],
            sqn_xor_ak[3] ^ ak[3],
            sqn_xor_ak[4] ^ ak[4],
            sqn_xor_ak[5] ^ ak[5],
        ];

        assert_eq!("000000000021", encode_hex(&sqn));

        let mac_a = m.f1(&rand, &sqn, &amf);

        assert_eq!("6ed9f592d86b709c", encode_hex(&mac_a));
    }

    #[test]
    fn test_milenage_f12345_res() {
        // Used from Test Set 2: https://www.etsi.org/deliver/etsi_ts/135200_135299/135208/06.00.00_60/ts_135208v060000p.pdf
        let k: [u8; 16] = decode_hex("465b5ce8b199b49faa5f0a2ee238a6bc").unwrap()[..]
            .try_into()
            .unwrap();
        let opc: [u8; 16] = decode_hex("cd63cb71954a9f4e48a5994e37a02baf").unwrap()[..]
            .try_into()
            .unwrap();
        let rand: [u8; 16] = decode_hex("23553cbe9637a89d218ae64dae47bf35").unwrap()[..]
            .try_into()
            .unwrap();
        let sqn: [u8; 6] = decode_hex("ff9bb4d0b607").unwrap()[..].try_into().unwrap();
        let amf: [u8; 2] = decode_hex("b9b9").unwrap()[..].try_into().unwrap();

        let mut m = Milenage::new_with_opc(k, opc);
        let (res, ck, ik, ak) = m.f2345(&rand);
        let mac_a = m.f1(&rand, &sqn, &amf);

        assert_eq!("a54211d5e3ba50bf", encode_hex(&res));
        assert_eq!("b40ba9a3c58b2a05bbf0d987b21bf8cb", encode_hex(&ck));
        assert_eq!("f769bcd751044604127672711c6d3441", encode_hex(&ik));
        assert_eq!("aa689c648370", encode_hex(&ak));
        assert_eq!("4a9ffac354dfafb3", encode_hex(&mac_a));
    }

    fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
            .collect()
    }

    fn encode_hex(bytes: &[u8]) -> String {
        let mut s = String::with_capacity(bytes.len() * 2);
        for &b in bytes {
            write!(&mut s, "{:02x}", b).unwrap();
        }
        s
    }
}
