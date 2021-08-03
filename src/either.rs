use actix_web::{post, Either, Error, HttpResponse};

type RegisterResult = Either<HttpResponse, Result<&'static str, Error>>;

#[post("/lang")]
pub async fn which_either(req_body: String) -> RegisterResult {
    let req = &req_body;
    if req.starts_with("Java") {
        Either::A(HttpResponse::BadRequest().body("Java and javascript is terrible language"))
    } else {
        Either::B(Ok("Hello other languages!"))
    }
}
