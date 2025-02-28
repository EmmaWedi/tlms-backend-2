use actix_web::{web, HttpResponse};

use crate::{
    libs::{error, pword::parse_uuid},
    utils::{
        file_methods::{file_exists, read_file},
        shared::get_media_by_id,
    },
    AppState,
};

async fn req_read_file(
    id: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let parsed_id = parse_uuid(&id);

    let media = get_media_by_id(parsed_id, &state).await;

    match media {
        Ok(Some(m)) => {
            let file_name = m.file_name.unwrap();
            let mime_type = m.mime_type.unwrap();
            let extension = mime_type.split('/').nth(1);
            if file_exists(&file_name, &extension.unwrap()).await {
                match read_file(&file_name, &extension.unwrap()).await {
                    Ok(data) => Ok(HttpResponse::Ok().content_type(mime_type).body(data)),
                    Err(_) => Ok(HttpResponse::InternalServerError().body("Error reading file")),
                }
            } else {
                Ok(HttpResponse::NotFound().body("File not found"))
            }
        }
        _ => Ok(HttpResponse::NotFound().body("Media not found")),
    }
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("api/v1/media/{id}").route(web::get().to(req_read_file)));
}
