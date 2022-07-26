use std::{ops::Deref, sync::Arc};

use axum::{response::Html, Extension};
use cached::proc_macro::cached;
use html_strong::{document_tree::Node, science_lab::NodeExt, tags::td::td, tags::th::th, tags::*};
use reqwest::StatusCode;

use crate::{base::html_doc, common::render, page::PostLead, Base, ContentUrl};

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
        Div.class("link-reset rounded padding breather-y card-bg ease soft-shadow")
            .kid(
                A::href(&self.url)
                    .kid(H2.text(self.name))
                    .kid(P.text(self.description)),
            )
    }
}

#[derive(Debug, Clone)]
struct PostLeadTable(Vec<PostLead>);

impl NodeExt for PostLeadTable {
    fn into_node(self) -> Node {
        let mut table = Table.class("recent-posts-table breather-y rounded soft-shadow");

        // Header row
        table.push_kid(
            Tr.kid(th().text("Date"))
                .kid(th().text("Subject"))
                .kid(th().text("Title")),
        );

        // Assume posts are sorted such that oldest come first.
        for post_lead in self.0.into_iter().rev().take(5) {
            let date = post_lead.date.format("%Y-%m-%d").to_string();

            // URL in post is e.g. /herbs/basil/pruning
            let category: String = post_lead
                .url
                .split("/")
                .into_iter()
                .skip(1)
                .take(2)
                .collect::<Vec<&str>>()
                .join("/");

            let title = post_lead.title;
            let url = post_lead.url;

            table.push_kid(
                Tr.kid(td().text(date))
                    .kid(td().text(category))
                    .kid(td().kid(A::href(&url).text(title))),
            );
        }

        table
    }
}

#[cached(size = 1, result = true)]
fn home_impl(posts: Arc<Vec<PostLead>>) -> Result<Html<String>, (StatusCode, String)> {
    let recent_posts = PostLeadTable(posts.deref().clone());

    let contents = Div
        .kid(H1.text("torstein's homepage").class("breather-y"))
        .kid(P.text("It's my homepage. These are the most recent things that have happened:"))
        .kid(recent_posts)
        .kid(P.text("These are the various categories:"))
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

pub async fn home(
    Extension(posts): Extension<Arc<Vec<PostLead>>>,
) -> Result<Html<String>, (StatusCode, String)> {
    home_impl(posts)
}
