mod either;
mod errors;
mod extractors;
mod post;
mod security;
mod states;
mod stream;

use actix_web::{guard, web, App, HttpResponse, HttpServer};

const PORT: &'static str = "4000";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let builder = ssl_builder();
    HttpServer::new(move || {
        App::new()
            .configure(post::configs::post_config)
            .configure(states::configs::appstate_config)
            .configure(states::configs::req_count_config)
            .configure(either::configs::config)
            .configure(extractors::configs::item_config)
            .configure(extractors::configs::user_config)
            .configure(post::configs::basic_config)
            .configure(stream::configs::config)
            .configure(errors::configs::config)
            .service(
                web::scope("/")
                    // Guard will filter accordingly to predicate
                    .guard(guard::Header("Host", "users.rust-lang.org"))
                    .route("", web::to(|| HttpResponse::Ok().body("user"))),
            )
    })
    .bind(format!("127.0.0.1:{}", PORT))?
    .run()
    .await
}
