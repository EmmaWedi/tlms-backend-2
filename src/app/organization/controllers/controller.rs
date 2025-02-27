use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;

use crate::{
    app::organization::{
        dto::dtos::{get_organization_by_phone, get_organizations, save_organization},
        models::model::{AddOrganizationDto, AddOrganizationModel, CreatedResponseModel},
    },
    libs::{error, validator},
    utils::{
        models::{HttpClientResponse, SaveMemberOrgDto},
        shared::save_member_from_org,
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

    if let Ok(Some(_)) = get_organization_by_phone(&mobile, &state).await {
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

pub async fn get_all(_req: HttpRequest, state: web::Data<AppState>) -> Result<HttpResponse, error::Error> {
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
