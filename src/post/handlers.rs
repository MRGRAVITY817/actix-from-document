use std::future::{ready, Ready};

use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!\n")
}

pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(format!("{}\n", req_body))
}

#[derive(Serialize)]
pub struct Post {
    title: String,
    content: String,
}

impl Responder for Post {
    type Error = actix_web::Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        // Create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

pub async fn read_post() -> impl Responder {
    // Since we implied the Responder trait to Post, we can return Post as a Responder
    Post {
        title: "Hello world".to_owned(),
        content: "Good morning, good afternoon, good evening and good night!".to_owned(),
    }
}
