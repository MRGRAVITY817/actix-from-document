use actix_web::{get, web, Result};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Info {
    user_id: u32,
    peer: String,
}

impl Info {
    pub fn get_user_info(&self) -> (&u32, &str) {
        (&self.user_id, &self.peer)
    }
}

#[get("users/{user_id}/{friend}")]
pub async fn path_extract(
    // you can parse instantly from the param
    web::Path((user_id, friend)): web::Path<(u32, String)>,
) -> Result<String> {
    Ok(format!("Welcome {}, user_id {}!\n", friend, &user_id))
}

#[get("users/{user_id}/{peer}")]
pub async fn path_struct_extract(info: web::Path<Info>) -> Result<String> {
    let (user_id, peer) = info.get_user_info();
    Ok(format!("Hello {}, user_id {}!", peer, user_id))
}
