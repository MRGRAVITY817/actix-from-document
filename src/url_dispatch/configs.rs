use actix_web::{guard, web, HttpResponse};

use super::handlers::{external_url, resource_url};

pub fn configs(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test/{a}/{b}/{c}")
            .name("foo")
            .guard(guard::Get())
            .to(|| HttpResponse::Ok()),
    )
    .service(resource_url)
    .external_resource("youtube", "https://youtube.com/watch/{video_id}")
    .service(external_url);
}
