#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]

extern crate mio;

use mio::*;
use std::io;

pub trait HandlerMock {
    fn new() -> Self;
}
pub struct DummyHandler;
impl HandlerMock for DummyHandler {
    fn new() -> Self { DummyHandler {} }
}

pub trait TokenMock {
}
pub struct DummyToken;
impl TokenMock for DummyToken {}

pub trait EventLoopMock : Sized {
    fn new<'a>() -> io::Result<Self>;
    fn register<H: HandlerMock, T: TokenMock>(&self, handler: &H, token: T);
    fn run<H: HandlerMock>(&mut self, handlr: &mut H) -> io::Result<()>;
}
#[derive(Copy,Clone)]
pub struct DummyLoop;
impl EventLoopMock for DummyLoop {
    fn new<'a>() -> io::Result<Self> { Ok(DummyLoop{}) }
    fn register<H, T>(&self, _handler: &H, _token: T)
    where H: HandlerMock, T: TokenMock {

    }
    fn run<H: HandlerMock>(&mut self, _handler: &mut H) -> io::Result<()> { Ok(()) }
}




#[cfg(test)]
mod tests {
    pub use super::*;
    describe! EventLoop {
        it "should mimick the EventLoop interface" {
            let mut e = DummyLoop::new().unwrap();
            let mut handler_mock = DummyHandler::new();
            let tok = DummyToken{};
            e.register(&handler_mock, tok);
            e.run(&mut handler_mock);
            assert!(true, "should be always ok when it compiles");
        }
    }
}
