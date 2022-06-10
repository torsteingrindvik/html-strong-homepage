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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Series {
    Tracing,
}

#[derive(Debug, Clone)]
pub struct SeriesState {
    name: String,
    description: String,
}

#[derive(Debug, Clone)]
struct BlogSeries {
    path: String,
    series: SeriesState,
}

impl BlogSeries {
    fn new(path: &str, info: SeriesState) -> Self {
        Self {
            path: path.to_string(),
            series: info,
        }
    }
}

impl NodeExt for BlogSeries {
    fn into_node(self) -> Node {
        Div.class("card-bg padding rounded ease link-reset").kid(
            A::href(&self.path).kid(
                Div.class("grid-3")
                    .kid(H2.text(self.series.name))
                    .kid(P.text(self.series.description))
                    .kid(Pre.text("info_span!(\"hello-world\");")),
            ),
        )
    }
}

#[derive(Debug, Clone)]
pub struct EntryState {
    name: String,
    description: String,
}

#[derive(Debug, Clone)]
struct BlogEntry {
    path: String,
    entry: EntryState,
}

impl BlogEntry {
    fn new(path: &str, info: EntryState) -> Self {
        Self {
            path: path.to_string(),
            entry: info,
        }
    }
}

impl NodeExt for BlogEntry {
    fn into_node(self) -> Node {
        Div.class("card-bg padding rounded ease link-reset").kid(
            A::href(&self.path).kid(
                Div.class("grid-3")
                    .kid(H2.text(self.entry.name))
                    .kid(P.text(self.entry.description))
                    .kid(P.text("info_span!(\"hello-world\");")),
            ),
        )
    }
}

async fn blog_page(node: Node) -> Result<Html<String>, (StatusCode, String)> {
    let content = ContentUrl::new(Base::Blog);
    let html = html_doc::<String>(Some(vec![content.css("blog.css")]), None, None, node);

    render(html).await
}

mod tracing;

pub type State = Arc<HashMap<Series, SeriesState>>;

pub fn blog() -> Router {
    let state: State = Arc::new(HashMap::from([(
        Series::Tracing,
        SeriesState {
            name: "Tracing".into(),
            description: "An attempt at understanding tokio's tracing library.".into(),
        },
    )]));

    Router::new()
        .route("/", get(landing))
        .nest("/tracing", tracing::router())
        .layer(Extension(state))
}

pub async fn series(Path(series): Path<String>) -> Result<Html<String>, (StatusCode, String)> {
    let mut url = ContentUrl::new(Base::Blog);
    url.dive(&series);

    blog_page(H1.text(&format!("Yah: {}", url.url()))).await
}

pub async fn landing(
    Extension(state): Extension<State>,
) -> Result<Html<String>, (StatusCode, String)> {
    let content = ContentUrl::new(Base::Blog);

    let tracing = BlogSeries::new(
        &content.suburl("tracing"),
        state.get(&Series::Tracing).unwrap().clone(),
    );

    let contents = Div
        .kid(H1.text("Blog series"))
        .kid(P.text(
            "Here you'll find links to blog series. These will likely be explorations of various \
             Rust related things. 🦀",
        ))
        .kid(Br)
        .kid(Div.class("breather-y").kid(tracing));

    blog_page(contents).await
}
