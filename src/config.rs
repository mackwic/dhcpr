
use std::default::Default;
use std::path::Path;
use toml_config::ConfigFactory;

#[derive(RustcDecodable,RustcEncodable)]
pub struct Config {
    toto: String
}

impl Default for Config {
    fn default() -> Self {
        Config {
            toto: "toto".into()
        }
    }
}

pub fn init_config(file: Option<&str>) -> Config {
    if let Some(path) = file {
        ConfigFactory::load(Path::new(path))
    } else {
        Config::default()
    }
}
