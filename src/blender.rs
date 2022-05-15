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

const THUMBNAIL_WIDTH: usize = 320;
const THUMBNAIL_HEIGHT: usize = 180;

#[derive(Debug, Clone)]
struct BeforeAfterImages {
    before: String,
    after: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Series {
    LowPolyLandscapes,
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

        Div.class("blender-series rounded ease link-reset").kid(
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
        let thumbnail_classes = "blender-series-thumbnail rounded margin-auto";

        Div.kid(H2.text(self.series.name).class("text-center"))
            .kid(
                Div.class("flex-column")
                    .kid(Em.text(self.series.description).class("breather-y"))
                    .kid(
                        // Thumbnails wrapped in a single div such that
                        // they flex-wrap together.
                        Div.class("flex-row flex-wrap")
                            .kid(
                                Img::new_sized(
                                    &content.image(&self.series.before_after.before),
                                    THUMBNAIL_WIDTH,
                                    THUMBNAIL_HEIGHT,
                                )
                                .class(thumbnail_classes),
                            )
                            .kid(
                                Img::new_sized(
                                    &content.image(&self.series.before_after.after),
                                    THUMBNAIL_WIDTH,
                                    THUMBNAIL_HEIGHT,
                                )
                                .class(thumbnail_classes),
                            ),
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

mod low_poly_landscapes;

pub type State = Arc<HashMap<Series, SeriesState>>;

pub fn blender() -> Router {
    let state: State = Arc::new(HashMap::from([(
        Series::LowPolyLandscapes,
        SeriesState {
            before_after: BeforeAfterImages {
                before: "after-the-mirror-modifier.webp".into(),
                after: "goal.webp".into(),
            },
            name: "Low Poly Landscapes".into(),
            description: "A tutorial on creating low polygon count landscapes.
		Very stylized.
		Taught by the superb Grant Abbitt."
                .into(),
        },
    )]));

    Router::new()
        .route("/", get(landing))
        .nest("/low-poly-landscapes", low_poly_landscapes::router())
        .layer(Extension(state))
}

pub async fn series(Path(series): Path<String>) -> Result<Html<String>, (StatusCode, String)> {
    let url = ContentUrl::new_with_subpage(Base::Blender, &series);

    blender_page(H1.text(&format!("Yah: {}", url.url()))).await
}

pub async fn landing(
    Extension(state): Extension<State>,
) -> Result<Html<String>, (StatusCode, String)> {
    let content = ContentUrl::new(Base::Blender);

    let tut1 = BlenderSeries::new(
        &content.suburl("low-poly-landscapes"),
        state.get(&Series::LowPolyLandscapes).unwrap().clone(),
    );

    let contents = Div
        .kid(H1.text("Blender Work Logs"))
        .kid(P.text("Here follows my work logs (e.g. in-progress images and such)."))
        .kid(Br)
        .kid(P.text("I might log work from following paid tutorials, youtube videos, or just doodling."))
        .kid(P.text("The point anyway is to have something to look back at in the future, and to not take learning Blender too seriously."))
        .kid(Div.class("breather-y").kid(tut1));

    // for fake_tutorial in 0..10 {
    //     contents.push_kid(BlenderSeries::new(
    //         &format!("Tutorial Ipsum {fake_tutorial}"),
    //         "This describes the thing as well",
    //         &content.suburl(&format!("fake-{fake_tutorial}")),
    //         "after-the-mirror-modifier.webp",
    //         "after-the-mirror-modifier.webp",
    //     ));
    // }

    blender_page(contents).await
}
