#[derive(serde::Deserialize)]
pub struct Settings {
    pub s3: S3Settings,
    pub mqtt: MqttSettings,
}

#[derive(serde::Deserialize)]
pub struct S3Settings {
    pub host: String,
    pub port: u16,
    pub access_key: String,
    pub secret_key: String,
}

#[derive(serde::Deserialize)]
pub struct MqttSettings {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;
    settings.try_deserialize::<Settings>()
}
