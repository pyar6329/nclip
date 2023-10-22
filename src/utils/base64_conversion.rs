use super::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Base64(String);

impl<T> From<T> for Base64
where
    T: Into<String>,
{
    // set base64 string directly
    fn from(s: T) -> Self {
        Self(s.into())
    }
}

impl Deref for Base64 {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Base64 {
    // convert <string> -> <base64>
    pub fn encode<T: AsRef<[u8]>>(input: &T) -> Self {
        let string = general_purpose::STANDARD.encode(input.as_ref());
        Self::from(&string)
    }

    // convert <base64> -> <string>
    pub fn decode(&self) -> Result<String, DecodeError> {
        let decoded_bytes = general_purpose::STANDARD.decode(self.to_string())?;
        let decoded_str = str::from_utf8(&decoded_bytes).map_err(|e| {
            let invalid_index = e.valid_up_to();
            let invalid_value = decoded_bytes.get(invalid_index).unwrap_or(&0_u8);
            DecodeError::InvalidByte(invalid_index, *invalid_value)
        })?;
        Ok(decoded_str.to_string())
    }

    // convert <base64> -> <binary>
    pub fn decode_as_bytes(&self) -> Result<Vec<u8>, DecodeError> {
        let decoded_bytes = general_purpose::STANDARD.decode(self.to_string())?;
        Ok(decoded_bytes)
    }
}
