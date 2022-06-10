use html_strong::{science_lab::NodeExt, document_tree::Node, tags::*};

#[derive(Debug)]
pub enum Tidbit {
    Text(String),
    Url { url: String, text: String },
    Image(String),
    Code(String),
    Shell(String),
}

#[derive(Debug)]
pub struct Paragraph {
    tidbits: Vec<Tidbit>,
}

impl Paragraph {
    pub fn new() -> Self {
        Self { tidbits: vec![] }
    }

    fn push(&mut self, tidbit: Tidbit) {
        self.tidbits.push(tidbit);
    }
}

#[derive(Debug)]
struct Section {
    title: String,
    paragraphs: Vec<Paragraph>,
}

impl Section {
    fn new(title: &'static str) -> Self {
        Self {
            title: title.into(),
            paragraphs: vec![],
        }
    }
}

impl NodeExt for Section {
    fn into_node(self) -> Node {
        let mut output = Div.class("component-section");

        // We always start out a section with some title.
        output.push_kid(H2.text(self.title));

        for paragraph in self.paragraphs {
            let mut to_render: Option<Node> = None;

            for tidbit in paragraph.tidbits {
                let to_add: Node = match tidbit {
                    Tidbit::Url { url, text } => A::href(&url).text(text),
                    Tidbit::Image(path) => Img::new(&path).into_node(),
                    Tidbit::Code(code) => Code.class("component-code rounded").text(code).into_node(),
                    Tidbit::Shell(command) => {
                        Code.class("component-shell rounded").text(command).into_node()
                    }
                    Tidbit::Text(text) => {
                        to_render = Some(to_render.get_or_insert(P.into_node()).text(text));

                        continue;
                    }
                };

                to_render.as_mut().unwrap().push_kid(to_add);
            }

            output.push_kid(to_render.unwrap());
        }

        output
    }
}

#[derive(Debug)]
pub struct Article {
    sections: Vec<Section>,
}

impl Article {
    pub fn new() -> Self {
        Self { sections: vec![] }
    }

    fn add_tidbit(mut self, tidbit: Tidbit) -> Self {
        self.sections
            .last_mut()
            .unwrap()
            .paragraphs
            .last_mut()
            .unwrap()
            .push(tidbit);
        self
    }

    /// This starts a new paragraph with the given text.
    pub fn p(mut self, text: &'static str) -> Self {
        self.sections
            .last_mut()
            .unwrap()
            .paragraphs
            .push(Paragraph::new());
        self.text(text)
    }

    /// This adds more text to an ongoing paragraph.
    pub fn text(self, text: &'static str) -> Self {
        self.add_tidbit(Tidbit::Text(text.into()))
    }

    pub fn code(self, text: &'static str) -> Self {
        self.add_tidbit(Tidbit::Code(text.into()))
    }

    pub fn image(self, text: &'static str) -> Self {
        self.add_tidbit(Tidbit::Image(text.into()))
    }

    /// This adds an inline shell-like command to an ongoing paragraph.
    pub fn shell(self, text: &'static str) -> Self {
        self.add_tidbit(Tidbit::Shell(html_escape::encode_text(text).to_string()))
    }

    /// This adds an inline url to an ongoing paragraph.
    pub fn url(self, url: &'static str, text: &'static str) -> Self {
        self.add_tidbit(Tidbit::Url {
            url: url.into(),
            text: text.into(),
        })
    }

    /// This starts a new section with the given header.
    pub fn header(mut self, text: &'static str) -> Self {
        self.sections.push(Section::new(text));
        self
    }
}

impl NodeExt for Article {
    fn into_node(self) -> Node {
        let mut output = Div.class("component-article");

        for section in self.sections {
            output.push_kid(Div.class("breather-y").kid(section));
        }

        output
    }
}