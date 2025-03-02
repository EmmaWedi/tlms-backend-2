use actix_web::web;

use crate::{
    app::organization::controllers::controller::{add_organization, get_all, upload_img},
    middlewares::auth::JwtAuthMiddleware,
    AppState,
};

pub fn all_routes(cfg: &mut web::ServiceConfig, state: web::Data<AppState>) {
    cfg.service(
        web::scope("/api/v1/organization")
            .route("/add", web::post().to(add_organization))
            .route("/get", web::get().to(get_all).wrap(JwtAuthMiddleware))
            .route("/upload", web::post().to(upload_img).wrap(JwtAuthMiddleware)),
    );
}
