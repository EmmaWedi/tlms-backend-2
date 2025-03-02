

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
    pub date_of_birth: Option<chrono::NaiveDate>,
}

pub struct SaveMediaDto {
    pub file_name: String,
    pub mime_type: String,
    pub file_path: String,
    pub file_size: i64,
    pub media_type: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration: Option<i32>,
}