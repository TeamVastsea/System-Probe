pub mod refresh;
mod query;

use serde::{Deserialize, Serialize};
use crate::monitor::query::query;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MachineInfo {
    pub free_space: f32,
    pub cpu: f32,
    pub memory: f32,
    pub upload: u64,
    pub download: u64,
    pub boot_time: String,
}

pub fn init() {
    tokio::spawn(query());
}