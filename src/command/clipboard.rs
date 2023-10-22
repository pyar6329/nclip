use super::*;
use crate::client::*;
use crate::config::Port;
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

        rx.await?
    }

    pub async fn copy(port: &Port, compressed_content: &Zstd) -> Result<(), Error> {
        let client = Self::create_client(port)?;
        let s = compressed_content.to_string();
        let (tx, rx) = oneshot::channel();
        tokio::spawn(async move {
            let maybe_content = ClipboardClient::post(&client, &s).await;
            let _ = tx.send(maybe_content);
        });
        let _ = rx.await?;
        Ok(())
    }

    fn create_client(port: &Port) -> Result<Client, Error> {
        let base_url: Url = Url::parse("http://localhost").unwrap();
        let client = Client::new(base_url, port, &TIMEOUT, &RETRY)?;
        Ok(client)
    }
}
