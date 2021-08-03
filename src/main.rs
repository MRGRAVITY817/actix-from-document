mod handlers;
mod states;

use std::sync::Mutex;

use actix_web::{web, App, HttpServer};
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
