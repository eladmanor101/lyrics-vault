use async_trait::async_trait;
use thiserror::Error;

use crate::{lyrics_sources::track_aliases, models::{Lyrics, Track}};

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

    async fn lyrics(&self, track: Track) -> Result<Lyrics, LyricsError> {
        let candidates = track_aliases(track.clone());

        for candidate in candidates {
            if let Ok(lyrics) = self.fetch_lyrics(candidate).await {
                return Ok(lyrics);
            }
        }

        Err(LyricsError::NotFound(track))
    }
}