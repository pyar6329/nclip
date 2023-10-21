use super::*;
use crate::client::*;
use crate::config::Port;
use std::borrow::Cow;
use tokio::sync::oneshot;

const TIMEOUT: u8 = 5;
const RETRY: u8 = 15;

pub struct Clipboard;

impl Clipboard {
    pub async fn paste(port: &Port) -> Result<String, Error> {
        let client = Self::create_client(port)?;
        let (tx, rx) = oneshot::channel();
        tokio::spawn(async move {
            let maybe_content = ClipboardClient::get(&client).await;
            let _ = tx.send(maybe_content);
        });
        let content = rx.await?;
        Ok(content?)
    }

    pub async fn copy<T>(port: &Port, content: T) -> Result<(), Error>
    where
        T: Into<String>,
    {
        let client = Self::create_client(port)?;
        let s = content.into();
        println!("copy client aaaaaaaaa");
        let (tx, rx) = oneshot::channel();
        tokio::spawn(async move {
            // println!("copy client was called");
            let maybe_content = ClipboardClient::post(&client, &s).await;
            let _ = tx.send(maybe_content);
        });
        println!("copy client ggggggggggggg");
        let _ = rx.await?;
        println!("copy client nnnnnnnnnnnnnnn");
        Ok(())
    }

    fn create_client(port: &Port) -> Result<Client, Error> {
        let base_url: Url = Url::parse("http://localhost").unwrap();
        let client = Client::new(base_url, port, &TIMEOUT, &RETRY)?;
        Ok(client)
    }
}
