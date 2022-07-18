use axum::response::Html;
use cached::proc_macro::cached;
use html_strong::{document_tree::Node, science_lab::NodeExt, tags::*};
use reqwest::StatusCode;

use crate::{base::html_doc, common::render, Base, ContentUrl};

struct Entry {
    name: String,
    url: String,
    description: String,
}

impl Entry {
    fn new(url: String, name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            url,
        }
    }
}

impl NodeExt for Entry {
    fn into_node(self) -> Node {
        Div.class("link-reset rounded padding breather-y card-bg ease")
            .kid(
                A::href(&self.url)
                    .kid(H2.text(self.name))
                    .kid(P.text(self.description)),
            )
    }
}

#[cached(size = 1, result = true)]
fn home_impl() -> Result<Html<String>, (StatusCode, String)> {
    let contents = Div
        .kid(H1.text("torstein's homepage"))
        .kid(P.text("It's my homepage. Here's the stuff you can look at:"))
        .kid(Entry::new(
            ContentUrl::new(Base::Blog).url(),
            "Blog 📚",
            "The blog will™ contain explorations of Rust stuff probably.",
        ))
        .kid(Entry::new(
            ContentUrl::new(Base::Herbs).url(),
            "Herbs 🌱",
            "Let's grow these.",
        ))
        .kid(Entry::new(
            ContentUrl::new(Base::Timelapse).url(),
            "Timelapse 🕒",
            "Auto-uploaded timelapse videos of herbs!",
        ))
        .kid(Entry::new(
            ContentUrl::new(Base::Blender).url(),
            "Blender ⛰️",
            "If I ever git gud at Blender it would be fun to have some progress images.",
        ))
        .kid(Entry::new(
            ContentUrl::new(Base::Training).url(),
            "Training 🏋️",
            "Just some broscience notes on exercise, don't mind carry on.",
        ));

    let html = html_doc::<String>("torste.in", None, None, None, contents);

    render(html)
}

pub async fn home() -> Result<Html<String>, (StatusCode, String)> {
    home_impl()
}
