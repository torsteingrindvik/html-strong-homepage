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
    Training,
}

impl Display for Base {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Base::Home => "home",
            Base::Blender => "blender",
            Base::Shared => "shared",
            Base::Blog => "blog",
            Base::Bus => "bus",
            Base::Training => "training",
        };

        write!(f, "{s}")
    }
}

#[derive(Debug)]

pub struct ContentUrl {
    base: Base,
    children: Vec<String>,
}

impl ContentUrl {
    pub fn new(base: Base) -> Self {
        Self {
            base,
            children: vec![],
        }
    }

    /// Make this url go deeper by the given sub path.
    ///
    /// If a previous call to [url] would return a string "/blog",
    /// after calling `dive("into_someplace"), the next [url] call would
    /// then return "/blog/into_someplace" instead.
    pub fn dive(&mut self, sub: &str) {
        self.children.push(sub.into());
    }

    /// Get the url to this content.
    pub fn url(&self) -> String {
        let mut base = match &self.base {
            Base::Home => "/".into(),
            others => format!("/{others}"),
        };

        for child in &self.children {
            base += &format!("/{child}");
        }

        base
    }

    /// Get the url to this content, appending an extra url fragment at the end.
    /// Removes a leading slash if it contains it.
    pub fn suburl(&self, url: &str) -> String {
        let url = if url.starts_with('/') {
            &url[1..]
        } else {
            &url[..]
        };

        format!("{}/{url}", self.url())
    }

    /// Get image path.
    pub fn image(&self, image_name: &str) -> String {
        format!("/static/img/{}/{image_name}", self.base)
    }

    /// Get CSS path.
    pub fn css(&self, css_name: &str) -> String {
        format!("/static/css/{}/{css_name}", self.base)
    }

    /// Get shared base CSS.
    pub fn base_css() -> String {
        Self::new(Base::Shared).css("shared.css")
    }
}

pub mod blender;
pub mod blog;
pub mod bus;
pub mod home;
pub mod training;
