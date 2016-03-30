
use mio;
use mio::udp;

use std::io::Result;
use std::net::*;
use std::str::FromStr;

use super::listener::Listener;
use super::messages;
use config;

const _SERVER_4: mio::Token = mio::Token(0);

#[derive(Debug)]
pub struct Server {
    sock: Box<udp::UdpSocket>,
}

impl Listener for Server {
    fn new(config: config::Config, sock: udp::UdpSocket) -> Result<Box<Self>> {
        let local_ip4 = IpAddr::from_str("0.0.0.0").unwrap();
        try!(sock.bind(&SocketAddr::new(local_ip4, config.listen4_port)));
        Ok(Box::new(Server { sock: Box::new(sock) }))
    }
}

impl mio::Handler for Server {
    type Timeout = (); // FIXME
    type Message = messages::S2CMessages;

    fn ready(&mut self, _eloop: &mut mio::EventLoop<Server>, _token: mio::Token, events: mio::EventSet) {
        info!("event loop {:?}", events)
    }
}
