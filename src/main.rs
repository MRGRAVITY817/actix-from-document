mod handlers;
mod states;

use std::sync::Mutex;

use actix_web::{guard, web, App, HttpResponse, HttpServer};
use handlers::{echo, hello, manual_hello};

use crate::states::{echo_counts, hello_name, AppState, AppStateWithCounter};

const PORT: &'static str = "4000";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });
    HttpServer::new(move || {
        App::new()
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
    .bind(format!("127.0.0.1:{}", PORT))?
    .run()
    .await
}
