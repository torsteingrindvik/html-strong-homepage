use html_strong::{document_tree::Node, science_lab::NodeExt, tags::*};
use tracing::error;

use crate::listing::Listing;

#[derive(Debug, Clone)]
pub enum Tidbit {
    Text(String),
    Url { url: String, text: String },
    Image(String),
    Code(Listing),
    CodeInline(String),
    Shell(String),
    Youtube(String),
    H2(String),
    H3(String),
    List(Vec<String>),
    Breather,
    Video(String),
    Sidenote(Article),
    Quote(Article),
}

#[derive(Debug, Clone)]
pub struct Article {
    stuff: Vec<Tidbit>,
    pub url_prefix: Option<String>,
}

impl Article {
    pub fn new() -> Self {
        Self {
            stuff: vec![],
            url_prefix: None,
        }
    }

    fn absolute_path(&self, url: &str) -> String {
        if let Some(absolute_prefix) = self.url_prefix.as_ref() {
            format!("{absolute_prefix}/{url}",)
        } else if !url.starts_with("/") {
            error!(?url, "bad url");
            panic!("Expected url to be absolute")
        } else {
            url.to_string()
        }
    }

    fn add_tidbit(mut self, tidbit: Tidbit) -> Self {
        self.stuff.push(tidbit);
        self
    }

    /// Add a visually distinct side note.
    /// Contents are arbitrary- therefore takes an article as well.
    pub fn quote(self, contents: Article) -> Self {
        self.add_tidbit(Tidbit::Quote(contents))
    }

    /// Add a visually distinct side note.
    /// Contents are arbitrary- therefore takes an article as well.
    pub fn sidenote(self, contents: Article) -> Self {
        self.add_tidbit(Tidbit::Sidenote(contents))
    }

    /// Add paragraph text.
    pub fn p(self, text: &str) -> Self {
        self.add_tidbit(Tidbit::Text(html_escape::encode_text(text).to_string()))
    }

    /// Add a br.
    pub fn br(self) -> Self {
        self.add_tidbit(Tidbit::Breather)
    }

    /// Adds code.
    /// Is displayed in its own area.
    pub fn code(self, listing: Listing) -> Self {
        self.add_tidbit(Tidbit::Code(listing))
    }

    /// Adds code.
    /// Inline- i.e. continues paragraph.
    pub fn code_inline(self, code: &str) -> Self {
        self.add_tidbit(Tidbit::CodeInline(code.into()))
    }

    /// Adds an image.
    /// Is displayed in its own area.
    pub fn image(self, path: &str) -> Self {
        self.add_tidbit(Tidbit::Image(path.into()))
    }

    /// Adds a video with loop and controls.
    /// Is displayed in its own area.
    pub fn video(self, path: &str) -> Self {
        self.add_tidbit(Tidbit::Video(path.to_string()))
    }

    /// This adds an inline shell-like command.
    pub fn shell(self, command: &str) -> Self {
        self.add_tidbit(Tidbit::Shell(html_escape::encode_text(command).to_string()))
    }

    /// This adds an inline url.
    pub fn url(self, url: &str, text: &str) -> Self {
        self.add_tidbit(Tidbit::Url {
            url: url.into(),
            text: text.into(),
        })
    }

    /// Add an h2 element.
    pub fn h2(self, text: &str) -> Self {
        self.add_tidbit(Tidbit::H2(text.into()))
    }

    /// Add an h3 element.
    pub fn h3(self, text: &str) -> Self {
        self.add_tidbit(Tidbit::H3(text.into()))
    }

    /// Adds a new YouTube video under the current section.
    pub fn youtube(self, _url: &'static str) -> Self {
        unimplemented!("Youtube embeds bring SO much garbage into the site load so screw that");
        // self.add_tidbit(Tidbit::Youtube(url.into()))
    }

    /// Adds a new unordered list of text items.
    pub fn list<S: AsRef<str>>(self, entries: Vec<S>) -> Self {
        self.add_tidbit(Tidbit::List(
            entries
                .into_iter()
                .map(|s| s.as_ref().to_string())
                .collect::<Vec<String>>(),
        ))
    }
}

