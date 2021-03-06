use std::str::Lines;

use html_strong::{document_tree::Node, science_lab::NodeExt, tags::*};
use tracing::debug;

#[derive(Debug)]
pub struct Source {
    text: String,
    file: String,
}

#[derive(Debug)]
pub enum ListingId {
    Num(usize),
    Text(String),
}

impl Into<ListingId> for usize {
    fn into(self) -> ListingId {
        ListingId::Num(self)
    }
}
impl Into<ListingId> for &str {
    fn into(self) -> ListingId {
        ListingId::Text(self.into())
    }
}

#[derive(Debug, Clone)]
pub struct Listing {
    file: String,
    code: String,
    name: String,
    start: usize,
    end: usize,
}

impl NodeExt for Listing {
    fn into_node(self) -> Node {
        let title = Div
            .class("text-center")
            .kid(Em.text(format!("\"{}\"", self.name)));

        let link = A::href(&format!(
            "https://github.com/torsteingrindvik/html-strong-homepage/blob/main/{}#L{}-L{}",
            self.file, self.start, self.end
        ))
        .with_target(a::Target::Blank)
        .class("code-link")
        .kid(Img::new_sized("/static/gh_light.webp", 24, 24).class("ease"));

        let code = Div
            .style("position: relative;")
            .kid(
                Pre.kid(
                    Code.class("language-rust")
                        .text(html_escape::encode_text(&self.code)),
                ),
            )
            .kid(link);

        let subtitle = Div.class("text-center").kid(Em.text(format!(
            "Source: {}, lines {}-{}",
            self.file, self.start, self.end
        )));

        Div.class("code-listing breather-y")
            .kid(title)
            .kid(code)
            .kid(subtitle)
    }
}

impl Source {
    pub fn new(source_path: &str) -> Self {
        debug!("Source: {source_path}");
        Self {
            // Technically this should be async.
            // We use [`Source`] when making content via [`Article`], and that API
            // is not async.
            // So that would be a hassle.
            // Also, let's be fair, the pref impact from doing a blocking read of a small text file
            // should (?) at this point be negligible.
            text: std::fs::read_to_string(source_path).expect("file should exist and be readable"),
            file: source_path.to_string(),
        }
    }

    fn lines(&self) -> Lines {
        self.text.lines()
    }

    fn listing_end() -> &'static str {
        "// ~listing"
    }

    fn find_listing_range(&self, listing_id: &str) -> Listing {
        let listing_start = self
            .lines()
            .position(|line| line.contains(listing_id))
            .expect("referenced listing should be found in code");

        // An example would be
        //
        //  listing: main
        //
        // We want to get the index (right before the colon) and name
        // (the stuff after).
        let (_, listing_name) = self
            .lines()
            .nth(listing_start)
            .unwrap()
            .split_once(": ")
            .expect("listing should have `: ` in it");

        // Now skip the line with the listing opener itself.
        let listing_start = listing_start + 1;

        let listing_size = self
            .lines()
            .skip(listing_start)
            .position(|line| line.contains(Source::listing_end()))
            .expect("referenced listing should have an end");

        Listing {
            file: self.file.clone(),

            // Skip the line with the listing opener.
            code: self
                .lines()
                .skip(listing_start)
                .take(listing_size)
                .collect::<Vec<&str>>()
                .join("\n"),

            name: listing_name.to_string(),

            // Human readable line number means we don't want it 0-indexed, so add one.
            start: listing_start + 1,
            end: listing_start + listing_size,
        }
    }

    pub fn listing(&self, listing_id: impl Into<ListingId>) -> Listing {
        let listing_id: ListingId = listing_id.into();

        match listing_id {
            ListingId::Num(num) => self.find_listing_range(&format!("// listing {num}")),
            ListingId::Text(text) => self.find_listing_range(&text),
        }
    }
}
