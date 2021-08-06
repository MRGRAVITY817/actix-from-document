use actix_web::web;

use super::handlers::{echo, hello, read_post};

pub fn basic_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            .route("", web::get().to(hello))
            .route("/echo", web::post().to(echo)),
    );
}

pub fn post_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/post").route("", web::get().to(read_post)));
}
