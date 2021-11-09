use milenage::Milenage;

/// Uses provided k, opc, and rand with milenage.
/// Returns auth vector data (rand, autn, xres, kasme)
pub fn generate_auth_vector(k: [u8; 16], opc: [u8; 16], rand: [u8; 16]) {

    // let mut m = Milenage::new_with_op(k, op);
    let mut m = Milenage::new_with_opc(k, opc);
    let (res, ck, ik, ak) = m.f2345(&rand);

    // TODO(nickfh7) further process data and set return
}
