use axum::Extension;
use axum::{extract::Path, response::Html, routing::get, Router};
use html_strong::{science_lab::NodeExt, tags::*};
use reqwest::StatusCode;

use crate::{Base, ContentUrl};

use super::{blog_page, BlogEntry, Series, State};

pub fn router() -> Router {
    Router::new()
        .route("/", get(landing))
        .route("/:sub", get(sub))
}

pub async fn landing(
    Extension(state): Extension<State>,
) -> Result<Html<String>, (StatusCode, String)> {
    let mut url = ContentUrl::new(Base::Blog);
    url.dive("tracing");

    let state = state.get(&Series::Tracing).unwrap().clone();

    let tracing = BlogEntry::new(
        &url.suburl("2022-06-04"),
        super::EntryState {
            name: "2022-06-04".into(),
            description: "Hello world/tracing! Let's get an overview of what tracing is and why \
                          we'd want to use it."
                .into(),
        },
    );

    let contents = Div
        .kid(H1.text(state.name))
        .kid(P.text(state.description))
        .kid(Br)
        .kid(Div.class("breather-y").kid(tracing));

    blog_page(contents).await
}

pub async fn sub(
    Path(sub): Path<String>,
    Extension(state): Extension<State>,
) -> Result<Html<String>, (StatusCode, String)> {
    blog_page(H1.text(&format!("Yah: {} + {sub}, heh: {state:?}", "hi"))).await
}
