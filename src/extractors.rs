use actix_web::{get, web, HttpRequest, Responder, Result};
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

#[get("/users/{user_id}/{friend}")]
pub async fn path_extract(
    // you can parse instantly from the param
    web::Path((user_id, friend)): web::Path<(u32, String)>,
) -> Result<String> {
    Ok(format!("Welcome {}, user_id {}!\n", friend, &user_id))
}

#[get("/peers/{user_id}/{peer}")]
pub async fn path_struct_extract(info: web::Path<Info>) -> Result<String> {
    let (user_id, peer) = info.get_user_info();
    Ok(format!("Hello peer {}, user_id {}!", peer, user_id))
}

#[get("/items/{tier}/{weapon}")]
pub async fn path_weapon(req: HttpRequest) -> Result<String> {
    let name: String = req.match_info().get("weapon").unwrap().parse().unwrap();
    let tier: i32 = req.match_info().query("tier").parse().unwrap();
    Ok(format!("The weapon {}, tier {}.", name, tier))
}

#[get("/peers")]
pub async fn welcome_peer(info: web::Query<Info>) -> String {
    let (_, peer) = info.get_user_info();
    format!("Welcome Peer {}", peer)
}

#[get("/peers/json")]
pub async fn welcome_peer_json(info: web::Json<Info>) -> Result<String> {
    let (_, peer) = info.get_user_info();
    Ok(format!("Welcome {}!", peer))
}

#[derive(Deserialize)]
pub struct UserInfo {
    username: String,
}

impl UserInfo {
    pub fn get_username(&self) -> &str {
        &self.username
    }
}

pub async fn get_username(info: web::Json<UserInfo>) -> impl Responder {
    let username = info.get_username();
    format!("Welcome username: {}\n", username)
}
