use anyhow::Result;
use axum::{extract::Multipart, http::StatusCode, response::IntoResponse, routing::post, Router};
use std::fs;

pub fn routes() -> Router {
    Router::new().route("/upload", post(upload_file))
}

async fn upload_file(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name() == Some("file") {
            let data = field.bytes().await.unwrap();

            let filename = "uploaded_file.bin";
            let filepath = format!("uploads/{}", filename);

            if let Err(e) = fs::write(&filepath, &data) {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to save file: {}", e),
                )
                    .into_response();
            }

            return (StatusCode::OK, "File uploaded successfully").into_response();
        }
    }

    (StatusCode::BAD_REQUEST, "File field missing").into_response()
}
