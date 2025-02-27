use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddMemberModel {
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: String,
    pub address: String,
    pub gender: String,
    pub date_joined: Option<chrono::NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddMemberDto {
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: String,
    pub organization_id: uuid::Uuid,
    pub address: String,
    pub gender: String,
    pub date_joined: Option<chrono::NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMemberDto {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub gender: Option<String>,
    pub department: Option<String>,
    pub aux_department: Option<String>,
    pub sub_department: Option<String>,
    pub member_type: Option<String>,
}