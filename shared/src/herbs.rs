use std::env;

use tracing::warn;

// TODO: Would be nice to create this from parts.
/// Endpoint where [`Image`]s can be POSTed.
pub const HERBS_NEW_IMAGE_POST_ENDPOINT: &'static str = "/herbs-new-image";

/// The bearer token needed to post an image.
/// Stored in the env var `HERBS_NEW_IMAGE_PW`, or a fallback default if that is not set.
pub fn herbs_new_image_auth() -> String {
    env::var("HERBS_NEW_IMAGE_PW").unwrap_or_else(|_| {
        warn!("Default herbs new image bearer token used!");
        "basilisk".to_string()
    })
}
