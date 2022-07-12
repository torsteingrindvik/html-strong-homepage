use std::fmt::Debug;

use chrono::{DateTime, Utc, Local};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Image {
    pub buffer: Vec<u8>,
    pub timestamp: DateTime<Utc>,
}

impl Image {
    pub fn filename(&self) -> String {
        let local: DateTime<Local> = DateTime::from(self.timestamp);
        local.format("%F_%H-%M-%S").to_string()
    }
}

impl Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Image")
            .field("buffer", &format!("{} bytes", self.buffer.len()))
            .field("timestamp", &self.timestamp)
            .finish()
    }
}

impl Image {
    pub fn new(buffer: &[u8]) -> Self {
        Self {
            buffer: buffer.to_owned(),
            timestamp: Utc::now(),
        }
    }
}
