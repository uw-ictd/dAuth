pub mod data;
pub mod types;

use milenage::Milenage;
use rand as r;

use crate::data::AuthVectorData;
use crate::types::AuthVectorConversionError;

/// Uses provided k, opc, and rand with milenage.
/// Returns tuple of auth vector data (xres, rand, sqn_xor_ak, mac_a)
pub fn generate_vector(mcc: &str, mnc: &str, k: &types::K, opc: &types::Opc, sqn: &types::Sqn) -> AuthVectorData {
    let rand= types::Rand::new(&mut r::thread_rng());

    generate_vector_with_rand(mcc, mnc, k, opc, &rand, sqn)
}

/// Generate auth vector data with a provided rand
fn generate_vector_with_rand(
    mcc: &str, mnc: &str,
    k: &types::K,
    opc: &types::Opc,
    rand: &types::Rand,
    sqn: &types::Sqn,
) -> AuthVectorData {
    let mut m = Milenage::new_with_opc(k.clone(), opc.clone());

    let (xres, ck, ik, ak) = m.f2345(&rand.as_array());

    let xres_star = m
        .compute_res_star(mcc, mnc, &rand.as_array(), &xres)
        .unwrap();

    let xres_star_hash = types::gen_xres_star_hash(rand, &xres_star);
    let xres_hash = types::gen_xres_hash(rand, &xres);



    let autn = types::build_autn(sqn, &ak, rand, &mut m);

    let kseaf = types::gen_kseaf(mcc, mnc, &types::gen_kausf(mcc, mnc, &ck, &ik, &autn));
    let kasme = types::Kasme::derive(mcc, mnc, &ck, &ik, &autn);

    AuthVectorData {
        xres_star_hash,
        xres_star,
        autn,
        rand: rand.to_owned(),
        kseaf,
        kasme,
        xres_hash,
        xres,
    }
}

// Encode the PLMN for keying per TS 33.401: A.2-1
fn get_encoded_plmn(mcc: &str, mnc: &str) -> Result<Vec<u8>, AuthVectorConversionError> {
    if mcc.len() != 3 {
        return Err(AuthVectorConversionError::BoundsError());
    }

    if !(mnc.len() == 3 || mnc.len() == 2) {
        return Err(AuthVectorConversionError::BoundsError());
    }

    // Decode so each "digit" is mapped to a standalone byte.
    let mut mcc_bytes: Vec<u8> = Vec::new();
    for digit in mcc.chars() {
        mcc_bytes.push(u8::from_str_radix(&digit.to_string(), 10)?);
    }

    assert_eq!(mcc_bytes.len(), 3);

    let mut mnc_bytes: Vec<u8> = Vec::new();
    for digit in mnc.chars() {
        mnc_bytes.push(u8::from_str_radix(&digit.to_string(), 10)?);
    }

    if mnc_bytes.len() == 2 {
        mnc_bytes.push(0x0F);
    }

    assert_eq!(mnc_bytes.len(), 3);

    return Ok(vec![
        (mcc_bytes[1] << 4) | mcc_bytes[0],
        (mnc_bytes[2] << 4) | mcc_bytes[2],
        (mnc_bytes[1] << 4) | mnc_bytes[0],
    ])
}

#[cfg(test)]
mod tests {
    use std::{fmt::Write, num::ParseIntError};

    use hex;
    use milenage::Milenage;

    use crate::generate_vector_with_rand;
    use crate::get_encoded_plmn;
    use crate::types;

    #[test]
    fn test_generation_eps_5g_combined() {
        // Used from successful ueransim 5G attach
        let k: types::K = hex::decode("3aef49b1c6ee9700d42afca230cb0589")
            .unwrap()
            .try_into()
            .unwrap();
        let opc: types::Opc = hex::decode("e006d9ca10142b42b2830f3c603c3d63")
            .unwrap()
            .try_into()
            .unwrap();
        let rand: types::Rand = hex::decode("645f677edb96b100e3db14eae181c8e4")
            .unwrap()
            .try_into()
            .unwrap();
        let sqn: types::Sqn = hex::decode("000000003803").unwrap().try_into().unwrap();

        let result = generate_vector_with_rand("910", "54", &k, &opc, &rand, &sqn);

        // Shared EPS and 5G values:
        assert_eq!("645f677edb96b100e3db14eae181c8e4", hex::encode(result.rand.as_array()));
        assert_eq!("f37a166fd60880001e2530ed5a1e4d11", hex::encode(result.autn));

        // 5G-specific values:
        // assert_eq!(
        //     "4cc63b268aa5ff97516cc3ee0c5fad53",
        //     hex::encode(result.xres_star_hash)
        // ); // Need to confirm
        // assert_eq!(
        //     "b2cb4eda3b7fa56fb0bfefde811560a366836bcd2b14782d9293460efa9792af",
        //     hex::encode(result.kseaf)
        // ); // Need to confirm

        // EPS(4G)-specific values:
        assert_eq!("669c736ebf393c6e", hex::encode(result.xres));
        assert_eq!("3b85f4778a99be5324c7d233e4b68defc2db856a468125224f42f240e91926a5", hex::encode(result.kasme.as_array()));
    }

    #[test]
    fn test_plmn_encode() {
        // Used from successful ueransim 5G attach
        assert_eq!(get_encoded_plmn("910", "54").unwrap(), hex::decode("19f045").unwrap());

        // Test Edge Case Errors
        assert!(get_encoded_plmn("9", "54").is_err());
        assert!(get_encoded_plmn("910", "5").is_err());
        assert!(get_encoded_plmn("CAT", "5").is_err());
        assert!(get_encoded_plmn("9101337", "5").is_err());
        assert!(get_encoded_plmn("910", "51337").is_err());
    }

    #[test]
    fn test_generation() {
        // Used from successful ueransim 5G attach
        let k: types::K = hex::decode("465B5CE8B199B49FAA5F0A2EE238A6BC")
            .unwrap()
            .try_into()
            .unwrap();
        let opc: types::Opc = hex::decode("E8ED289DEBA952E4283B54E88E6183CA")
            .unwrap()
            .try_into()
            .unwrap();
        let rand: types::Rand = hex::decode("562d716dbd058b475cfecdbb48ed038f")
            .unwrap()
            .try_into()
            .unwrap();
        let sqn: types::Sqn = hex::decode("000000000021").unwrap().try_into().unwrap();

        let result = generate_vector_with_rand("901", "70", &k, &opc, &rand, &sqn);

        assert_eq!("562d716dbd058b475cfecdbb48ed038f", hex::encode(result.rand.as_array()));
        assert_eq!("67c325a93c6880006ed9f592d86b709c", hex::encode(result.autn));
        assert_eq!(
            "4cc63b268aa5ff97516cc3ee0c5fad53",
            hex::encode(result.xres_star_hash)
        ); // Need to confirm
        assert_eq!(
            "b2cb4eda3b7fa56fb0bfefde811560a366836bcd2b14782d9293460efa9792af",
            hex::encode(result.kseaf)
        ); // Need to confirm
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
