use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddOrganizationModel {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub first_name: String,
    pub last_name: String,
    pub member_email: Option<String>,
    pub member_phone: String,
    pub member_address: String,
    pub gender: String,
    pub date_joined: Option<chrono::NaiveDate>,
    pub date_of_birth: Option<chrono::NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddOrganizationDto {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateOrganizationDto {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatedResponseModel {
    pub organization: String,
    pub member: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadImgModel {
    pub id: String,
    pub data: String,
}