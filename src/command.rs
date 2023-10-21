mod arg;
mod clipboard;

use crate::server;
use anyhow::{anyhow, Error, Result};
pub use arg::*;
pub use clipboard::*;
use std::process;
use url::Url;

pub async fn run() {
    let mode = Arg::get();

    let port = mode.get_port();

    if mode.is_server_command() {
        server::run(&port).await
    }

    if mode.is_copy_command() {
        println!("--copy is start! port: {}", &port);
        let content = get_stdin().await;
        let result = Clipboard::copy(&port, &content).await;
        if result.is_ok() {
            process::exit(0)
        } else {
            process::exit(1)
        }
    }

    if mode.is_paste_command() {
        let result = Clipboard::paste(&port).await;
        if let Ok(content) = result {
            post_stdout(&content).await;
            process::exit(0)
        } else {
            process::exit(1)
        }
    }
}

pub async fn get_stdin() -> String {
    "hello".to_string()
}

pub async fn post_stdout(s: &str) {
    // TODO: print!ではなく、tokioでBufWriteでやる
    print!("{}", s);
}
