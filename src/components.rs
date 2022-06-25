use html_strong::{document_tree::Node, science_lab::NodeExt, tags::*};

#[derive(Debug, Clone)]
pub enum Tidbit {
    Text(String),
    Url { url: String, text: String },
    Image(String),
    Code(String),
    Shell(String),
    Youtube(String),
    Section(String),
    List(Vec<String>),
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
        format!(
            "{}/{url}",
            self.url_prefix
                .as_ref()
                .expect("should set a url_prefix before resolving absolute path")
        )
    }

    fn add_tidbit(mut self, tidbit: Tidbit) -> Self {
        self.stuff.push(tidbit);
        self
    }

    /// Add paragraph text.
    pub fn p(self, text: &'static str) -> Self {
        self.add_tidbit(Tidbit::Text(text.into()))
    }

    /// Adds code.
    /// Is displayed in its own area.
    pub fn code(self, text: &'static str) -> Self {
        self.add_tidbit(Tidbit::Code(text.into()))
    }

    /// Adds an image.
    /// Is displayed in its own area.
    pub fn image(self, path: &'static str) -> Self {
        self.add_tidbit(Tidbit::Image(path.into()))
    }

    /// This adds an inline shell-like command.
    pub fn shell(self, command: &'static str) -> Self {
        self.add_tidbit(Tidbit::Shell(html_escape::encode_text(command).to_string()))
    }

    /// This adds an inline url.
    pub fn url(self, url: &'static str, text: &'static str) -> Self {
        self.add_tidbit(Tidbit::Url {
            url: url.into(),
            text: text.into(),
        })
    }

    /// Add a new header.
    pub fn header(self, text: &'static str) -> Self {
        self.add_tidbit(Tidbit::Section(text.into()))
    }

    /// Adds a new YouTube video under the current section.
    pub fn youtube(self, _url: &'static str) -> Self {
        unimplemented!(
            "Youtube embeds bring SO much garbage into the site load so screw that"
        );
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
                Tidbit::Url { url, text } => {
                    output.continue_paragraph(ParagraphContent::kid(
                        A::href(url).text(text),
                    ));
                }

                Tidbit::Image(path) => {
                    output.add_standalone(
                        Img::new(&self.absolute_path(&path))
                            .class("rounded breather-y center width-article-image"),
                    );
                }
                Tidbit::Code(code) => {
                    output.add_standalone(Code.class("component-code rounded").text(code));
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
                    output.add_standalone(Iframe::new(&url));
                }
                Tidbit::Section(title) => output.add_standalone(H2.text(&title)),
                Tidbit::List(text_list) => {
                    let mut list = Ul.into_node();
                    for text in text_list {
                        list.push_kid(Li.text(text));
                    }
                    output.add_standalone(list)
                }
            };

            // If we know if the last tidbit was a paragraph/text,
            // we can handle line breaks in a smarter way.
            output.last_was_p = matches!(tidbit, Tidbit::Text(_));
        }
        output.finish_up();

        output.output
    }
}
