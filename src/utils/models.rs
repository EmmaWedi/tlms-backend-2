

use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct HttpClientResponse {
    pub code: u16,
    pub status: bool,
    pub message: String,
    pub data: Value
}

pub struct SaveMemberOrgDto {
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: String,
    pub address: String,
    pub gender: String,
    pub date_joined: Option<chrono::NaiveDate>,
}