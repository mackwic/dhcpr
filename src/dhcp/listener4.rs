
use mio;
use mio::udp;

use std::io::Result;
use std::net::*;
use std::str::FromStr;

use super::listener::Listener;
use super::messages;

const _SERVER_4: mio::Token = mio::Token(0);

#[derive(Debug)]
pub struct Server {
    sock: Box<udp::UdpSocket>,
}

impl Listener for Server {
    fn new(sock: udp::UdpSocket) -> Result<Box<Self>> {
        // FIXME: configurable listening port + handle permission denied if port < 1024
        try!(sock.bind(&SocketAddr::from_str("0.0.0.0:6767").unwrap()));
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
