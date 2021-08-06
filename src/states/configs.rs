use std::{
    cell::Cell,
    sync::{atomic::AtomicUsize, Arc, Mutex},
};

use actix_web::web;

use super::handlers::{
    add_req, echo_counts, hello_name, show_reqs, AppState, AppStateWithCounter, RequestCountState,
};

pub fn appstate_config(cfg: &mut web::ServiceConfig) {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });
    cfg.service(
        web::scope("/states")
            .data(AppState {
                app_name: String::from("Actix-web"),
            })
            .route("/name", web::get().to(hello_name))
            .data(counter.clone())
            .service(web::resource("/counts").route(web::get().to(echo_counts))),
    );
}

pub fn req_count_config(cfg: &mut web::ServiceConfig) {
    let req_counter = RequestCountState::new(Cell::new(0), Arc::new(AtomicUsize::new(0)));
    cfg.service(
        web::scope("/req")
            .data(req_counter.clone())
            .service(show_reqs)
            .service(add_req),
    );
}
