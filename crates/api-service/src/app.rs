use crate::routes::user_routes::create_user;
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/users", web::post().to(create_user));
}
