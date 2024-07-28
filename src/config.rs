use std::{
    fs::{self, File},
    io::{self, BufRead, Write},
    path::Path,
};

use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Php {
    pub port: u16,
    pub enabled: bool,
}
#[derive(Deserialize, Serialize)]
pub struct Phpmyadmin {
    pub port: u16,
    pub enabled: bool,
}
#[derive(Deserialize, Serialize)]
pub struct Mariadb {
    pub port: u16,
    pub enabled: bool,
}
#[derive(Deserialize, Serialize)]
pub struct Config {
    pub php: Php,
    pub phpmyadmin: Phpmyadmin,
    pub mariadb: Mariadb,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            php: Php {
                port: 8080,
                enabled: true,
            },
            phpmyadmin: Phpmyadmin {
                port: 8080,
                enabled: true,
            },
            mariadb: Mariadb {
                port: 3306,
                enabled: true,
            },
        }
    }
}

impl Config {
    pub fn get_config() -> Config {
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        let mut config = Config::default();

        fn read_input(handle: &mut dyn BufRead, prompt: &str, default: &str) -> String {
            let mut buffer = String::new();
            loop {
                print!("{}: [{}] ", prompt.bold(), default.green());
                io::stdout().flush().unwrap();
                buffer.clear();
                handle.read_line(&mut buffer).unwrap();
                let input = buffer.trim();
                if input.is_empty() {
                    return default.to_string();
                } else {
                    return input.to_string();
                }
            }
        }

        fn read_port(handle: &mut dyn BufRead, prompt: &str, default: u16) -> u16 {
            loop {
                let input = read_input(handle, prompt, &default.to_string());
                match input.parse::<u16>() {
                    Ok(port) => return port,
                    Err(_) => println!("Invalid port number. Please enter a valid number."),
                }
            }
        }

        fn read_bool(handle: &mut dyn BufRead, prompt: &str, default: bool) -> bool {
            loop {
                let input = read_input(handle, prompt, if default { "YES" } else { "NO" });
                match input.to_lowercase().as_str() {
                    "yes" => return true,
                    "no" => return false,
                    _ => println!("Invalid input. Please enter YES or NO."),
                }
            }
        }

        config.php.port = read_port(&mut handle, "PHP port", config.php.port);

        config.phpmyadmin.enabled =
            read_bool(&mut handle, "Use phpMyAdmin?", config.phpmyadmin.enabled);
        if config.phpmyadmin.enabled {
            config.phpmyadmin.port =
                read_port(&mut handle, "phpMyAdmin port", config.phpmyadmin.port);
        }

        config.mariadb.enabled = read_bool(&mut handle, "Use MariaDB?", config.mariadb.enabled);
        if config.mariadb.enabled {
            config.mariadb.port = read_port(&mut handle, "MariaDB port", config.mariadb.port);
        }
        config
    }

    pub fn save_config(&self, path: &Path) -> anyhow::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(toml::to_string(&self)?.as_bytes())?;
        Ok(())
    }

    pub fn load_config(path: &Path) -> Config {
        toml::from_str(&fs::read_to_string(path).unwrap()).unwrap()
    }
}
