use axum::{extract::Path, response::Html, routing::get, Extension, Router};
use html_strong::{document_tree::Node, science_lab::NodeExt, tags::*};
use reqwest::StatusCode;
use std::sync::Arc;

use crate::{
    base::html_doc,
    common::{no_such_page, render},
    components::Article,
};

#[derive(Debug, Clone)]
pub enum Rhs {
    OneImage { path: String },
    TwoImages { path1: String, path2: String },
    Code(String),
    Nothing,
}

impl Rhs {
    pub fn code(code: &str) -> Self {
        Self::Code(code.to_string())
    }

    pub fn one_image(path: &str) -> Self {
        Self::OneImage {
            path: path.to_string(),
        }
    }

    pub fn two_images(path1: &str, path2: &str) -> Self {
        Self::TwoImages {
            path1: path1.to_string(),
            path2: path2.to_string(),
        }
    }

    fn url_prefix(&mut self, prefix: &str) {
        match self {
            Rhs::OneImage { path } => *path = format!("{prefix}/{path}"),
            Rhs::TwoImages { path1, path2 } => {
                *path1 = format!("{prefix}/{path1}");
                *path2 = format!("{prefix}/{path2}");
            }
            _ => {}
        }
    }
}

#[derive(Debug, Clone)]
pub struct Card {
    /// Card title.
    title: String,

    /// Displayed in a less prominent manner than the title.
    subtitle: String,

    /// Card description.
    description: String,

    /// Url to go to when card is clicked.
    url: String,

    /// What do display on the right hand side of the card.
    rhs: Rhs,
}

impl Card {
    /// A card.
    ///
    /// # Example
    ///
    /// ```rust
    /// use html_strong_homepage::page::{Card, Rhs};
    ///
    /// // Creates a card with nothing displayed on the right-hand side.
    /// let card = Card::new("My Card", "2022-06-26", "This is my card", "nrk.no", Rhs::Nothing);
    /// ```
    pub fn new(title: &str, subtitle: &str, description: &str, url: &str, rhs: Rhs) -> Self {
        Self {
            title: title.to_string(),
            subtitle: subtitle.to_string(),
            description: description.to_string(),
            url: url.to_string(),
            rhs,
        }
    }
}

impl NodeExt for Card {
    fn into_node(self) -> Node {
        // Wrap the whole thing in a clickable link
        let card = A::href(&self.url);

        // The card will always have a title, subltitle, and a description
        let title_subtitle = Div.kid(H2.text(self.title)).kid(Em.text(self.subtitle));
        let description = P.text(self.description);

        let card_contents = Div.kid(title_subtitle).kid(description);

        let thumbnail_classes = "card-thumbnail rounded center";

        // The right hand side of the card might have various things,
        // which also determines the grid class.
        let card_contents = match self.rhs {
            Rhs::OneImage { path } => card_contents
                .class("grid-3")
                .kid(Img::new(&path).class(thumbnail_classes)),
            Rhs::TwoImages { path1, path2 } => card_contents
                .class("grid-4")
                .kid(Img::new(&path1).class(thumbnail_classes))
                .kid(Img::new(&path2).class(thumbnail_classes)),
            Rhs::Code(code) => card_contents
                .class("grid-3")
                .kid(Pre.kid(Code.class("rounded language-rust").text(code))),
            Rhs::Nothing => card_contents.class("grid-2"),
        };

        Div.class("card-bg padding rounded ease link-reset")
            .kid(card.kid(card_contents))
    }
}

#[derive(Debug, Clone)]
struct Post {
    card: Card,
    contents: Article,
}

impl Post {
    pub fn new(card: Card, contents: Article) -> Self {
        Self { card, contents }
    }
}

#[derive(Debug, Clone)]
struct Series {
    pub card: Card,
    pub posts: Vec<Post>,
}

impl Series {
    pub fn new(card: Card, posts: Vec<Post>) -> Self {
        Self { card, posts }
    }

    pub fn posts(&self) -> &[Post] {
        &self.posts
    }
}

#[derive(Debug, Clone)]
struct Context {
    /// Page title.
    title: String,

    /// Page description.
    description: String,

    /// The series on this page.
    series: Arc<Vec<Series>>,

    /// Url to this page.
    pub url: &'static str,
}

