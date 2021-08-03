use actix_web::{get, web, Result};

#[get("users/{user_id}/{friend}")]
pub async fn path_extract(
    // you can parse instantly from the param
    web::Path((user_id, friend)): web::Path<(u32, String)>,
) -> Result<String> {
    Ok(format!("Welcome {}, user_id {}!\n", friend, &user_id))
}
