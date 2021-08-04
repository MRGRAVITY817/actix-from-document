mod app_config;
mod either;
mod extractors;
mod handlers;
mod security;
mod states;
mod stream;

use std::sync::Mutex;

use actix_web::{guard, web, App, HttpResponse, HttpServer};
use handlers::{echo, hello, manual_hello};

use crate::{
    app_config::{config, scoped_config},
    either::which_either,
    extractors::{path_extract, path_struct_extract},
    handlers::read_post,
    security::ssl_builder,
    states::{echo_counts, hello_name, AppState, AppStateWithCounter},
    stream::stream_test,
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
            .service(stream_test)
            .service(which_either)
            .service(path_extract)
            .service(path_struct_extract)
            .route("/hey", web::get().to(manual_hello))
            .route("/count", web::get().to(echo_counts))
            .route("/post", web::get().to(read_post))
    })
    .bind_openssl(format!("127.0.0.1:{}", PORT), builder)?
    .run()
    .await
}
