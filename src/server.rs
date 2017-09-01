extern crate mioco;
extern crate net2;

use std::net::{SocketAddr, IpAddr};
use std::io::{self};
use std::time::Duration;
use mioco::tcp::{TcpListener};
use net2::unix::UnixTcpBuilderExt;
use net2::TcpListenerExt;
use bbs;

const KEEPALIVE_TIME: Option<u32> = Some(180);

pub fn run(addr: IpAddr, port: u16) {
    let ip = SocketAddr::new(addr, port);
    //let linger_time = Some(Duration::new(0, 0));
    let builder: net2::TcpBuilder;
    if ip.is_ipv4() {
        builder = net2::TcpBuilder::new_v4().unwrap()
    } else {
        builder = net2::TcpBuilder::new_v6().unwrap()
    }
    let net2_listener = builder.reuse_port(true).unwrap()
                               .bind(&ip).unwrap()
                               .listen(1000).unwrap();
    let _ = net2_listener.set_linger(Some(Duration::new(0, 0)));
    let lst = TcpListener::from_listener(net2_listener, &ip).unwrap();
    mioco::start(move || {
        for _ in 0..mioco::thread_num() {
            let listener = lst.try_clone().unwrap();
            mioco::spawn(move || -> io::Result<()> {
                loop {
                    let conn = listener.accept().unwrap();
                    let _ = conn.set_keepalive(KEEPALIVE_TIME);
                    let _ = conn.set_nodelay(true);
                    //let _ = conn.set_linger(linger_time);
                    println!("receive connection from {}",
                             conn.peer_addr().unwrap());
                    mioco::spawn(move || -> bbs::BBSResult {
                        bbs::run(conn)
                    });
                }
            });
        }
    }).unwrap();
}
