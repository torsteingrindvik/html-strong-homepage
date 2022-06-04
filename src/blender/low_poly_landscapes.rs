use axum::Extension;
use axum::{extract::Path, response::Html, routing::get, Router};
use html_strong::{science_lab::NodeExt, tags::*};
use reqwest::StatusCode;

use super::{blender_page, BlenderSeriesHeader, Series, State};

pub fn router() -> Router {
    Router::new()
        .route("/", get(landing))
        .route("/:sub", get(sub))
}

pub async fn landing(
    Extension(state): Extension<State>,
) -> Result<Html<String>, (StatusCode, String)> {
    let header = BlenderSeriesHeader::new(state.get(&Series::LowPolyLandscapes).unwrap().clone());

    blender_page(header.id("low-poly-landscapes")).await
}

pub async fn sub(
    Path(sub): Path<String>,
    Extension(state): Extension<State>,
) -> Result<Html<String>, (StatusCode, String)> {
    blender_page(H1.text(&format!("Yah: {} + {sub}, heh: {state:?}", "hi"))).await
}
