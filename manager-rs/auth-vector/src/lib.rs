use milenage::Milenage;
use rand as r;

/// Uses provided k, opc, and rand with milenage.
/// Returns auth vector data (rand, autn, xres, kasme)
pub fn generate_vector(k: [u8; 16], opc: [u8; 16], sqn: [u8; 6]) {
    // generate rand
    let rand: [u8; 16] = r::random();

    milenage_generate_vector(k, opc, rand, sqn);
}

fn milenage_generate_vector(
    k: [u8; 16],
    opc: [u8; 16],
    rand: [u8; 16],
    sqn: [u8; 6],
) -> ([u8; 8], [u8; 16], [u8; 16], [u8; 6]) {
    let mut m = Milenage::new_with_opc(k, opc);

    let (res, ck, ik, ak) = m.f2345(&rand);

    // TODO(nickfh7) further process data and return
    (res, ck, ik, ak)
}

#[cfg(test)]
mod tests {
    use milenage::Milenage;
    use std::{fmt::Write, num::ParseIntError};

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
        let (_res, _ck, _ik, ak) = m.f2345(&rand);

        assert_eq!("67c325a93c49", encode_hex(&ak));

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
}
