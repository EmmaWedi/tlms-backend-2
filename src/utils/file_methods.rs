pub async fn save_file(file_name: &str, extension: &str, data: &[u8]) -> Result<(), std::io::Error> {
    let file_path = format!("uploads/{}.{}", file_name, extension);
    tokio::fs::write(file_path, data).await
}

pub async fn read_file(file_name: &str, extension: &str) -> Result<Vec<u8>, std::io::Error> {
    let file_path = format!("uploads/{}.{}", file_name, extension);
    tokio::fs::read(file_path).await
}

pub async fn file_exists(file_name: &str, extension: &str) -> bool {
    let file_path = format!("uploads/{}.{}", file_name, extension);
    tokio::fs::metadata(file_path).await.is_ok()
}
