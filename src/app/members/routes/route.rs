use actix_web::web;

use crate::{
    app::members::controllers::controller::{add_member, get_all},
    middlewares::auth::JwtAuthMiddleware,
    AppState,
};

pub fn all_routes(cfg: &mut web::ServiceConfig, state: web::Data<AppState>) {
    cfg.service(
        web::scope("/api/v1/members")
            .route("/add", web::post().to(add_member).wrap(JwtAuthMiddleware))
            .route("/get", web::get().to(get_all).wrap(JwtAuthMiddleware)),
    );
}
