mod config;
mod monitor;
mod broadcast;
mod authenticate;

use std::borrow::Cow;
use std::net::{TcpListener, TcpStream};
use chrono::Local;
use lazy_static::lazy_static;
use simple_log::{info, LogConfigBuilder, warn};
use tungstenite::{accept};
use tungstenite::protocol::CloseFrame;
use tungstenite::protocol::frame::coding::CloseCode;
use crate::authenticate::token::authenticate_token;
use crate::broadcast::CLIENTS;

use crate::config::{Config, get_config};


lazy_static! {
    static ref CONFIG: Config = get_config();
}

#[tokio::main]
async fn main() {
    let mut file_name = "./log/".to_owned();
    file_name += &Local::now().format("%Y-%m-%d.%H-%M-%S").to_string();
    file_name += ".log";

    let config = LogConfigBuilder::builder()
        .path(&file_name)
        .size(1 * 100)
        .roll_count(10)
        .time_format("%Y-%m-%d %H:%M:%S.%f") //E.g:%H:%M:%S.%f
        .level("debug")
        .output_file()
        .output_console()
        .build();
    simple_log::new(config).expect("Cannot init logger");

    monitor::init();
    authenticate::black::init();
    let server = TcpListener::bind(format!("{}:{}", &CONFIG.connection.address, &CONFIG.connection.port)).unwrap();
    info!("listening at {}:{}.", &CONFIG.connection.address, &CONFIG.connection.port);
    for stream in server.incoming() {
        tokio::spawn(handle_income(stream.unwrap()));
    }
}

async fn handle_income(stream: TcpStream) {
    let addr = &stream.peer_addr().unwrap().ip();
    let mut websocket = accept(stream).unwrap();

    if let Ok(a) = websocket.read_message() {
        info!("Received: {} from {}", a.to_string(), addr.to_string());

        match authenticate_token(a.to_string(), addr.to_string()).await {
            Ok(_) => {
                let mut trys = 0;
                loop {
                    trys += 1;
                    if trys > 100 {
                        websocket.close(Some(CloseFrame {
                            code: CloseCode::Again,
                            reason: Cow::from("Server error"),
                        })).unwrap();
                        warn!("Failed to connect with {}.", addr.to_string());
                        break;
                    }

                    if let Ok(mut clients) = CLIENTS.try_lock() {
                        clients.push(Some((addr.to_string(), websocket)));
                        break;
                    }
                };
            }
            Err(err) => {
                websocket.close(Some(CloseFrame {
                    code: CloseCode::Policy,
                    reason: Cow::from(err),
                })).unwrap();
            }
        }
    }
}