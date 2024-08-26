use actix_web::web;

mod user;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").configure(user::init));
}