impl Default for Article {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeExt for Article {
    fn into_node(self) -> Node {
        struct Output {
            last_was_p: bool,
            output: Node,
            ongoing: Option<Node>,
        }

        enum ParagraphContent {
            Text(String),
            Kid(Node),
        }

        impl ParagraphContent {
            fn text(text: String) -> Self {
                Self::Text(text)
            }
            fn kid(kid: impl NodeExt) -> Self {
                Self::Kid(kid.into_node())
            }
        }

        impl Output {
            fn new(output: Node) -> Self {
                Self {
                    last_was_p: false,
                    output,
                    ongoing: None,
                }
            }

            fn finish_up(&mut self) {
                if let Some(ongoing) = self.ongoing.take() {
                    self.output.push_kid(ongoing);
                }
            }

            // Get current node, or start a new paragraph if none is present.
            fn current_node(&mut self) -> Node {
                self.ongoing.take().unwrap_or_else(|| P.into_node())
            }

            fn add_standalone(&mut self, content: impl NodeExt) {
                self.finish_up();
                self.output.push_kid(content);
            }

            // Continues an ongoing paragraph, or adds to a new one if none was ongoing.
            fn continue_paragraph(&mut self, content: ParagraphContent) {
                self.ongoing = Some(match content {
                    ParagraphContent::Text(t) => self.current_node().add_text(&t),
                    ParagraphContent::Kid(k) => self.current_node().kid(k),
                });
            }
        }

        let mut output = Output::new(Div.class("component-article"));

        for tidbit in &self.stuff {
            match tidbit {
                Tidbit::Quote(article) => output.add_standalone(
                    Div.class("quote breather-y rounded")
                        .kid(Div.class("quote-mark").text("“"))
                        .kid(article.clone()),
                ),
                Tidbit::Sidenote(article) => {
                    output.add_standalone(article.clone().class("sidenote breather-y rounded"))
                }
                Tidbit::Url { url, text } => {
                    output.continue_paragraph(ParagraphContent::kid(A::href(url).text(text)));
                }

                Tidbit::Image(path) => {
                    output.add_standalone(
                        Img::new(&self.absolute_path(path))
                            .class("rounded breather-y center width-100"),
                    );
                }
                Tidbit::Video(path) => output.add_standalone({
                    let source = if path.ends_with("webm") {
                        Source::new_webm(self.absolute_path(path))
                    } else if path.ends_with("mp4") {
                        Source::new_mp4(self.absolute_path(path))
                    } else {
                        error!(?path, "unhandled video extension");
                        panic!()
                    };

                    Video::new()
                        .controls()
                        .kid(source)
                        .class("rounded breather-y width-100")
                }),
                Tidbit::CodeInline(code) => output.continue_paragraph(ParagraphContent::Kid(
                    Code.class("rust-inline rounded")
                        .text(html_escape::encode_text(code)),
                )),
                Tidbit::Code(code) => {
                    output.add_standalone(code.clone());
                }
                Tidbit::Shell(command) => {
                    output.continue_paragraph(ParagraphContent::kid(
                        Code.class("component-shell rounded").text(command),
                    ));
                }
                Tidbit::Text(text) => {
                    if output.last_was_p {
                        // Text twice in a row?
                        // Finish up that paragraph, as this likely means we want a line break.
                        // Else we would just add more text to the first call.
                        output.finish_up();
                    }
                    output.continue_paragraph(ParagraphContent::text(text.to_string()));
                }
                Tidbit::Youtube(url) => {
                    output.add_standalone(Iframe::new(url));
                }
                Tidbit::H2(title) => output.add_standalone(H2.text(&title)),
                Tidbit::H3(title) => output.add_standalone(H3.text(&title)),
                Tidbit::List(text_list) => {
                    let mut list = Ul.into_node();
                    for text in text_list {
                        list.push_kid(Li.text(text));
                    }
                    output.add_standalone(list.class("breather-y"))
                }
                Tidbit::Breather => output.add_standalone(Br),
            };

            // If we know if the last tidbit was a paragraph/text,
            // we can handle line breaks in a smarter way.
            output.last_was_p = matches!(tidbit, Tidbit::Text(_));
        }
        output.finish_up();

        output.output
    }
}
