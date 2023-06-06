pub mod token;
pub mod black;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserInfo {//_id: mc uuid
    pub _id: String,
    pub display_name: String,
    pub enabled: bool,
    pub group: Vec<String>,
    pub bind_qq: Option<i64>,
    pub ban_reason: Option<String>
}
