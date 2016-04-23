//#![feature(plugin)]
//#![plugin(clippy)]
#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]

extern crate clap;
extern crate colored;
extern crate fern;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate mio;
extern crate rustc_serialize;
extern crate time;
extern crate toml_config;

#[cfg(test)]
extern crate tempfile;

mod config;
mod dhcp;
mod logger;

use clap::{Arg,App};
use dhcp::*;
use dhcp::listener::*;
use std::env::{var_os};

lazy_static! {
    // we test if it is set AND different to 0
    static ref IS_DEBUG : bool = Some("0".into()) != var_os("DHCPR_DEBUG").or(Some("0".into()));
}


/*
 TODO:
 -

 IDEAS:
 - BOOTP relay mode
 - pid file: option to enable by providing the location to write the pid file
 - OPTIONAL persistant storage with adapters:
    - Sqlite4 K/V
    - Redis
    - Etcd / Consul
*/


fn main() {
    let matches = App::new("dhcpr")
        .version("v1.0-alpha")
        .author("Thomas \"mackwic\" Wickham <mackwic@gmail.com>")
        .about("dhcpr is a modern DHCP server for windows and linux, supporting both IPv4 and IPv6. \n\
                It is designed to de dynamically configured via an HTTP interface.")
        .arg(Arg::with_name("is-debug")
            .long("debug")
            .short("d")
            .help("Debug log output. Can also be obtained with DHCRP_DEBUG=1 in environment."))
        .arg(Arg::with_name("config-file")
            .long("config")
            .short("c")
            .takes_value(true)
            .value_name("TOML_FILE")
            .help("Set a base config file. Must be a valid TOML file."))
        .after_help("This binary is alpha stage. Do not use in production. Report any bug to \
        http://github.com/mackwic/dhcpr.")
        .get_matches();

    logger::init_logger(*IS_DEBUG || matches.is_present("is-debug"));
    let config = config::init_config(matches.value_of("config-file"));

    let sock = mio::udp::UdpSocket::v4().unwrap();
    let mut s = dhcp::listener4::Server::new(config, sock.try_clone().unwrap()).unwrap();
    let mut eloop = mio::EventLoop::new().unwrap();
    eloop.register(&sock, mio::Token(0), mio::EventSet::all(), mio::PollOpt::edge());
    info!("running dhcp server");
    eloop.run(&mut *s);
}
