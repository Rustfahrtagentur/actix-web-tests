#[derive(Debug)]
pub enum Error {
    S3(Box<minio::s3::error::Error>),
    MQTT(Box<paho_mqtt::Error>),
}

impl From<paho_mqtt::Error> for Error {
    fn from(value: paho_mqtt::Error) -> Self {
        Self::MQTT(Box::new(value))
    }
}

impl From<minio::s3::error::Error> for Error {
    fn from(value: minio::s3::error::Error) -> Self {
        Self::S3(Box::new(value))
    }
}
