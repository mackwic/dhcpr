use mio::udp::*;

use std::io::Result;

pub trait Listener {
    fn new(sock: UdpSocket) -> Result<Box<Self>>;
}
