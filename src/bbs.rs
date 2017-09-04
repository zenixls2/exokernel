#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
extern crate mioco;

use std::vec::Vec;
use std::io::{ErrorKind};
use std::result::Result;
use mioco::tcp::{TcpStream};
use std::io::{Read, BufWriter, Write, stdout};
use protocols::telnet::*;
use std::sync::mpsc::{channel, Sender, Receiver};
use user::User;

pub enum BBSError {
    ConnectionClosed,
}

enum State {
    Default = 0,
    Negotiate = 1,
    TTY = 2,
}

macro_rules! u8_to_u16 {
    ($from:tt) => {
        (($from[0] as u16) << 8) + ($from[1] as u16)
    };
}

macro_rules! printflush {
    ($fmt:expr) => {
        print!($fmt);
        let _ = stdout().flush();
    };
    ($fmt:expr, $($arg:tt)*) => {
        print!($fmt, $($arg)*);
        let _ = stdout().flush();
    };
}

pub type BBSResult = Result<i32, BBSError>;

pub fn run(stream: TcpStream) -> BBSResult {
    let (tx, rx) = channel::<u8>();
    let mut user = User::new();
    let mut _stream = stream.try_clone().unwrap();
    mioco::spawn(move || {
        reader_loop(&mut _stream, &mut user, tx);
    });
    mioco::spawn(move || {
        writer_loop(stream, rx);
    });
    Ok(0)
}

fn writer_loop(stream: TcpStream, rx: Receiver<u8>) {
    let mut _stream = stream.try_clone().unwrap();
    let mut writer = BufWriter::new(stream.try_clone().unwrap());
    let _ = writer.write(&TELNET_INIT).unwrap();
    let _ = writer.write(&TELNET_RESIZABLE).unwrap();
    let _ = writeln!(writer, "Hello World");
    let _ = writer.flush();
    loop {
        let mut b = [rx.recv().unwrap()];
        match b[0] {
            IAC => break,
            _ => _stream.write(&mut b),
        };
    }
}


fn reader_loop(stream: &mut TcpStream, user: &mut User, tx: Sender<u8>) {
    let mut state = State::Default;
    let mut byte: [u8; 1] = [0];
    loop {
        //let size = stream.read_exact(&mut byte).unwrap();
        match stream.read(&mut byte) {
            Ok(0) => break,
            Ok(..) => {},
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => {
                println!("{}", e); break;
            },
        }
        let b = byte[0];
        match state {
            State::Default => {
                match b {
                    IAC => {},
                    DONT | DO | WONT | WILL | SB => {state = State::Negotiate;},
                    GA => {},
                    EL => {},
                    EC => {},
                    AYT => {},
                    AO => {},
                    IP => {},
                    BREAK => {},
                    DM => {},
                    NOP => {},
                    SE => {
                        state = State::Default;
                    },
                    EOR => {},
                    ABORT => {state = State::Default;},
                    SUSP => {state = State::Default;},
                    xEOF => {},
                    _ => {
                        println!("Not expected: {}", b);
                        let _ = tx.send(b).unwrap();
                    },
                }
            },
            State::TTY => {
                match b {
                    TELOPT_BINARY => {
                        let mut vec = Vec::new();
                        for _byte in stream.bytes() {
                            let _b = _byte.unwrap();
                            if _b == IAC {break;}
                            else {
                                vec.push(_b);
                            }
                        }
                        let ttype = String::from_utf8(vec).unwrap();
                        println!("term type: {}", ttype);
                        state = State::Default;
                    },
                    IAC => {state = State::Default},
                    _ => {
                        println!("Not expected: {}", b);
                        let _ = tx.send(b).unwrap();
                    },
                }
            },
            State::Negotiate => {
                match b {
                    IAC => {state = State::Default;},
                    TELOPT_ECHO => {},
                    TELOPT_RCP => {},
                    TELOPT_SGA => {},
                    TELOPT_TTYPE => {state = State::TTY},
                    TELOPT_NAOL => {},
                    TELOPT_NAOP => {},
                    TELOPT_NAOCRD => {},
                    TELOPT_NAHTS => {},
                    TELOPT_NAOHTD => {},
                    TELOPT_NAOFFD => {},
                    TELOPT_NAOVTS => {},
                    TELOPT_NAOVTD => {},
                    TELOPT_NAOLFD => {},

                    TELOPT_XASCII => {},
                    TELOPT_LOGOUT => {},
                    TELOPT_BM => {},
                    TELOPT_DET => {},
                    TELOPT_SUPDUP => {},
                    TELOPT_NAWS => {
                        let mut peek: [u8; 1] = [0];
                        // since peek is not yet implemented in mio
                        let _ = stream.read_exact(&mut peek);
                        if peek[0] != IAC {
                            let mut col_bytes: [u8; 2] = [0, 0];
                            let mut line_bytes: [u8; 2] = [0, 0];
                            col_bytes[0] = peek[0];
                            let _ = stream.read_exact(&mut peek);
                            col_bytes[1] = peek[0];
                            let _ = stream.read_exact(&mut line_bytes);
                            println!("{:?}, {:?}", col_bytes, line_bytes);
                            let cols: u16 = u8_to_u16!(col_bytes);
                            let lines: u16 = u8_to_u16!(line_bytes);
                            println!("cols {}, lines {}", cols, lines);
                        }
                        state = State::Default;
                    }
                    _ => {
                        println!("Not expected: {}", b);
                        let _ = tx.send(b).unwrap();
                    }
                }
            },
        }
        println!("{}", b);
    }
    tx.send(IAC);
    println!("end");
}
