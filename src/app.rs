use minio::s3::client::Client;
use std::sync::Arc;
extern crate paho_mqtt as mqtt;

pub struct AppState {
    pub minio_client: Arc<Client>,
    pub mqtt_client: Arc<mqtt::Client>,
}
