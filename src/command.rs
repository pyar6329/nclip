mod arg;
mod clipboard;

use crate::server;
use crate::utils::Zstd;
use anyhow::{Error, Result};
pub use arg::Mode::*;
pub use arg::*;
pub use clipboard::*;
use std::process;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use url::Url;

pub async fn run() {
    let mode = Arg::get();

    match mode {
        ServerCommand(port) => {
            server::run(&port).await;
            process::exit(0)
        }
        CopyCommand(port) => {
            let content = get_stdin().await;
            let compressed_content = Zstd::encode(&content);
            let result = Clipboard::copy(&port, &compressed_content).await;
            if result.is_ok() {
                process::exit(0)
            } else {
                process::exit(1)
            }
        }
        PasteCommand(port) => {
            let result = Clipboard::paste(&port).await;
            if let Ok(compressed_content) = result {
                let content = compressed_content.decode().unwrap_or_default();
                post_stdout(&content).await;
                process::exit(0)
            } else {
                process::exit(1)
            }
        }
    }
}

pub async fn get_stdin() -> String {
    let mut stdin = io::stdin();
    let mut buffer = vec![];
    let _ = stdin.read_to_end(&mut buffer).await;
    let mut context = String::from_utf8(buffer).ok().unwrap_or_default();
    context.pop(); // remove last new line character ('\n')
    context
}

pub async fn post_stdout(s: &str) {
    let mut stdout = io::stdout();
    let _ = stdout.write_all(s.as_bytes()).await;
}
