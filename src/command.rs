use clap::Parser;
use strum::EnumIs;
use Mode::*;

pub type Port = u16;
// const DEFAULT_PORT: Port = 2230;
const DEFAULT_PORT: Port = 5800;

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
            return CopyCommand;
        }
        PasteCommand
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumIs)]
pub enum Mode {
    ServerCommand(Port),
    CopyCommand,
    PasteCommand,
}

impl Mode {
    pub fn get_port(&self) -> Port {
        match *self {
            ServerCommand(port) => port,
            _ => DEFAULT_PORT,
        }
    }
}
