use actix_web::{guard, web, HttpResponse};

pub fn configs(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/guards")
            .route(
                "/not",
                web::route()
                    .guard(guard::Not(guard::Get()))
                    .to(|| HttpResponse::MethodNotAllowed()),
            )
            .route(
                "/any",
                web::route()
                    .guard(guard::Any(guard::Get()).or(guard::Post()))
                    .to(|| HttpResponse::MethodNotAllowed()),
            )
            .route(
                "/all",
                web::route()
                    .guard(
                        guard::All(guard::Get()).and(guard::Header("content-type", "plain/text")),
                    )
                    .to(|| HttpResponse::MethodNotAllowed()),
            ),
    );
}
