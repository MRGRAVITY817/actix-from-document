use actix_web::{web, HttpResponse};

pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| HttpResponse::Ok().body("test\n")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| HttpResponse::Ok().body("app\n")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}
