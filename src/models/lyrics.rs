use std::time::Duration;

use crate::models::Track;

#[derive(Debug, Clone)]
pub struct Lyrics {
    pub track: Track,
    pub duration: Duration,
    pub lines: Box<[String]>,
    pub timestamps: Option<Box<[Duration]>>,
}

impl Lyrics {
    pub fn is_synced(&self) -> bool {
        self.timestamps.is_some()
    }
}