use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use minio::s3::{client::ClientBuilder, creds::StaticProvider};
use s3_client::app::AppState;
use s3_client::configuration::get_configuration;
use s3_client::routes::{get_image, upload_image};
use std::fs;
use std::path::Path;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load config
    let config = get_configuration().expect("Failed to read configuration.");

    // Create the MinIO client using ClientBuilder
    let static_provider = StaticProvider::new(&config.s3.access_key, &config.s3.secret_key, None);

    let s3_connection_string = format!("{}:{}", &config.s3.host, &config.s3.port)
        .as_str()
        .parse()
        .unwrap();
    let minio_client = ClientBuilder::new(s3_connection_string)
        .provider(Some(Box::new(static_provider)))
        .build()
        .expect("Failed to create MinIO client");

    // Wrap the client in an Arc and store in AppState
    let app_state = web::Data::new(AppState {
        minio_client: Arc::new(minio_client),
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

async fn index() -> impl Responder {
    let path = Path::new("static/index.html");
    match fs::read_to_string(path) {
        Ok(contents) => HttpResponse::Ok().content_type("text/html").body(contents),
        Err(_) => HttpResponse::InternalServerError().body("Failed to load index.html"),
    }
}
