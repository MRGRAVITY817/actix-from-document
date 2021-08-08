mod either;
mod errors;
mod extractors;
mod guards;
mod middlewares;
mod post;
mod security;
mod states;
mod stream;
mod url_dispatch;

use actix_session::CookieSession;
use env_logger::Env;

use actix_web::{
    guard,
    middleware::{self, Logger},
    web, App, HttpResponse, HttpServer,
};

const PORT: &'static str = "4000";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    HttpServer::new(move || {
        App::new()
            // NormalizePath will add / or reduce mulitple /'s in our path
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(middleware::NormalizePath::default())
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .configure(states::configs::appstate_config)
            .configure(states::configs::req_count_config)
            .configure(errors::configs::config)
            .configure(post::configs::post_config)
            .configure(extractors::configs::item_config)
            .configure(extractors::configs::user_config)
            .configure(post::configs::basic_config)
            .configure(stream::configs::config)
            .configure(either::configs::config)
            .configure(url_dispatch::configs::configs)
            .configure(guards::configs::configs)
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
