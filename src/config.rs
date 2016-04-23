
use std::default::Default;
use std::env::*;
use std::str::FromStr;
use std::path::Path;
use toml_config::ConfigFactory;

#[derive(RustcDecodable,RustcEncodable)]
pub struct Config {
    pub listen4_port: u16,
    toto: String,
}

type ipaddr_bin = [u8; 4];

#[derive(Debug,PartialEq,Eq)]
pub struct NetworkOptions {
    // A network config may depend on a root
    root: Option<Box<NetworkOptions>>,

    subnet_mask: ipaddr_bin,
    lease_time: u32,
    gateway: Option<ipaddr_bin>,
    timeserver: Option<ipaddr_bin>,
    // FIXME multiple DNS
    dns: Option<ipaddr_bin>,
    //hostname: Option<String>,
    domain: Option<String>,
    max_lease_time: u32,
}

macro_rules! env_override_item {
    ($env_str:expr => $conf_item:expr , u16) => {{
        let env_varname = format!("DHCPR_{}", $env_str);
        if let Ok(val) = var(env_varname) {
            if let Ok(val) = <u16 as FromStr>::from_str(&val) {
                debug!("Environment override: {}={}", stringify!($conf_item), val);
                $conf_item = val
            }
        }
    }}
}

impl Config {
    fn env_override(&mut self) {
        env_override_item!("LISTEN4_PORT" => self.listen4_port, u16)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            listen4_port: 64,
            toto: "toto".into()
        }
    }
}

impl Default for NetworkOptions {
    fn default() -> Self {
        NetworkOptions {
            root: None,
            subnet_mask: [255,255,255,0],
            lease_time: 86400, // 24 hours
            max_lease_time: 86400,
            gateway: None,
            timeserver: None,
            dns: None,
            domain: None
        }
    }
}

pub fn init_config(file: Option<&str>) -> Config {
    let mut config = if let Some(path) = file {
        ConfigFactory::load(Path::new(path))
    } else {
        Config::default()
    };

    config.env_override();
    info!("Configuration loaded");
    config
}
