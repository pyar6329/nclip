// extern crate openssl;

use nclip::command;
use nclip::server;

fn main() {
    let args = command::Arg::get();

    if args.is_server_command() {
        server::run(&args.get_port())
    }

    if args.is_copy_command() {
        println!("--copy is start!");
    }
}
