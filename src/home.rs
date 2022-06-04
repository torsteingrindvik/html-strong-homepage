use axum::response::Html;
use html_strong::{document_tree::Node, science_lab::NodeExt, tags::*};
use reqwest::StatusCode;

use crate::{base::html_doc, common::render};

struct Entry {
    name: String,
    description: String,
}

impl Entry {
    fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
        }
    }
}

impl NodeExt for Entry {
    fn into_node(self) -> Node {
        Div.class("breather-y")
            .kid(H2.text(self.name))
            .kid(P.text(self.description))
    }
}

pub async fn home() -> Result<Html<String>, (StatusCode, String)> {
    let contents = Div
        .kid(H1.text("torstein's homepage"))
        .kid(P.text("It's my homepage."))
        .kid(Entry::new(
            "Blog 📚",
            "The blog will™ contain explorations of Rust stuff probably.",
        ))
        .kid(Entry::new(
            "Bus 🚍",
            "This is just a shortcut for me to quickly check when the bus comes and goes.
            HAH! Just kidding, I bought a car before I implemented this.",
        ))
        .kid(Entry::new(
            "Blender ⛰️",
            "If I ever git gud at Blender it would be fun to have some progress images.",
        ))
        .kid(Entry::new(
            "Training 🏋️",
            "Just some broscience notes on exercise, don't mind carry on.",
        ));

    let html = html_doc::<String>(None, None, None, contents);

    render(html).await
}
