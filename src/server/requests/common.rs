use super::*;

#[derive(Deserialize, Extractible, Debug)]
#[salvo(extract(default_source(from = "body", format = "json")))]
pub struct RequestCommon<'a> {
    pub content: &'a str,
}

#[derive(Debug, Serialize)]
pub struct ResponseCommon<'a> {
    pub status: u16,
    pub content: &'a str,
}
