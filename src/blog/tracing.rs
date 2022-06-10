use axum::Extension;
use axum::{response::Html, routing::get, Router};
use html_strong::{science_lab::NodeExt, tags::*};
use reqwest::StatusCode;

use crate::{Base, ContentUrl, components};

use super::{blog_page, BlogEntry, Series, State};

enum Posts {
    Overview,
}

impl Posts {
    fn url(&self) -> &'static str {
        match self {
            Posts::Overview => "/overview",
        }
    }

    fn title(&self) -> &'static str {
        match self {
            Posts::Overview => "2022-06-04: Overview",
        }
    }

    // TODO: English name?
    fn leder(&self) -> &'static str {
        match self {
            Posts::Overview => {
                "Hello world/tracing! Let's get an overview of what tracing is and why we'd want \
                 to use it."
            }
        }
    }
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(landing))
        .route(Posts::Overview.url(), get(overview))
}


fn overview_impl() -> impl NodeExt {
    components::Article::new()
        .header("Tracing: An overview")
        .p("Tokio ")
        .url("https://github.com/tokio-rs/tracing", "tracing")
        .text(
            " looks really neat. I've sporadically used it for various small projects, \
             with various degrees of success. Most times I just use it to get the various \
             standard log macros, and rely on ",
        )
        .shell("RUST_LOG=<some log level>")
        .text(" to make things show up in my terminal.")
        .p("But there seems to be a lot more to the story!")
        .p("Let's lay out some goals if what to figure out. Note that at the time of writing, I don't know the answers, and as such the goals might not be accurate until I know enough to refine them.")
        .header("Goals")
        .p("There are several goals I'd like to achieve by investigating tracing. The goals can be split into a few categories, so let's do that.")
        .header("Goal: Understanding tracing itself")
        .p("I'd like to understand the concepts better.")
        .p("* What is a subscriber?")
        .p("* Can I use many subscribers at the same time? Should I?")
        .p("* How does writing my own subscriber look like?")
        .p("* What is an event?")
        .p("* What is a span?")
        .p("* How can the act of entering a span impact code in the same scope? In other words, how can one part of code introduce spooky state on following code?")
        .p("* Why do we have to be careful with spans when using async?")
        .p("* What does the instrument attribute do, and what does the instrument call on a future do?")
        .p("* How is global state managed? Specifically, how does logging interact with a global subscriber?")
        .header("Practical goal: Making distributed traces")
        .p("I want to know how I can make an application where I have some servers with clients talking to each server, and end up with traces which are scoped in such a way that it's easy to see what happened during each connection.")
        .p("Also, let's say we have 10 000 of the above sessions. Can I have little/no verbosity for happy connections, and higher verbosity if an error occurs in misbehaving connections?")
        .header("Goal: The relationship with OpenTelemetry")
        .p("For having this neat overview of distributed traces, I think ")
        .url( "https://opentelemetry.io/docs/", "OpenTelemetry")
        .text(" is a good way to go about things. There seems to be some nice integrations between tracing and OpenTelemetry. But OpenTelemetry too has concepts which I know little of. So...")
        .p("* How does OpenTelemetry work?")
        .p("* How well does it map to tracing?")
        .p("* Which collectors should I use?")
        .p("* Is a collector what I think it is: The thing which receives traces? Is it very close to being a (tracing) subscriber?")
}

async fn overview() -> Result<Html<String>, (StatusCode, String)> {
    blog_page(overview_impl().into_node()).await
}

pub async fn landing(
    Extension(state): Extension<State>,
) -> Result<Html<String>, (StatusCode, String)> {
    let mut url = ContentUrl::new(Base::Blog);
    url.dive("tracing");

    let tracing_post = Posts::Overview;
    let tracing = BlogEntry::new(
        &url.suburl(tracing_post.url()),
        super::EntryState {
            name: tracing_post.title().into(),
            description: tracing_post.leder().into(),
        },
    );

    let state = state.get(&Series::Tracing).unwrap().clone();
    let contents = Div
        .kid(H1.text(state.name))
        .kid(P.text(state.description))
        .kid(Br)
        .kid(Div.class("breather-y").kid(tracing));

    blog_page(contents).await
}
