use crate::data::error::DauthError;

pub fn convert_string_to_byte_vec(s: &str) -> Result<Vec<u8>, DauthError> {
    match hex::decode(s) {
        Ok(v) => Ok(v),
        Err(e) => Err(DauthError::DataError(format!("{}", e))),
    }
}
