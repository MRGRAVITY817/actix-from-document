use actix_web::{get, post, web, HttpRequest, Responder, Result};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserInfo {
    id: u32,
    user: String,
}

impl UserInfo {
    pub fn get_user_info(&self) -> (&u32, &str) {
        (&self.id, &self.user)
    }
}

#[get("/{id}/{user}")]
pub async fn get_user(
    // you can parse instantly from the param
    web::Path((id, user)): web::Path<(u32, String)>,
) -> Result<String> {
    Ok(format!("Hello {}, id {}!", user, id))
}

#[get("/{id}/{user}")]
pub async fn get_user_info(info: web::Path<UserInfo>) -> Result<String> {
    let (id, user) = info.get_user_info();
    Ok(format!("Hello {}, id {}!", user, id))
}

#[post("/json")]
pub async fn post_user_json(info: web::Json<UserInfo>) -> impl Responder {
    let (id, user) = info.get_user_info();
    format!("Hello {}, id {}!", user, id)
}

#[post("/form")]
pub async fn post_user_formdata(form: web::Form<UserInfo>) -> Result<String> {
    let (id, user) = form.get_user_info();
    Ok(format!("Hello {}, id {}!", user, id))
}

#[derive(Deserialize)]
pub struct UserItem {
    tier: u32,
    name: String,
}

impl UserItem {
    pub fn get_item_info(&self) -> (&u32, &str) {
        (&self.tier, &self.name)
    }
}

#[get("/{tier}/{name}")]
pub async fn get_item(req: HttpRequest) -> Result<String> {
    // Basically get("key").unwrap() === query("key")
    let name: String = req.match_info().get("name").unwrap().parse().unwrap();
    let tier: i32 = req.match_info().query("tier").parse().unwrap();
    Ok(format!("The weapon {} has tier {}.", name, tier))
}

pub async fn get_item_qstring(info: web::Query<UserItem>) -> String {
    // /items?tier=1&name=excalibur
    let (tier, name) = info.get_item_info();
    format!("Item info: {}, Tier {}", name, tier)
}

pub async fn get_item_json(info: web::Json<UserItem>) -> Result<String> {
    let (tier, name) = info.get_item_info();
    Ok(format!("Item info: {}, Tier {}", name, tier))
}
