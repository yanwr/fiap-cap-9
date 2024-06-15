use crate::infra::errors::{AppError, ToBusinessError};
use data_encoding::BASE64;

pub struct HashBase64;
impl HashBase64 {
    pub fn decode(key: &str) -> Result<String, AppError> {
        let bytes = BASE64
            .decode(key.as_bytes())
            .map_err(|e| e.to_business_error("Failed to decode base64", None))?;
        String::from_utf8(bytes)
            .map_err(|e| e.to_business_error("Failed to convert base64 bytes to string", None))
    }

    pub fn encode(plain_text: String) -> Result<String, AppError> {
        Ok(BASE64.encode(plain_text.as_bytes()))
    }
}

#[cfg(test)]
mod test {
    use crate::{infra::errors::AppError, support::hash::base64::HashBase64};

    #[test]
    fn should_encode_and_decode_successfully() -> Result<(), AppError> {
        let encoded = HashBase64::encode(String::from("plain_text"))?;
        let decoded = HashBase64::decode(&encoded)?;

        assert_eq!(decoded, String::from("plain_text"));
        Ok(())
    }
}
