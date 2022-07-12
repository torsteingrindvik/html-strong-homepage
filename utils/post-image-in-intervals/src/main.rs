use std::{collections::VecDeque, time::Duration};

use html_strong_homepage::{
    herbs::{herbs_new_image_auth, HERBS_NEW_IMAGE_POST_ENDPOINT},
    image::Image,
};
use reqwest::StatusCode;
use tracing::{info, warn};

const SECONDS_INTERVAL: u64 = 60 * 5;

#[cfg(feature = "use-webcam")]
fn impl_produce_image() -> Image {
    unimplemented!("TODO this one")
}

#[cfg(not(feature = "use-webcam"))]
fn impl_produce_image() -> Image {
    let example = include_bytes!("../example.jpeg");
    Image::new(example)
}

fn produce_image() -> Image {
    impl_produce_image()
}

struct Postman {
    // If we fail to post to the endpoint,
    // put it in the backlog, and try later.
    backlog: VecDeque<Image>,

    client: reqwest::blocking::Client,
}

impl Postman {
    fn new() -> Self {
        Self {
            backlog: VecDeque::new(),
            client: reqwest::blocking::Client::new(),
        }
    }

    fn send(&self, image: &Image) -> reqwest::Result<reqwest::blocking::Response> {
        let endpoint = HERBS_NEW_IMAGE_POST_ENDPOINT;
        let endpoint = format!("http://localhost:8000{endpoint}");

        self.client
            .post(endpoint)
            .bearer_auth(herbs_new_image_auth())
            .json(image)
            .send()
    }

    fn bad_reply(&mut self, image: Image) {
        info!("Putting image in backlog");

        // Back to FRONT here to preserve order.
        self.backlog.push_front(image);
    }

    fn post_image(&mut self, image: Image) {
        self.backlog.push_back(image);

        while let Some(backlog_image) = self.backlog.pop_front() {
            info!("Sending {backlog_image:?}");
            match self.send(&backlog_image) {
                Ok(response) if response.status() == StatusCode::OK => {
                    info!("Post image successful response: {response:?}")
                }
                Ok(response) => {
                    warn!("Post image bad response: {response:?}");

                    let text = response.text();
                    info!("Text: {text:?}");

                    self.bad_reply(backlog_image);

                    // Retry later...
                    return;
                }
                Err(error) => {
                    warn!("Post image error: {error:?}");

                    self.bad_reply(backlog_image);

                    // Retry later...
                    return;
                }
            }
        }
    }
}

fn main() {
    tracing_subscriber::fmt::init();

    info!("Producing and posting images forever!");

    let mut postman = Postman::new();

    loop {
        let image = produce_image();
        postman.post_image(image);

        std::thread::sleep(Duration::from_secs(SECONDS_INTERVAL));
    }
}
