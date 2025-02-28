use sha2::{Digest, Sha512};

pub fn salt() -> uuid::Uuid {
    uuid::Uuid::new_v4()
}

pub fn parse_uuid(id: &str) -> uuid::Uuid {
    uuid::Uuid::parse_str(id).expect("Invalid UUID string")
}

pub fn encrypt_password(password: &str, _salt: &uuid::Uuid) -> String {
    let pwd = format!("{}{}", password, _salt.to_string());

    let pwd = md5::compute(pwd);

    let pwd = format!("{:?}{}{}", pwd, password, _salt.to_string());

    let pwd = Sha512::new().chain_update(pwd).finalize();

    format!("{:x}", pwd)
}

pub fn validate_password(password: &str, salt: &uuid::Uuid, hash: &str) -> bool {
    let encrypted = encrypt_password(password, salt);

    encrypted == hash
}