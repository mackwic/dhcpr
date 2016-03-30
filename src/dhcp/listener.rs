use config;

use mio::udp::*;
use std::io::Result;

pub trait Listener {
    fn new(config: config::Config, sock: UdpSocket) -> Result<Box<Self>>;
}
