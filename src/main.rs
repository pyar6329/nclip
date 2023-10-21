// extern crate openssl;

use nclip::command;

#[tokio::main]
async fn main() {
    command::run().await
}
