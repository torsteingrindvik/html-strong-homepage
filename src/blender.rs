use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::Path;
use axum::response::Html;
use axum::routing::get;
use axum::{Extension, Router};
use html_strong::document_tree::Node;
use html_strong::{science_lab::NodeExt, tags::*};
use reqwest::StatusCode;

use crate::base::html_doc;
use crate::common::render;
use crate::{Base, ContentUrl};

mod low_poly_landscapes;
mod low_poly_characters;

#[derive(Debug, Clone)]
struct BeforeAfterImages {
    before: String,
    after: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Series {
    LowPolyLandscapes,
    LowPolyCharacters,
}

#[derive(Debug, Clone)]
pub struct SeriesState {
    name: String,
    description: String,
    before_after: BeforeAfterImages,
}

#[derive(Debug, Clone)]
struct BlenderSeries {
    path: String,
    series: SeriesState,
}

impl BlenderSeries {
    fn new(path: &str, info: SeriesState) -> Self {
        Self {
            path: path.to_string(),
            series: info,
        }
    }
}

impl NodeExt for BlenderSeries {
    fn into_node(self) -> Node {
        let content = ContentUrl::new(Base::Blender);
        let thumbnail_classes = "blender-series-thumbnail rounded";

        Div.class("card-bg padding rounded ease link-reset breather-y").kid(
            A::href(&self.path).kid(
                Div.class("grid-4")
                    .kid(H2.text(self.series.name))
                    .kid(P.text(self.series.description))
                    .kid(
                        Img::new(&content.image(&self.series.before_after.before))
                            .class(thumbnail_classes),
                    )
                    .kid(
                        Img::new(&content.image(&self.series.before_after.after))
                            .class(thumbnail_classes),
                    ),
            ),
        )
    }
}

#[derive(Debug, Clone)]
struct BlenderSeriesHeader {
    series: SeriesState,
}
impl BlenderSeriesHeader {
    fn new(info: SeriesState) -> Self {
        Self { series: info }
    }
}

impl NodeExt for BlenderSeriesHeader {
    fn into_node(self) -> Node {
        let content = ContentUrl::new(Base::Blender);
        let thumbnail_classes = "blender-series-thumbnail rounded";

        Div.kid(
            Div.class("blender-series-header")
                .kid(H2.text(self.series.name))
                .kid(Em.text(self.series.description))
                .kid(
                    Img::new(&content.image(&self.series.before_after.before))
                        .class(thumbnail_classes),
                )
                .kid(
                    Img::new(&content.image(&self.series.before_after.after))
                        .class(thumbnail_classes),
                ),
        )
        .kid(Hr.class("breather-y"))
    }
}

async fn blender_page(node: Node) -> Result<Html<String>, (StatusCode, String)> {
    let content = ContentUrl::new(Base::Blender);
    let html = html_doc(Some(vec![content.css("blender.css")]), None, None, node);

    render(html).await
}

pub type State = Arc<HashMap<Series, SeriesState>>;

pub fn blender() -> Router {
    let state: State = Arc::new(HashMap::from([
        (
            Series::LowPolyLandscapes,
            SeriesState {
                before_after: BeforeAfterImages {
                    before: "after-the-mirror-modifier.webp".into(),
                    after: "goal.webp".into(),
                },
                name: "Low Poly Landscapes".into(),
                description: "A tutorial on creating low polygon count landscapes.
		Very stylized.
		Taught by Grant Abbitt."
                    .into(),
            },
        ),
        (
            Series::LowPolyCharacters,
            SeriesState {
                before_after: BeforeAfterImages {
                    before: "low-poly-characters/cube.webp".into(),
                    after: "low-poly-characters/goal.webp".into(),
                },
                name: "Low Poly Characters".into(),
                description: "A tutorial on creating low polygon count characters.
		Very stylized.
		Taught by Grant Abbitt."
                    .into(),
            },
        ),
    ]));

    Router::new()
        .route("/", get(landing))
        .nest("/low-poly-landscapes", low_poly_landscapes::router())
        .nest("/low-poly-characters", low_poly_characters::router())
        .layer(Extension(state))
}

pub async fn series(Path(series): Path<String>) -> Result<Html<String>, (StatusCode, String)> {
    let mut url = ContentUrl::new(Base::Blender);
    url.dive(&series);

    blender_page(H1.text(&format!("Yah: {}", url.url()))).await
}

pub async fn landing(
    Extension(state): Extension<State>,
) -> Result<Html<String>, (StatusCode, String)> {
    let content = ContentUrl::new(Base::Blender);

    let low_poly_landscapes = BlenderSeries::new(
        &content.suburl("low-poly-landscapes"),
        state.get(&Series::LowPolyLandscapes).unwrap().clone(),
    );

    let low_poly_characters = BlenderSeries::new(
        &content.suburl("low-poly-characters"),
        state.get(&Series::LowPolyCharacters).unwrap().clone(),
    );

    let contents = Div
        .kid(H1.text("Blender Work Logs"))
        .kid(P.text("Here follows my work logs (e.g. in-progress images and such)."))
        .kid(Br)
        .kid(P.text(
            "I might log work from following paid tutorials, youtube videos, or just doodling.",
        ))
        .kid(P.text(
            "The point anyway is to have something to look back at in the future, and to not take \
             learning Blender too seriously.",
        ))
        .kid(
            Div.class("breather-y")
                .kid(low_poly_landscapes)
                .kid(low_poly_characters),
        );

    blender_page(contents).await
}
