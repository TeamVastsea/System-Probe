pub mod fire;

use std::net::TcpStream;
use std::sync::{Mutex};
use tungstenite::WebSocket;

pub static CLIENTS: Mutex<Vec<Option<(String, WebSocket<TcpStream>)>>> = Mutex::new(Vec::new());