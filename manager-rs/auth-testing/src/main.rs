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

fn test_sqn_from_ak(k_str: &str, opc_str: &str, rand_str: &str, sqn_xor_str: &str, amf_str: &str) {
    println!("k:     {}", k_str);
    println!("opc:   {}", opc_str);
    println!("rand:  {}", rand_str);
    println!("sqn_x: {}", sqn_xor_str);
    println!("amf:   {}", amf_str);

    // Used from successful ueransim 5G attach
    let k: [u8; 16] = decode_hex(k_str).unwrap()[..]
        .try_into()
        .unwrap();
    let opc: [u8; 16] = decode_hex(opc_str).unwrap()[..]
        .try_into()
        .unwrap();
    let rand: [u8; 16] = decode_hex(rand_str).unwrap()[..]
        .try_into()
        .unwrap();
    let sqn_xor_ak: [u8; 6] = decode_hex(sqn_xor_str).unwrap()[..].try_into().unwrap();
    let amf: [u8; 2] = decode_hex(amf_str).unwrap()[..].try_into().unwrap();

    let mut m = Milenage::new_with_opc(k, opc);
    let (_res, _ck, _ik, ak) = m.f2345(&rand);

    let sqn: [u8; 6] = [
        sqn_xor_ak[0] ^ ak[0],
        sqn_xor_ak[1] ^ ak[1],
        sqn_xor_ak[2] ^ ak[2],
        sqn_xor_ak[3] ^ ak[3],
        sqn_xor_ak[4] ^ ak[4],
        sqn_xor_ak[5] ^ ak[5],
    ];

    let mac_a = m.f1(&rand, &sqn, &amf);
    let mac_s = m.f1star(&rand, &sqn, &amf);
    let ak_rs = m.f5star(&rand);

    println!("ak:    {}", encode_hex(&ak));
    println!("sqn:   {}", encode_hex(&sqn));
    println!("mac_a: {}", encode_hex(&mac_a));
    println!("mac_s: {}", encode_hex(&mac_s));
    println!("ak_rs: {}", encode_hex(&ak_rs));
}

fn main() {
    test_sqn_from_ak(
        "465B5CE8B199B49FAA5F0A2EE238A6BC",
        "E8ED289DEBA952E4283B54E88E6183CA",
        "562d716dbd058b475cfecdbb48ed038f",
        "67c325a93c68",
        "8000",
    );
}
