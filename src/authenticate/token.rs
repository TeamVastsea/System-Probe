use base64::Engine;
use jwt_simple::prelude::{HS256Key, MACLike};
use simple_log::debug;
use simple_log::log::error;
use crate::authenticate::black::{is_in_blacklist, record};
use crate::authenticate::UserInfo;
use crate::CONFIG;

impl UserInfo {
    pub async fn from_token(token: String) -> Result<UserInfo, String> {
        let key = match &CONFIG.authenticate.secret {
            None => { return Err("Key is not defined.".to_string())}
            Some(a) => {a}
        };
        let key = HS256Key::from_bytes(base64::engine::general_purpose::STANDARD.decode(key).unwrap().as_slice());

        let claims = key.verify_token::<UserInfo>(&token, None);
        match claims {
            Ok(info) => Ok(info.custom),
            Err(err) => {

                error!("{}", err.to_string());
                Err("Cannot parse token or token is valid".to_string())
            }
        }
    }
}

pub async fn authenticate_token(token: String, ip: String) -> Result<(), String> {
    if is_in_blacklist(ip.clone()).await {
        debug!("{} is in black list.", ip);
        return Err("In blacklist.".to_string());
    }

    let info = match UserInfo::from_token(token).await {
        Ok(a) => {a}
        Err(err) => {
            record(ip.clone()).await;
            return Err(err);
        }
    };

    let group = info.group;
    for permission in group {
        if permission == "admin" {
            return Ok(());
        }
    }

    record(ip).await;
    Err("No permission".to_string())
}