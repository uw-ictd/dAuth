use crate::data::error::DauthError;

/// Converts string to array of bytes
pub fn convert_string_to_byte_vec(s: &str) -> Result<Vec<u8>, DauthError> {
    match hex::decode(s) {
        Ok(v) => Ok(v),
        Err(e) => Err(DauthError::DataError(format!("{}", e))),
    }
}

/// Compares two *equal sized* byte vectors
/// Returns v1 < v2
pub fn byte_vec_less_than(v1: &Vec<u8>, v2: &Vec<u8>) -> Result<bool, DauthError> {
    if v1.len() != v2.len() {
        return Err(DauthError::DataError(format!(
            "Cannot compare two vecs of different length: {:?} < {:?}",
            v1, v2
        )));
    }

    for (a, b) in v1.iter().zip(v2) {
        if a < b {
            return Ok(true);
        } else if a > b {
            return Ok(false);
        }
    }

    Ok(false) // At lease one element must be less
}

/// Compares two *equal sized* byte vectors
/// Returns v1 == v2
pub fn byte_vec_equal(v1: &Vec<u8>, v2: &Vec<u8>) -> Result<bool, DauthError> {
    if v1.len() != v2.len() {
        return Err(DauthError::DataError(format!(
            "Cannot compare two vecs of different length: {:?} < {:?}",
            v1, v2
        )));
    }

    Ok(v1.iter().zip(v2).filter(|&(a, b)| a != b).count() == 0)
}

/// Compares two *equal sized* byte vectors
/// Returns v1 <= v2
pub fn byte_vec_less_or_equal(v1: &Vec<u8>, v2: &Vec<u8>) -> Result<bool, DauthError> {
    if v1.len() != v2.len() {
        return Err(DauthError::DataError(format!(
            "Cannot compare two vecs of different length: {:?} < {:?}",
            v1, v2
        )));
    }

    Ok(byte_vec_less_than(v1, v2)? || byte_vec_equal(v1, v2)?)
}

#[cfg(test)]
mod tests {
    use crate::data::utilities;

    #[test]
    fn test_less_than() {
        assert!(utilities::byte_vec_less_than(&vec![1, 2, 2], &vec![1, 2, 3]).unwrap());
        assert!(utilities::byte_vec_less_than(&vec![1, 2, 3], &vec![2, 1, 0]).unwrap());
        assert!(!utilities::byte_vec_less_than(&vec![1, 2, 3], &vec![1, 2, 3]).unwrap());
        assert!(!utilities::byte_vec_less_than(&vec![2, 2, 3], &vec![1, 2, 3]).unwrap());
        assert!(!utilities::byte_vec_less_than(&vec![], &vec![]).unwrap());
    }

    #[test]
    fn test_equal() {
        assert!(utilities::byte_vec_equal(&vec![1, 1, 1], &vec![1, 1, 1]).unwrap());
        assert!(utilities::byte_vec_equal(&vec![], &vec![]).unwrap());
        assert!(!utilities::byte_vec_equal(&vec![1, 2, 3], &vec![1, 2, 4]).unwrap());
        assert!(!utilities::byte_vec_equal(&vec![2, 2, 3], &vec![1, 2, 3]).unwrap());
    }

    #[test]
    fn test_less_or_equal() {
        assert!(utilities::byte_vec_less_or_equal(&vec![1, 2, 2], &vec![1, 2, 3]).unwrap());
        assert!(utilities::byte_vec_less_or_equal(&vec![1, 2, 3], &vec![2, 1, 0]).unwrap());
        assert!(utilities::byte_vec_less_or_equal(&vec![1, 1, 1], &vec![1, 1, 1]).unwrap());
        assert!(utilities::byte_vec_less_or_equal(&vec![], &vec![]).unwrap());
        assert!(!utilities::byte_vec_less_or_equal(&vec![1, 2, 4], &vec![1, 2, 3]).unwrap());
        assert!(!utilities::byte_vec_less_or_equal(&vec![2, 2, 3], &vec![1, 2, 3]).unwrap());
    }

    #[test]
    fn test_bad_vec() {
        assert!(utilities::byte_vec_less_than(&vec![1, 1, 1], &vec![1, 1]).is_err());
        assert!(utilities::byte_vec_less_than(&vec![1, 1, 1], &vec![]).is_err());
        assert!(utilities::byte_vec_equal(&vec![1, 1, 1], &vec![1, 1]).is_err());
        assert!(utilities::byte_vec_equal(&vec![1, 1, 1], &vec![]).is_err());
        assert!(utilities::byte_vec_less_or_equal(&vec![1, 1, 1], &vec![1, 1]).is_err());
        assert!(utilities::byte_vec_less_or_equal(&vec![1, 1, 1], &vec![]).is_err());
    }
}
