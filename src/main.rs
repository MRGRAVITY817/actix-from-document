mod app_config;
mod handlers;
mod security;
mod states;

use std::sync::Mutex;

use actix_web::{guard, web, App, HttpResponse, HttpServer};
use handlers::{echo, hello, manual_hello};

use crate::{
    app_config::{config, scoped_config},
    security::ssl_builder,
    states::{echo_counts, hello_name, AppState, AppStateWithCounter},
};

const PORT: &'static str = "4000";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });
    let builder = ssl_builder();
    HttpServer::new(move || {
        App::new()
            .configure(config)
            .service(web::scope("/api").configure(scoped_config))
            .data(AppState {
                app_name: String::from("Actix-web"),
            })
            .app_data(counter.clone())
            .service(
                web::scope("/")
                    // Guard will filter accordingly to predicate
                    .guard(guard::Header("Host", "users.rust-lang.org"))
                    .route("", web::to(|| HttpResponse::Ok().body("user"))),
            )
            .service(hello_name)
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/count", web::get().to(echo_counts))
    })
    .bind_openssl(format!("127.0.0.1:{}", PORT), builder)?
    .run()
    .await
}
