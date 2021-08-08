use actix_web::{get, http::header, HttpRequest, HttpResponse, Responder, Result};

#[get("/test/")]
pub async fn resource_url(req: HttpRequest) -> Result<HttpResponse> {
    let url = req.url_for("foo", &["1", "2", "3"])?;
    Ok(HttpResponse::Found()
        .header(header::LOCATION, url.as_str())
        .finish())
}

#[get("/external")]
pub async fn external_url(req: HttpRequest) -> impl Responder {
    let url = req.url_for("youtube", &["oHg5JSyRHA0"]).unwrap();
    assert_eq!(url.as_str(), "https://youtube.com/watch/oHg5JSyRHA0");
    url.to_string()
}
