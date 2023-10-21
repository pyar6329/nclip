use super::*;

#[derive(Deserialize, Extractible, Debug)]
#[salvo(extract(default_source(from = "body", format = "json")))]
pub struct RequestClipboard<'a> {
    pub content: &'a str,
}

#[derive(Debug, Serialize)]
pub struct ResponseClipboard<'a> {
    pub status: u16,
    pub content: &'a str,
}
