use std::sync::Arc;

use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use serde_json::json;

use crate::{
    app::members::{dto::dtos::{get_all_members, get_member_by_phone, save_member}, models::model::{AddMemberDto, AddMemberModel}},
    libs::{error, jwt::Claims, pword::parse_uuid, validator},
    utils::models::HttpClientResponse,
    AppState,
};

pub async fn add_member(
    req: HttpRequest,
    payload: web::Json<AddMemberModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let first_name = validator::required_str(&payload.first_name, "First Name")?;
    let last_name = validator::required_str(&payload.last_name, "Last Name")?;
    let email = validator::email(payload.email.as_deref().unwrap_or(""), "Email")?;
    let mobile = validator::mobile(&payload.phone, "Phone")?;
    let address = validator::required_str(&payload.address, "Address")?;
    let gender = validator::required_str(&payload.gender, "Gender")?;
    let date_joined = validator::date(
        payload
            .date_joined
            .map(|d| d.to_string())
            .as_deref()
            .unwrap_or(""),
        "Date Joined",
    )?;

    let claims = req.extensions().get::<Arc<Claims>>().cloned();

    //move this check into middleware that checks user role and add only org id to the extension
    // if claims.is_none() {
    //     return Ok(HttpResponse::Unauthorized().json(HttpClientResponse {
    //         code: 2000,
    //         status: false,
    //         message: "Unauthorized".to_string(),
    //         data: json!({}),
    //     }));
    // }

    let organization_id = parse_uuid(&claims.unwrap()._id);

    if let Ok(Some(_)) = get_member_by_phone(&mobile, &state).await {
        return Ok(HttpResponse::Forbidden().json(HttpClientResponse {
            code: 2001,
            status: false,
            message: format!("Member With Contact {} Exists", mobile),
            data: json!({}),
        }));
    }

    let member = AddMemberDto {
        first_name,
        last_name,
        email: Some(email),
        phone: mobile,
        organization_id,
        gender,
        address,
        date_joined: Some(date_joined)
    };

    let result = save_member(member, &state).await;

    match result {
        Ok(res) => Ok(HttpResponse::Ok().json(HttpClientResponse {
            code: 200,
            status: true,
            message: "Member Added Successfully".to_string(),
            data: json!(res.last_insert_id),
        })),
        Err(err) => Ok(HttpResponse::InternalServerError().json(HttpClientResponse {
            code: 500,
            status: false,
            message: format!("Failed to Add Member: {}", err),
            data: json!({}),
        })),
    }
}

pub async fn get_all(_req: HttpRequest, state: web::Data<AppState>) -> Result<HttpResponse, error::Error> {
    let members = get_all_members(&state).await;

    match members {
        Ok(res) => Ok(HttpResponse::Ok().json(HttpClientResponse {
            code: 2000,
            status: true,
            message: "Members Retrieved Successfully".to_string(),
            data: json!(res)
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(HttpClientResponse {
            code: 2001,
            status: false,
            message: format!("Error Retrieving Members: {}", e),
            data: json!({})
        }))
    }
}