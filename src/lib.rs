use std::fmt::Display;

pub mod base;
pub mod common;

// pub mod base_urls {
//     pub const HOME: &str = "/";
//     pub const BLENDER: &str = "/blender";

//     // Append one of the base urls to one of these
//     // to get the path to more specific content.
//     // pub mod statics {
//     //     pub const CSS: &str = "/static/css";
//     //     pub const IMG: &str = "/static/img";
//     // }
// }

#[derive(Debug)]
pub enum Base {
    Shared,
    Home,
    Blog,
    Bus,
    Blender,
}

impl Display for Base {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Base::Home => "home",
            Base::Blender => "blender",
            Base::Shared => "shared",
            Base::Blog => "blog",
            Base::Bus => "bus",
        };

        write!(f, "{s}")
    }
}

#[derive(Debug)]

pub struct ContentUrl {
    base: Base,
    subpage: Option<String>,
}

impl ContentUrl {
    pub fn new(base: Base) -> Self {
        Self {
            base,
            subpage: None,
        }
    }

    pub fn new_with_subpage(base: Base, sub: &str) -> Self {
        Self {
            base,
            subpage: Some(sub.to_string()),
        }
    }

    pub fn url(&self) -> String {
        let base = match &self.base {
            Base::Home => "/".into(),
            others => format!("/{others}"),
        };

        if let Some(sub) = &self.subpage {
            format!("{base}/{sub}")
        } else {
            base
        }
    }

    pub fn suburl(&self, url: &str) -> String {
        format!("{}/{url}", self.url())
    }

    // Get image path.
    pub fn image(&self, image_name: &str) -> String {
        format!("/static/img/{}/{image_name}", self.base)
    }

    // Get CSS path.
    pub fn css(&self, css_name: &str) -> String {
        format!("/static/css/{}/{css_name}", self.base)
    }

    // Get shared base CSS.
    pub fn base_css() -> String {
        Self::new(Base::Shared).css("shared.css")
    }
}

pub mod blender;
pub mod blog;
pub mod bus;
pub mod home;
