use std::sync::Mutex;

use actix_web::{get, web};
pub struct AppState {
    pub app_name: String,
}

#[get("/name")]
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
