use actix_web::web;

use super::handlers::{more_error, test_myerror};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/error")
            .service(test_myerror)
            .service(more_error),
    );
}

pub fn hello_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/hello")
            .service(test_myerror)
            .service(more_error),
    );
}
