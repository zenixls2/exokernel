#![warn(unused_imports)]
#![warn(dead_code)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate docopt;
extern crate mioco;
extern crate net2;
extern crate time;

use docopt::Docopt;
use std::net::{IpAddr};

mod server;
mod bbs;
mod protocols;
mod user;

const DEFAULT_PORT: u16 = 8080;
const DEFAULT_ADDR: &'static str = "127.0.0.1";

macro_rules! HELP_MESSAGE {($($arg:tt)*) => (format!(r#"
Usage: {name} [--addr=<ADDR>] [--port=<PORT>]
       {name} --help
Options:
  --help       Show this help message.
  --addr=<ADDR>  IP address [default: '{}'].
  --port=<PORT>  Port number [default: {}]."#, $($arg)*))
}

#[derive(Debug, Deserialize)]
struct Args {
    flag_addr: Option<String>,
    flag_port: Option<u16>
}

fn main() {
    let args: Args = Docopt::new(HELP_MESSAGE!(DEFAULT_ADDR, DEFAULT_PORT,
                                               name=env!("CARGO_PKG_NAME")))
                    .and_then(|d| d.deserialize())
                    .unwrap_or_else(|e| e.exit());
    let addr: IpAddr = args.flag_addr.unwrap_or(String::from(DEFAULT_ADDR))
                                     .parse().unwrap_or(DEFAULT_ADDR.parse().unwrap());
    let port: u16 = args.flag_port.unwrap_or(DEFAULT_PORT);
    println!("Listen to {} {}", addr, port);
    server::run(addr, port);
}
