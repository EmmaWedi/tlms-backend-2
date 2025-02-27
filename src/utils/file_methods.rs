pub async fn save_file(file_path: &str, data: &[u8]) -> Result<(), std::io::Error> {
    tokio::fs::write(file_path, data).await
}

pub async fn read_file(file_path: &str) -> Result<Vec<u8>, std::io::Error> {
    tokio::fs::read(file_path).await
}

pub async fn file_exists(path: &str) -> bool {
    tokio::fs::metadata(path).await.is_ok()
}
