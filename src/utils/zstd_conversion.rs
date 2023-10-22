use super::*;

const COMPRESSION_LEVEL: u8 = 3;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Zstd(Base64);

impl<T> From<T> for Zstd
where
    T: Into<String>,
{
    // set base64's zstd string directly
    fn from(s: T) -> Self {
        Self(Base64::from(s))
    }
}

impl Deref for Zstd {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Zstd {
    fn get_base64(&self) -> Base64 {
        self.0.to_owned()
    }

    pub fn encode(s: &str) -> Self {
        let compressed_bytes =
            zstd::encode_all(s.as_bytes(), COMPRESSION_LEVEL as i32).unwrap_or_default();
        let compressed_base64 = Base64::encode(&compressed_bytes);
        Zstd(compressed_base64)
    }

    pub fn decode(&self) -> Result<String, Error> {
        let compressed_bytes = self.get_base64().decode_as_bytes()?;
        let extracted_bytes: Vec<u8> = zstd::decode_all(compressed_bytes.as_slice())?;
        let decoded_text = str::from_utf8(&extracted_bytes)?;
        Ok(decoded_text.to_string())
    }

    pub async fn async_encode(s: &str) -> Self {
        Self::encode(s)
    }

    pub async fn async_decode(&self) -> Result<String, Error> {
        self.decode()
    }
}
