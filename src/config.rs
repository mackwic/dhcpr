
use std::default::Default;
use std::env::*;
use std::str::FromStr;
use std::path::Path;
use toml_config::ConfigFactory;

// DHCP servers listen on port 64
const DEFAULT_LISTEN4_PORT : u16 = 64;

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
            listen4_port: DEFAULT_LISTEN4_PORT,
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

#[cfg(test)]
mod tests {

    pub use super::*;
    use std::ffi::OsStr;

    pub fn with_env_var<K: AsRef<OsStr>, V: AsRef<OsStr>, F: FnOnce()>(key: K, val: V, fun: F) {
        use std::env;
        let old_var = env::var(&key);
        env::set_var(&key, &val);

        fun();

        // restore env variable
        match old_var {
            Ok(val) => env::set_var("DHCPR_LISTEN4_PORT", &val),
            Err(_) => env::remove_var("DHCPR_LISTEN4_PORT")
        }
    }

    describe! config {
        it "should have a default impl" {
            let c = Config::default();
            assert_eq!(c.listen4_port, ::config::DEFAULT_LISTEN4_PORT)
        }

        it "can be overriden by env 2" {
            use super::with_env_var;
            let mut c = Config::default();
            let test_value1 = 6464;
            let test_value2 = 6465;

            with_env_var("DHCPR_LISTEN4_PORT", test_value1.to_string(), || {
                c.env_override();
                assert_eq!(test_value1, c.listen4_port);
            });

            with_env_var("DHCPR_LISTEN4_PORT", test_value2.to_string(), || {
                c.env_override();
                assert_eq!(test_value2, c.listen4_port);
            });

            with_env_var("DHCPR_LISTEN4_PORT", test_value1.to_string(), || {
                c.env_override();
                assert_eq!(test_value1, c.listen4_port);
            });
        }

        it "env override the file init" {
            extern crate tempfile;
            use std::io::Write;

            let reason = "unable to create a temporary file for the test";
            let file_value = 7272;
            let env_value = 2727;

            let mut file = tempfile::NamedTempFile::new().expect(reason);
            <tempfile::NamedTempFile as Write>::write_all(&mut file, format!(
                "listen4_port = {}", file_value
            ).as_bytes());

            let mut c = init_config(Some(file.path().to_str().unwrap()));

            with_env_var("DHCPR_LISTEN4_PORT", env_value.to_string(), || {
                c.env_override();
                assert_eq!(env_value, c.listen4_port);
            });
        }

    }
}
