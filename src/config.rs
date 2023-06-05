use std::fs::OpenOptions;
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};
use toml::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub connection: ConnectionSetting,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConnectionSetting {
    pub address: String,
    pub port: i32,
}


pub fn get_config() -> Config {
    let mut config: String = Default::default();
    let mut file = OpenOptions::new().read(true).write(true).create(true).open("config.toml").expect("Can not open 'config.toml'");
    file.read_to_string(&mut config).expect("Can not read 'config.toml'");
    if config == "" {
        config = "[mongodb]\n[connect]\n[feature]".to_string();
    }

    let mut config: Value = toml::from_str(config.as_str()).expect("Cannot parse config file, you should probably generate another.");

    let mut edit = false;

    let mut result_config: Config = Config {
        connection: ConnectionSetting {
            address: "".to_string(),
            port: 0,
        },
    };

    result_config.connection = if let Some(a) = config.get("connection") {
        let address = if let Some(b) = a.get("address") {
            if let Some(c) = b.as_str() {
                c.to_string()
            } else {
                "127.0.0.1".to_string()
            }
        } else {
            edit = true;
            "127.0.0.1".to_string()
        };
        let port = if let Some(b) = a.get("address") {
            if let Some(c) = b.as_integer() {
                c as i32
            } else {
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
        ConnectionSetting {
            address: "127.0.0.1".to_string(),
            port: 7890,
        }
    };

    if edit {
        let mut file = OpenOptions::new().write(true).create(true).open("config.toml").expect("Can not open 'config.toml'");
        file.write(toml::to_string(&config).unwrap().as_bytes()).expect("Cannot save config.");
    }

    result_config
}
