use std::ops::{DerefMut};
use simple_log::{debug, warn};
use tungstenite::{Message};
use crate::broadcast::{CLIENTS};
use crate::monitor::MachineInfo;

pub async fn fire(data: MachineInfo) {
    let data_str = serde_json::to_string(&data).unwrap();

    let mut trys = 0;
    let mut clients = loop {
        if trys > 100 {
            return;
        }
        trys += 1;

        if let Ok(clients) = CLIENTS.try_lock() {
            break clients;
        }
    };

    for client_info in clients.deref_mut() {
        let (ip, client) = client_info.as_mut().unwrap();
        match client.write_message(Message::Text(data_str.clone())) {
            Ok(_) => {}
            Err(_) => {
                warn!("Removing client: {}.", ip);
                *client_info = None;
            }
        };
    }
    if clients.len() > 0 {
        clients.retain(|x| x.is_some());
        debug!("Send to {} clients.", clients.len());
    }
}