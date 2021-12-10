use auth_vector::types::Id;

use crate::data::error::DauthError;

/// Converts hex string to byte vec
pub fn convert_hex_string_to_byte_vec(s: &str) -> Result<Vec<u8>, DauthError> {
    let mut ns = String::from(s);
    if ns.len() % 2 == 1 {
        ns = format!("0{}", ns)
    }
    match hex::decode(ns) {
        Ok(v) => {
            let mut i = 0;
            for e in &v {
                if *e != 0 {
                    return Ok(v[i..].to_vec());
                }
                i += 1;
            }
            return Ok(vec![0]);
        }
        Err(e) => Err(DauthError::DataError(format!("{}", e))),
    }
}

/// Converts decimal string to byte vec
/// NOTE: DOES NOT ALLOW DECIMAL REPRESENATIONS LARGER THAN 128 BITS!
pub fn convert_int_string_to_byte_vec(s: &str) -> Result<Vec<u8>, DauthError> {
    match s.parse::<u128>() {
        Ok(dec) => {
            let mut ns = format!("{:x}", dec);
            if ns.len() % 2 == 1 {
                ns = format!("0{}", ns)
            }
            match hex::decode(ns) {
                Ok(v) => Ok(v),
                Err(e) => Err(DauthError::DataError(format!("{}", e))),
            }
        }
        Err(e) => Err(DauthError::DataError(format!("{}", e))),
    }
}

/// Pads the vector with zeros up to length
/// Returns result of vector, or error if vector is bigger than length
pub fn zero_pad(v: Vec<u8>, length: usize) -> Result<Vec<u8>, DauthError> {
    if v.len() > length {
        Err(DauthError::DataError(format!(
            "Data is larger than max: {}/{}; {:?}",
            v.len(),
            length,
            v
        )))
    } else {
        let mut n = vec![0; length - v.len()];
        n.extend(v);
        Ok(n)
    }
}

/// Converts hex string to array of bytes
/// Pads resulting byte array with zeros
pub fn convert_hex_string_to_byte_vec_with_length(
    s: &str,
    length: usize,
) -> Result<Vec<u8>, DauthError> {
    zero_pad(convert_hex_string_to_byte_vec(s)?, length)
}

/// Converts hex string to array of bytes
/// Pads resulting byte array with zeros
pub fn convert_int_string_to_byte_vec_with_length(
    s: &str,
    length: usize,
) -> Result<Vec<u8>, DauthError> {
    zero_pad(convert_int_string_to_byte_vec(s)?, length)
}

/// Compares two ids as unsigned integers
/// Returns id1 < id2
pub fn id_less_than(id1: &Id, id2: &Id) -> bool {
    for (a, b) in id1.iter().zip(id2) {
        if a < b {
            return true;
        } else if a > b {
            return false;
        }
    }

    false // At lease one element must be less
}

/// Compares two ids as unsigned integers
/// Returns id1 == id2
pub fn id_equal(id1: &Id, id2: &Id) -> bool {
    id1.iter().zip(id2).filter(|&(a, b)| a != b).count() == 0
}

/// Compares two ids as unsigned integers
/// Returns id1 <= id2
pub fn id_less_or_equal(id1: &Id, id2: &Id) -> bool {
    id_less_than(id1, id2) || id_equal(id1, id2)
}

#[cfg(test)]
mod tests {
    use crate::data::utilities;

    #[test]
    fn test_string_hex_to_byte_vec() {
        assert_eq!(
            utilities::convert_hex_string_to_byte_vec("0").unwrap(),
            vec![0x00]
        );
        assert_eq!(
            utilities::convert_hex_string_to_byte_vec("1").unwrap(),
            vec![0x01]
        );
        assert_eq!(
            utilities::convert_hex_string_to_byte_vec("01").unwrap(),
            vec![0x01]
        );
        assert_eq!(
            utilities::convert_hex_string_to_byte_vec("0000000000000000001").unwrap(),
            vec![0x01]
        );
        assert_eq!(
            utilities::convert_hex_string_to_byte_vec("ff").unwrap(),
            vec![0xff]
        );
        assert_eq!(
            utilities::convert_hex_string_to_byte_vec("100").unwrap(),
            vec![0x01, 0x00]
        );
        assert_eq!(
            utilities::convert_hex_string_to_byte_vec("0334176431A801").unwrap(),
            vec![0x03, 0x34, 0x17, 0x64, 0x31, 0xA8, 0x01]
        );
    }

    #[test]
    fn test_string_int_to_byte_vec() {
        assert_eq!(
            utilities::convert_int_string_to_byte_vec("0").unwrap(),
            vec![0x00]
        );
        assert_eq!(
            utilities::convert_int_string_to_byte_vec("1").unwrap(),
            vec![0x01]
        );
        assert_eq!(
            utilities::convert_int_string_to_byte_vec("01").unwrap(),
            vec![0x01]
        );
        assert_eq!(
            utilities::convert_int_string_to_byte_vec("0000000000000000001").unwrap(),
            vec![0x01]
        );
        assert_eq!(
            utilities::convert_int_string_to_byte_vec("255").unwrap(),
            vec![0xff]
        );
        assert_eq!(
            utilities::convert_int_string_to_byte_vec("256").unwrap(),
            vec![0x01, 0x00]
        );
        assert_eq!(
            utilities::convert_int_string_to_byte_vec("901700000000001").unwrap(),
            vec![0x03, 0x34, 0x17, 0x64, 0x31, 0xA8, 0x01]
        );
    }

