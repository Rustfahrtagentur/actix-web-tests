use crate::app::AppState;
use actix_multipart::{form::tempfile::TempFile, form::text::Text, form::MultipartForm};
use actix_web::{web, Error, HttpResponse, Responder};
use minio::s3::{
    args::{BucketExistsArgs, MakeBucketArgs},
    builders::ObjectContent,
    types::S3Api,
};

const BUCKET_NAME: &str = "s3-client-test";

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(limit = "100MB")]
    file: TempFile,
    username: Text<String>,
}

async fn upload_image_intern(
    app_state: web::Data<AppState>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<(), minio::s3::error::Error> {
    println!("{:?}", form);

    let minio_client = &app_state.minio_client;

    let exists = minio_client
        .bucket_exists(&BucketExistsArgs::new(BUCKET_NAME)?)
        .await?;

    if !exists {
        minio_client
            .make_bucket(&MakeBucketArgs::new(BUCKET_NAME)?)
            .await?;
    }

    let object_path = format!(
        "{}/{}",
        form.username.0,
        form.file.file_name.expect("filename missing")
    );
    let content = ObjectContent::from(form.file.file.path());

    minio_client
        .put_object_content(BUCKET_NAME, &object_path, content)
        .send()
        .await?;

    Ok(())
}

pub async fn upload_image(
    app_state: web::Data<AppState>,
    form: MultipartForm<UploadForm>,
) -> impl Responder {
    match upload_image_intern(app_state, form).await {
        Ok(_) => HttpResponse::Ok().body("Image uploaded successfully"),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to upload image: {}", e))
        }
    }
}

pub async fn get_image(
    app_state: web::Data<AppState>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, Error> {
    let minio_client = &app_state.minio_client;

    let (username, filename) = path.into_inner();

    let object_path = format!("{}/{}", username, filename);

    match minio_client
        .get_object(BUCKET_NAME, &object_path)
        .send()
        .await
    {
        Ok(_response) => {
            // let foo = response.content.to_stream().await?;
            // let foo2 = response.content.to_segmented_bytes().await?;
            // let foo3 = &foo2.to_bytes();
            // let mut buffer = Vec::new();
            // response.read_to_end(&mut buffer).unwrap();
            // Ok(HttpResponse::Ok().content_type("application/octet-stream").body(&foo3))

            Ok(HttpResponse::NotImplemented().finish())
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().body(format!("Failed to get image: {}", e)))
        }
    }
}
