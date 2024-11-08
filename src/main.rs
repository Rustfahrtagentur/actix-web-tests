use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use minio::s3::{client::ClientBuilder, creds::StaticProvider};
use s3_client::{
    app::AppState,
    configuration::{get_configuration, MqttSettings, S3Settings},
    error::Error,
    routes::{get_image, upload_image},
};
use std::{fs, sync::Arc, time::Duration};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load config
    let config = get_configuration().expect("Failed to read configuration.");

    // create instance of mqtt client
    let mqtt_client = create_mqtt_client(&config.mqtt).expect("Failed to create mqtt client.");

    // create instance of s3 client
    let minio_client = create_s3_client(&config.s3).await.expect("Failed to create s3.");

    // Wrap the clients in an Arc and store in AppState
    let app_state = web::Data::new(AppState {
        minio_client: Arc::new(minio_client),
        mqtt_client: Arc::new(mqtt_client),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/", web::get().to(index))
            .route("/upload", web::post().to(upload_image))
            .route("/get/{username}/{filename}", web::get().to(get_image))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn create_s3_client(s3settings: &S3Settings) -> Result<minio::s3::Client, Error> {
    // Create the MinIO client using ClientBuilder
    let static_provider = StaticProvider::new(&s3settings.access_key, &s3settings.secret_key, None);
    // Build S3 connection string
    let connection_string = format!("{}:{}", &s3settings.host, &s3settings.port)
        .as_str()
        .parse()?;

    let client = ClientBuilder::new(connection_string)
        .provider(Some(Box::new(static_provider)))
        .build()?;

    Ok(client)
}

fn create_mqtt_client(mqtt_settings: &MqttSettings) -> Result<paho_mqtt::Client, Error> {
    let host = &mqtt_settings.host;
    let port = &mqtt_settings.port;

    // Create a client & define connect options
    let mqtt_client = paho_mqtt::Client::new(format!("tcp://{host}:{port}"))?;

    let conn_opts = paho_mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(true)
        .finalize();

    mqtt_client.connect(conn_opts)?;

    // Create a message and publish it
    let msg = paho_mqtt::Message::new("rustfahrtagentur", "Hello world! 23", 2);
    mqtt_client.publish(msg)?;

    Ok(mqtt_client)
}

async fn index() -> impl Responder {
    let path = "static/index.html";
    match fs::read_to_string(path) {
        Ok(contents) => HttpResponse::Ok().content_type("text/html").body(contents),
        Err(_) => HttpResponse::InternalServerError().body("Failed to load index.html"),
    }
}
