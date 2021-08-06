use actix_web::web;

use super::handlers::{
    get_item, get_item_json, get_item_qstring, get_user, get_user_info, post_user_formdata,
    post_user_json,
};

pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(get_user)
            .service(get_user_info)
            .service(post_user_json)
            .service(post_user_formdata),
    );
}

pub fn item_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/item")
            .service(get_item)
            .route("/json", web::post().to(get_item_json))
            .route("/query", web::get().to(get_item_qstring)),
    );
}
