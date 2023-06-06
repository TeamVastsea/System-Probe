use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use simple_log::{info, warn};

lazy_static! {
    static ref BLACKLIST: Mutex<HashMap<String, u8>> = Mutex::new(HashMap::new());
}

pub async fn is_in_blacklist(ip: String) -> bool {
    let mut trys = 0;
    loop {
        trys += 1;
        if trys > 100 {
            return true;
        }

        if let Ok(map) = BLACKLIST.try_lock() {
            return match map.get(ip.as_str()) {
                None => {
                    false
                }
                Some(a) => {
                    if *a > 50 {
                        true
                    } else {
                        false
                    }
                }
            }
        }
    }
}

pub async fn clear_blacklist() {
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1200)).await;

        let mut trys = 0;
        loop {
            trys += 1;
            if trys > 100 {
                break;
            }

            if let Ok(mut map) = BLACKLIST.try_lock() {
                if map.len() > 0 {
                    map.clear();
                    info!("Clearing black list.");
                }
                break;
            }
        }

    }
}

pub async fn record(ip: String) {
    warn!("Recording {} into blacklist.", ip.as_str());
    let mut trys = 0;
    loop {
        trys += 1;
        if trys > 100 {
            return;
        }

        if let Ok(mut map) = BLACKLIST.try_lock() {
            let num = match map.get(ip.as_str()) {
                None => {
                    0
                }
                Some(a) => {
                    *a
                }
            };
            map.insert(ip.clone(), num + 1);
            return;
        }
    }
}

pub fn init() {
    tokio::spawn(clear_blacklist());
}