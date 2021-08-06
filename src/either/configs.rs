use actix_web::web::{self, post};

use super::handlers::which_either;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/either").route("/lang", post().to(which_either)));
}
