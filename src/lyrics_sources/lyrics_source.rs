use async_trait::async_trait;
use thiserror::Error;

use crate::models::{Lyrics, Track};

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum LyricsError {
    #[error("lyrics not found for track {0}")]
    NotFound(Track),

    #[error("unknown network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("failed to parse the source's response")]
    ParseError
}

#[async_trait]
pub trait LyricsSource {
    async fn fetch_lyrics(&self, track: Track) -> Result<Lyrics, LyricsError>;
}