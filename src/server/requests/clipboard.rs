use super::*;
use crate::utils::*;
use anyhow::{Error, Result};
use arboard::Clipboard as Arboard;
use strum::EnumIs;
use thiserror::Error as ThisError;
use ClipboardError::*;

#[derive(ThisError, Copy, Clone, Debug, Eq, PartialEq, EnumIs)]
pub enum ClipboardError {
    #[error("Clipboard::new() was failed")]
    CannotCreateClipboardInstance,
    #[error("clipboard.get_text() was failed")]
    CannotGetClipboardString,
    #[error("clipboard.set_text() was failed")]
    CannotSetClipboardString,
}

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

#[derive(Clone)]
pub struct Clipboard;

impl Clipboard {
    pub fn get_clipboard() -> Result<String, Error> {
        let mut client = Arboard::new().map_err(|e| {
            err_msg(e.to_string());
            CannotCreateClipboardInstance
        })?;

        let data: String = client.get_text().map_err(|e| {
            err_msg(e.to_string());
            CannotGetClipboardString
        })?;

        Ok(data)
    }

    pub fn set_clipboard<S>(s: S) -> Result<(), Error>
    where
        S: AsRef<str>,
    {
        let text = s.as_ref();
        let mut clipboard = Arboard::new().map_err(|e| {
            err_msg(e.to_string());
            CannotCreateClipboardInstance
        })?;

        let _ = clipboard.set_text(text).map_err(|e| {
            err_msg(e.to_string());
            CannotSetClipboardString
        })?;

        Ok(())
    }
}
