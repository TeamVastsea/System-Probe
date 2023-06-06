use std::fs::OpenOptions;
use std::io::{Read, Write};
use base64::Engine;
use jwt_simple::prelude::HS256Key;
use serde::{Deserialize, Serialize};
use simple_log::error;
use toml::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub connection: ConnectionSetting,
    pub authenticate: AuthenticateSetting,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConnectionSetting {
    pub address: String,
    pub port: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthenticateSetting {
    pub method: String,
    pub secret: Option<String>,
    pub db_url: Option<String>,
    pub db_name: Option<String>,
}


pub fn get_config() -> Config {
    let mut config: String = Default::default();
    let mut file = OpenOptions::new().read(true).write(true).create(true).open("config.toml").expect("Can not open 'config.toml'");
    file.read_to_string(&mut config).expect("Can not read 'config.toml'");

    let config: Value = toml::from_str(config.as_str()).expect("Cannot parse config file, you should probably generate another.");

    let mut edit = false;

    let mut result_config: Config = Config {
        connection: ConnectionSetting {
            address: "".to_string(),
            port: 0,
        },
        authenticate: AuthenticateSetting {
            method: "".to_string(),
            secret: None,
            db_url: None,
            db_name: None,
        },
    };

    result_config.connection = if let Some(a) = config.get("connection") {
        let address = if let Some(b) = a.get("address") {
            if let Some(c) = b.as_str() {
                c.to_string()
            } else {
                edit = true;
                "127.0.0.1".to_string()
            }
        } else {
            edit = true;
            "127.0.0.1".to_string()
        };




        let port = if let Some(b) = a.get("port") {
            if let Some(c) = b.as_integer() {
                c as i32
            } else {
                edit = true;
                7890
            }
        } else {
            edit = true;
            7890
        };


        ConnectionSetting {
            address,
            port,
        }
    } else {
        edit = true;
        ConnectionSetting {
            address: "127.0.0.1".to_string(),
            port: 7890,
        }
    };


    result_config.authenticate = if let Some(a) = config.get("authenticate") {
        let method = if let Some(b) = a.get("method") {
            if let Some(c) = b.as_str() {
                if c == "token" || c == "key" {
                    c.to_string()
                } else {
                    edit = true;
                    "token".to_string()
                }
            } else {
                edit = true;
                "token".to_string()
            }
        } else {
            edit = true;
            "token".to_string()
        };

        let secret = if method == "token" {
            let key = if let Some(b) = a.get("secret") {
                if let Some(c) = b.as_str() {
                    c.to_string()
                } else {
                    edit = true;
                    let key = HS256Key::generate();
                    base64::engine::general_purpose::STANDARD.encode(key.to_bytes())
                }
            } else {
                edit = true;
                let key = HS256Key::generate();
                base64::engine::general_purpose::STANDARD.encode(key.to_bytes())
            };

            Some(key)
        } else {
            None
        };

        AuthenticateSetting {
            method,
            secret,
            db_url: None,
            db_name: None,
        }
    } else {
        let key = HS256Key::generate();
        let key = base64::engine::general_purpose::STANDARD.encode(key.to_bytes());
        AuthenticateSetting {
            method: "token".to_string(),
            secret: Some(key),
            db_url: None,
            db_name: None,
        }
    };

    if edit {
        error!("Saving config.");
        let mut file = OpenOptions::new().write(true).create(true).open("config.toml").expect("Can not open 'config.toml'");
        file.write(toml::to_string(&result_config).unwrap().as_bytes()).expect("Cannot save config.");
        panic!("Config changed")
    }

    result_config
}
