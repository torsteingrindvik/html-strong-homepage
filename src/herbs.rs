use std::{io::Cursor, path::PathBuf};

use axum::{response::IntoResponse, routing::post, Json, Router};
use image::GenericImageView;
use reqwest::StatusCode;
use thiserror::Error;
use tower_http::{auth, limit};
use tracing::{debug, info};

use shared::image::Image;

pub mod basil;

#[derive(Debug, Error)]
pub enum ImageError {
    #[error("Io issue")]
    Io(#[from] std::io::Error),

    #[error("Image decode issue")]
    Image(#[from] image::ImageError),

    #[error("Image wrong size {width}, {height}, want 1280x720")]
    BadSize { width: u32, height: u32 },
}

impl IntoResponse for ImageError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, format!("{self:?}")).into_response()
    }
}

async fn handle_new_image(Json(image): Json<Image>) -> Result<(), ImageError> {
    debug!(?image, "New image");
    let file_name = image.filename();

    let image = image::io::Reader::new(Cursor::new(image.buffer))
        .with_guessed_format()?
        .decode()?;

    let (width, height) = image.dimensions();
    if width != 1280 || height != 720 {
        return Err(ImageError::BadSize { width, height });
    }

    let output_path = PathBuf::from(format!(
        "{}/upload/herbs/webcam/{file_name}.jpg",
        std::env!("CARGO_MANIFEST_DIR")
    ));
    debug!("Storing image: `{output_path:?}`");

    if let Some(parent) = output_path.parent() {
        if !parent.exists() {
            info!("Creating folder: {parent:?}");

            std::fs::create_dir_all(parent)?;
        }
    }

    image.save(output_path)?;

    debug!("Image saved!");

    Ok(())
}

pub fn new_image_endpoint() -> (&'static str, Router) {
    (
        shared::herbs::HERBS_NEW_IMAGE_POST_ENDPOINT,
        Router::new()
            .route("/", post(handle_new_image))
            .layer(auth::RequireAuthorizationLayer::bearer(
                &shared::herbs::herbs_new_image_auth(),
            ))
            .layer(
                // Max 500 KB images
                limit::RequestBodyLimitLayer::new(1024 * 500),
            ),
    )
}