async fn render_page(node: Node) -> Result<Html<String>, (StatusCode, String)> {
    let html = html_doc(
        Some(vec![
            // For highlight.js
            "/static/css/shared/monokai.min.css",
        ]),
        Some(vec![
            "/static/js/highlight.min.js",
        ]),
        Some(vec!["hljs.highlightAll();"]),
        node,
    );

    render(html)
}

async fn page(Extension(state): Extension<Page>) -> Result<Html<String>, (StatusCode, String)> {
    let &Context {
        title, description, ..
    } = &state.context.as_ref();

    let mut content = Div
        .class("page")
        .kid(H1.text(title))
        .kid(P.text(description))
        .kid(Br);

    for series in state.series() {
        content.push_kid(series.card.clone().class("breather-y"));
    }

    render_page(content.into_node()).await
}

async fn series(
    Path(series_path): Path<String>,
    Extension(state): Extension<Page>,
) -> Result<Html<String>, (StatusCode, String)> {
    let mut content = Div.class("series");

    if let Some(series) = state.serie(&series_path) {
        for post in &*series.posts() {
            content.push_kid(post.card.clone().class("breather-y"));
        }
        render_page(content.into_node()).await
    } else {
        Err(no_such_page(series_path).await)
    }
}

async fn post(
    Path((series_path, post_path)): Path<(String, String)>,
    Extension(state): Extension<Page>,
) -> Result<Html<String>, (StatusCode, String)> {
    if let Some(post) = state.post(&series_path, &post_path) {
        render_page(Div.class("post").kid(post.contents.clone().into_node())).await
    } else {
        Err(no_such_page(format!("{series_path}/{post_path}")).await)
    }
}

#[derive(Debug, Clone)]
pub struct Page {
    context: Arc<Context>,
}

#[derive(Debug, Clone)]
pub struct PageBuilder {
    url: &'static str,
    title: String,
    description: String,
    series: Vec<Series>,
}

impl PageBuilder {
    pub fn new(url: &'static str, title: &str, description: &str) -> Self {
        Self {
            url,
            title: title.to_string(),
            description: description.to_string(),
            series: vec![],
        }
    }

    pub fn series(
        mut self,
        url: &str,
        title: &str,
        subtitle: &str,
        description: &str,
        mut rhs: Rhs,
    ) -> Self {
        let url = format!("{}/{url}", self.url);

        // url already has a leading slash.
        rhs.url_prefix(&format!("/static{url}"));

        self.series.push(Series::new(
            Card::new(title, subtitle, description, &url, rhs),
            vec![],
        ));
        self
    }

    pub fn post(
        mut self,
        url: &str,
        title: &str,
        subtitle: &str,
        description: &str,
        mut rhs: Rhs,
        mut contents: Article,
    ) -> Self {
        let current_series = self
            .series
            .last_mut()
            .expect("should start a series before posts are added");

        let url = format!("{}/{url}", current_series.card.url);
        let static_url = format!("/static{url}");

        // Gotta update the card href with the url prefix.
        rhs.url_prefix(&static_url);

        // Gotta update the contents with the url prefix.
        // Note: The url already has a leading slash.
        contents.url_prefix = Some(static_url);

        current_series.posts.push(Post::new(
            Card::new(title, subtitle, description, &url, rhs),
            contents,
        ));
        self
    }

    pub fn build(self) -> Page {
        Page::new(self.url, &self.title, &self.description, self.series)
    }
}

impl Page {
    fn new(url: &'static str, title: &str, description: &str, series: Vec<Series>) -> Self {
        let context = Context {
            title: title.to_string(),
            description: description.to_string(),
            series: Arc::new(series),
            url,
        };

        Self {
            context: Arc::new(context),
        }
    }

    pub fn router(&self) -> Router {
        let state: Page = self.clone();

        Router::new()
            .route("/", get(page))
            .route("/:series", get(series))
            .route("/:series/:post", get(post))
            .layer(Extension(state))
    }

    pub fn url(&self) -> &'static str {
        self.context.url
    }

    fn series(&self) -> &[Series] {
        self.context.as_ref().series.as_ref()
    }

    fn serie(&self, serie_url: &str) -> Option<&Series> {
        self.series()
            .iter()
            .find(|serie| serie.card.url == format!("{}/{serie_url}", self.url()))
    }

    fn post(&self, serie_url: &str, post_url: &str) -> Option<&Post> {
        self.serie(serie_url).and_then(|serie| {
            serie
                .posts()
                .iter()
                .find(|post| post.card.url == format!("{}/{serie_url}/{post_url}", self.url()))
        })
    }
}
