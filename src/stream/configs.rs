use actix_web::web;

use super::handlers::stream_test;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(stream_test);
}
