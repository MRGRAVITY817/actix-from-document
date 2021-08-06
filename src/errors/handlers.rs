use actix_web::{
    dev::HttpResponseBuilder,
    error, get,
    http::{header, StatusCode},
    HttpRequest, HttpResponse, Result,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
#[display(fmt = "my error: {}", name)]
pub struct MyError {
    name: &'static str,
}

impl error::ResponseError for MyError {}

#[get("/test")]
pub async fn test_myerror() -> Result<&'static str, MyError> {
    Err(MyError { name: "test" })
}

#[derive(Debug, Display, Error)]
pub enum MoreError {
    #[display(fmt = "internal error")]
    InternalError,
    #[display(fmt = "bad request")]
    BadRequest,
    #[display(fmt = "timeout")]
    Timeout,
}

impl error::ResponseError for MoreError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            MoreError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MoreError::BadRequest => StatusCode::BAD_REQUEST,
            MoreError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}

#[get("/more/{error_type}")]
pub async fn more_error(req: HttpRequest) -> Result<&'static str, MoreError> {
    let error_type: i32 = req.match_info().query("error_type").parse().unwrap();
    match &error_type {
        1 => Err(MoreError::InternalError),
        2 => Err(MoreError::BadRequest),
        _ => Err(MoreError::Timeout),
    }
}