    #[test]
    fn test_string_hex_to_byte_vec_with_length() {
        assert!(utilities::convert_hex_string_to_byte_vec_with_length("1", 0).is_err());
        assert!(utilities::convert_hex_string_to_byte_vec_with_length("1ff", 1).is_err());
        assert!(utilities::convert_hex_string_to_byte_vec_with_length("1ffff", 2).is_err());
        assert_eq!(
            utilities::convert_hex_string_to_byte_vec_with_length("1", 1).unwrap(),
            vec![0x01]
        );
        assert_eq!(
            utilities::convert_hex_string_to_byte_vec_with_length("1", 2).unwrap(),
            vec![0x00, 0x01]
        );
        assert_eq!(
            utilities::convert_hex_string_to_byte_vec_with_length("1", 3).unwrap(),
            vec![0x00, 0x00, 0x01]
        );
        assert_eq!(
            utilities::convert_hex_string_to_byte_vec_with_length("1ff", 2).unwrap(),
            vec![0x01, 0xff]
        );
        assert_eq!(
            utilities::convert_hex_string_to_byte_vec_with_length("1ff", 3).unwrap(),
            vec![0x00, 0x01, 0xff]
        );
    }

    #[test]
    fn test_string_int_to_byte_vec_with_length() {
        assert!(utilities::convert_int_string_to_byte_vec_with_length("1", 0).is_err());
        assert!(utilities::convert_int_string_to_byte_vec_with_length("256", 1).is_err());
        assert!(utilities::convert_int_string_to_byte_vec_with_length("65536", 2).is_err());
        assert_eq!(
            utilities::convert_int_string_to_byte_vec_with_length("1", 1).unwrap(),
            vec![0x01]
        );
        assert_eq!(
            utilities::convert_int_string_to_byte_vec_with_length("1", 2).unwrap(),
            vec![0x00, 0x01]
        );
        assert_eq!(
            utilities::convert_int_string_to_byte_vec_with_length("1", 3).unwrap(),
            vec![0x00, 0x00, 0x01]
        );
        assert_eq!(
            utilities::convert_int_string_to_byte_vec_with_length("256", 2).unwrap(),
            vec![0x01, 0x00]
        );
        assert_eq!(
            utilities::convert_int_string_to_byte_vec_with_length("256", 3).unwrap(),
            vec![0x00, 0x01, 0x00]
        );
    }

    #[test]
    fn test_less_than() {
        assert!(utilities::id_less_than(
            &[0, 0, 0, 0, 1, 2, 2],
            &[0, 0, 0, 0, 1, 2, 3]
        ));
        assert!(utilities::id_less_than(
            &[0, 0, 0, 0, 1, 2, 3],
            &[0, 0, 0, 0, 2, 1, 0]
        ));
        assert!(!utilities::id_less_than(
            &[0, 0, 0, 0, 1, 2, 3],
            &[0, 0, 0, 0, 1, 2, 3]
        ));
        assert!(!utilities::id_less_than(
            &[0, 0, 0, 0, 2, 2, 3],
            &[0, 0, 0, 0, 1, 2, 3]
        ));
    }

    #[test]
    fn test_equal() {
        assert!(utilities::id_equal(
            &[0, 0, 0, 0, 1, 1, 1],
            &[0, 0, 0, 0, 1, 1, 1]
        ));
        assert!(!utilities::id_equal(
            &[0, 0, 0, 0, 1, 2, 3],
            &[0, 0, 0, 0, 1, 2, 4]
        ));
        assert!(!utilities::id_equal(
            &[0, 0, 0, 0, 2, 2, 3],
            &[0, 0, 0, 0, 1, 2, 3]
        ));
    }

    #[test]
    fn test_less_or_equal() {
        assert!(utilities::id_less_or_equal(
            &[0, 0, 0, 0, 1, 2, 2],
            &[0, 0, 0, 0, 1, 2, 3]
        ));
        assert!(utilities::id_less_or_equal(
            &[0, 0, 0, 0, 1, 2, 3],
            &[0, 0, 0, 0, 2, 1, 0]
        ));
        assert!(utilities::id_less_or_equal(
            &[0, 0, 0, 0, 1, 1, 1],
            &[0, 0, 0, 0, 1, 1, 1]
        ));
        assert!(!utilities::id_less_or_equal(
            &[0, 0, 0, 0, 1, 2, 4],
            &[0, 0, 0, 0, 1, 2, 3]
        ));
        assert!(!utilities::id_less_or_equal(
            &[0, 0, 0, 0, 2, 2, 3],
            &[0, 0, 0, 0, 1, 2, 3]
        ));
    }
}
