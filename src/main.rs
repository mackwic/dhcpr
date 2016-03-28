#![feature(plugin)]
#![plugin(clippy)]

extern crate clap;
extern crate colored;
extern crate fern;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate rustc_serialize;
extern crate time;
extern crate toml_config;

mod config;
mod logger;

use clap::{Arg,App};
use std::env::{var_os};

lazy_static! {
    // we test if it is set AND different to 0
    static ref IS_DEBUG : bool = Some("0".into()) != var_os("DHCPR_DEBUG").or(Some("0".into()));
}

fn main() {
    let matches = App::new("dhcpr")
        .version("v1.0-alpha")
        .author("Thomas \"mackwic\" Wickham <mackwic@gmail.com>")
        .about("dhcrp is a modern DHCP server for windows and linux, supporting both IPv4 and IPv6. \n\
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
        .after_help("This binary is alpha stage. Do not use in production, and report any but to \
        http://github.com/mackwic/dhcpr.")
        .get_matches();

    logger::init_logger(*IS_DEBUG || matches.is_present("is-debug"));
    let _c = config::init_config(matches.value_of("config-file"));
}
