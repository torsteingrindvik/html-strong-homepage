use axum::Extension;
use axum::{extract::Path, response::Html, routing::get, Router};
use html_strong::{science_lab::NodeExt, tags::*};
use reqwest::StatusCode;

// use crate::{Base, ContentUrl};

use super::{blender_page, BlenderSeriesHeader, Series, State};

pub fn router() -> Router {
    Router::new()
        .route("/", get(landing))
        .route("/:sub", get(sub))
}

pub async fn landing(
    Extension(state): Extension<State>,
) -> Result<Html<String>, (StatusCode, String)> {
    // let url = ContentUrl::new_with_subpage(Base::Blender, "low-poly-landscapes");
    // let content = ContentUrl::new(Base::Blender);

    let header = BlenderSeriesHeader::new(state.get(&Series::LowPolyLandscapes).unwrap().clone());

    blender_page(header.id("low-poly-landscapes")
        // Div.kid(H1.text("Early progress"))
        //     .kid(Img::new_sized(
        //         &content.image("after-the-mirror-modifier.webp"),
        //         640,
        //         480,
        //     ))
        //     .kid(H1.text("Goal"))
        //     .kid(Img::new_sized(&content.image("goal.webp"), 640, 480)),
    )
    .await
}

pub async fn sub(
    Path(sub): Path<String>,
    Extension(state): Extension<State>,
) -> Result<Html<String>, (StatusCode, String)> {
    blender_page(H1.text(&format!("Yah: {} + {sub}, heh: {state:?}", "hi"))).await
}
