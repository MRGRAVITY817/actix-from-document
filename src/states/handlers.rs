use std::{
    cell::Cell,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
};

use actix_web::{get, web, Responder};
pub struct AppState {
    pub app_name: String,
}

pub async fn hello_name(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {}!\n", app_name)
}

pub struct AppStateWithCounter {
    pub counter: Mutex<i32>, // mutex is necessary because we don't want other process to access this
}

pub async fn echo_counts(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Request number: {}\n", counter)
}

#[derive(Clone)]
pub struct RequestCountState {
    local_count: Cell<usize>,
    global_count: Arc<AtomicUsize>,
}

impl RequestCountState {
    pub fn new(local_count: Cell<usize>, global_count: Arc<AtomicUsize>) -> Self {
        RequestCountState {
            local_count,
            global_count,
        }
    }
}

#[get("/")]
pub async fn show_reqs(data: web::Data<RequestCountState>) -> impl Responder {
    format!(
        "Global count: {}\nLocal count: {}\n",
        data.global_count.load(Ordering::Relaxed),
        data.local_count.get()
    )
}

#[get("/add")]
pub async fn add_req(data: web::Data<RequestCountState>) -> impl Responder {
    // You cannot set cell and arc values in normal way.
    // "fetch_add(value, order)" is used for adding atomic int value.
    data.global_count.fetch_add(1, Ordering::Relaxed);
    let local_count = data.local_count.get();
    // "set(val)" is used for setting the cell value.
    data.local_count.set(local_count + 1);

    format!(
        "Global count: {}\nLocal count: {}\n",
        data.global_count.load(Ordering::Relaxed),
        data.local_count.get()
    )
}
