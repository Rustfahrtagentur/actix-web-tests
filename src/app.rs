use minio::s3::client::Client;
use std::sync::Arc;

pub struct AppState {
    pub minio_client: Arc<Client>,
}
