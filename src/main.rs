mod api;

use std::path::Path;

use api::config::PHP_API;
use config::Config;

mod config;
fn main() {
    let path = Path::new("test.toml");

    let test = api::php::PhpVersion::get_versions().unwrap();

    for val in test {
        println!("{}:{:?}", val.version, val.path)
    }

    let config = Config::get_config();
    let _ = config.save_config(path);
}
