use std::env;

use tracing::warn;

// TODO: Would be nice to create this from parts.
/// Endpoint where [`Image`]s can be POSTed.
pub const IMAGE_POST_ENDPOINT: &'static str = "/herbs-new-image";

/// The bearer token needed to post an image.
/// Stored in the env var `HERBS_NEW_IMAGE_PW`, or a fallback default if that is not set.
pub fn new_image_auth() -> String {
    env::var("HERBS_NEW_IMAGE_PW").unwrap_or_else(|_| {
        warn!("Default herbs new image bearer token used!");
        "basilisk".to_string()
    })
}

pub fn new_image_output_relative_folder() -> &'static str {
    "upload/herbs/webcam"
}

pub fn processed_image_output_relative_folder() -> &'static str {
    "processed/herbs/webcam"
}

pub fn timelapse_output_relative_folder() -> &'static str {
    "static/herbs/basil/timelapse/days"
}