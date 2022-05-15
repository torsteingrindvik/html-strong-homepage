use html_strong::science_lab::NodeExt;
use html_strong::{document_tree::Node, tags::*, template};

use crate::{Base, ContentUrl};

// use crate::base_urls;

// pub trait Extend {
//     fn router(&self) -> Router;
// }

/// Wrap contents in a common HTML document template.
///
/// Note that this function will wrap the passed body node like this:
///
/// <body>
/// <nav>...</nav>
/// <your body></your body>
/// </body>
///
/// So don't actually pass a `Body`.
pub fn html_doc<S: AsRef<str>>(
    css: Option<Vec<S>>,
    script: Option<Vec<S>>,
    script_inline: Option<Vec<S>>,
    body: Node,
) -> Node {
    // Use html-strong's base head template.
    let mut head = template::head();

    // Add stylesheets.
    if let Some(css) = css {
        for css in css {
            head.push_kid(Link::stylesheet("text/css", css.as_ref()));
        }
    }

    // Let's generally use the same font everywhere.
    head.push_kid(Link::stylesheet(
        "text/css",
        "https://fonts.googleapis.com/css2?family=Domine:wght@500&display=swap",
    ));
    // Always want the "base CSS" used for the top nav.
    head.push_kid(Link::stylesheet("text/css", &ContentUrl::base_css()));

    // Add scripts.
    if let Some(script) = script {
        for script in script {
            head.push_kid(Script::src(script.as_ref()));
        }
    }

    // Add scripts where content is defined inline.
    if let Some(script) = script_inline {
        for script in script {
            head.push_kid(Script::new().text(script));
        }
    }

    let nav = Nav
        .kid(A::href(&ContentUrl::new(Base::Home).url()).text("Home 🏠"))
        .kid(A::href(&ContentUrl::new(Base::Blog).url()).text("Blog 📚"))
        .kid(A::href(&ContentUrl::new(Base::Bus).url()).text("Bus 🚍"))
        .kid(A::href(&ContentUrl::new(Base::Blender).url()).text("Blender ⛰️"));

    let footer = Footer.kid(
        Div.kid(P.text("Made using "))
            .kid(A::href("https://github.com/torsteingrindvik/html-strong").text("html-strong")),
    );

    let body = Body
        .kid(nav.class("base-nav"))
        .kid(body.class("base-body"))
        .kid(footer.class("base-footer"));

    template::HtmlDocumentBuilder::new()
        .with_head(head)
        .with_body(body)
        .build()
}
