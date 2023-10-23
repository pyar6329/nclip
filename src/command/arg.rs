use crate::config::{Port, DEFAULT_PORT};
use clap::Parser;
use strum::EnumIs;
use Mode::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arg {
    #[arg(short, long, help = "copy from stdin")]
    pub copy: bool,
    #[arg(short, long, default_value_t = DEFAULT_PORT, help = "running port")]
    pub port: Port,
    #[arg(short, long, help = "running server")]
    pub server: bool,
}

impl Arg {
    pub fn get() -> Mode {
        let args = Arg::parse();
        if args.server {
            return ServerCommand(args.port);
        }
        if args.copy {
            return CopyCommand(args.port);
        }
        PasteCommand(args.port)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumIs)]
pub enum Mode {
    ServerCommand(Port),
    CopyCommand(Port),
    PasteCommand(Port),
}
