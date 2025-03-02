use actix_web::{web, HttpRequest, HttpResponse};
use base64::{engine::general_purpose, Engine as _};
use serde_json::json;

use crate::{
    app::organization::{
        dto::dtos::{
            get_organization_by_id, get_organization_by_phone, get_organizations, save_organization,
        },
        models::model::{
            AddOrganizationDto, AddOrganizationModel, CreatedResponseModel, UploadImgModel,
        },
    },
    libs::{error, validator},
    utils::{
        file_methods::save_file,
        models::{HttpClientResponse, SaveMediaDto, SaveMemberOrgDto},
        shared::{save_media_meta, save_member_from_org},
    },
    AppState,
};

pub async fn add_organization(
    _req: HttpRequest,
    payload: web::Json<AddOrganizationModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let name = validator::required_str(&payload.name, "Name")?;
    let email = validator::email(&payload.email, "Email")?;
    let mobile = validator::mobile(&payload.phone, "Phone")?;
    let address = validator::required_str(&payload.address, "Address")?;
    let first_name = validator::required_str(&payload.first_name, "First Name")?;
    let last_name = validator::required_str(&payload.last_name, "Last Name")?;
    let member_email = validator::email(
        payload.member_email.as_deref().unwrap_or(""),
        "Member Email",
    )?;
    let member_phone = validator::mobile(&payload.member_phone, "Member Phone")?;
    let member_address = validator::required_str(&payload.member_address, "Member Address")?;
    let gender = validator::required_str(&payload.gender, "Gender")?;
    let date_joined = validator::date(
        payload
            .date_joined
            .map(|d| d.to_string())
            .as_deref()
            .unwrap_or(""),
        "Date Joined",
    )?;
    let date_of_birth = validator::date(
        payload
            .date_of_birth
            .map(|d| d.to_string())
            .as_deref()
            .unwrap_or(""),
        "Date of Birth",
    )?;

    if let Ok(_) = get_organization_by_phone(&mobile, &state).await {
        return Ok(HttpResponse::Forbidden().json(HttpClientResponse {
            code: 2001,
            status: false,
            message: format!("Organization With Contact {} Exists", mobile),
            data: json!({}),
        }));
    }

    let organization = AddOrganizationDto {
        name,
        email,
        phone: mobile,
        address,
    };

    let result = save_organization(organization, &state).await;

    match result {
        Ok(res) => {
            let data = SaveMemberOrgDto {
                first_name,
                last_name,
                email: Some(member_email),
                phone: member_phone,
                address: member_address,
                gender,
                date_joined: Some(date_joined),
                date_of_birth: Some(date_of_birth),
            };

            let save_member = save_member_from_org(res.last_insert_id, data, &state).await;

            if let Err(e) = save_member {
                return Err(error::Error::from_db_err(e));
            };

            Ok(HttpResponse::Created().json(HttpClientResponse {
                code: 2000,
                status: true,
                message: "Organization Added Successfully".to_string(),
                data: json!(CreatedResponseModel {
                    organization: res.last_insert_id.to_string(),
                    member: save_member.unwrap().last_insert_id.to_string(),
                }),
            }))
        }
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(HttpClientResponse {
                code: 2001,
                status: false,
                message: format!("Error Adding Organization: {}", e),
                data: json!({}),
            }),
        ),
    }
}

pub async fn get_all(
    _req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let organizations = get_organizations(&state).await;

    match organizations {
        Ok(res) => Ok(HttpResponse::Ok().json(HttpClientResponse {
            code: 2000,
            status: true,
            message: "Organizations Fetched Successfully".to_string(),
            data: json!(res),
        })),
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(HttpClientResponse {
                code: 2001,
                status: false,
                message: format!("Error Fetching Organizations: {}", e),
                data: json!({}),
            }),
        ),
    }
}

pub async fn upload_img(
    _req: HttpRequest,
    data: web::Json<UploadImgModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let id = validator::uuid(&data.id, "ID")?;

    let data = match general_purpose::STANDARD.decode(&data.data) {
        Ok(data) => data,
        Err(e) => {
            return Ok(HttpResponse::BadRequest().json(HttpClientResponse {
                code: 2001,
                status: true,
                message: format!("Invalid Image with Error: {}", e),
                data: json!({}),
            }))
        }
    };

    let organization = get_organization_by_id(id, &state).await;

    if let Err(e) = organization {
        return Err(error::Error::from_db_err(e));
    };

    let owner = organization.unwrap().id;

    let media = SaveMediaDto {
        file_path: "uploads/".to_string(),
        mime_type: "image/png".to_string(),
        file_size: 0,
        file_name: format!("{}-{}", owner, uuid::Uuid::new_v4()),
        media_type: "image".to_string(),
        width: Some(0),
        height: Some(0),
        duration: Some(0),
    };

    let mime_type = media.mime_type.clone();
    let media_file_name = media.file_name.clone();

    let result = save_media_meta(owner, media, &state).await;

    if let Err(e) = result {
        return Err(error::Error::from_db_err(e));
    };

    let file_name = format!("{}-{}", result.unwrap().last_insert_id, media_file_name);
    let ext = mime_type.split('/').nth(1).unwrap();

    if let Err(err) = save_file(&file_name, &ext, &data).await {
        return Ok(
            HttpResponse::InternalServerError().json(HttpClientResponse {
                code: 2001,
                status: true,
                message: format!("Error Saving File: {}", err),
                data: json!({}),
            }),
        );
    };

    Ok(HttpResponse::Ok().json(HttpClientResponse {
        code: 2000,
        status: true,
        message: "Image Uploaded Successfully".to_string(),
        data: json!({}),
    }))
}
