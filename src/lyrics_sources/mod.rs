mod lyrics_source;
mod lrclib;

pub use lyrics_source::{LyricsSource, LyricsError};
pub use lrclib::LrcLibLyricsSource;

use crate::models::Track;

const ARTIST_ALIASES: &[&[&str]] = &[
    &["Yorushika", "ヨルシカ"],
    &["ZUTOMAYO", "ずっと真夜中でいいのに。"],
];

pub fn track_aliases(track: Track) -> Vec<Track> {
    let mut candidates = vec![track.clone()];

    if let Some(group) = ARTIST_ALIASES.iter().find(|g| {
        g.iter().any(|&name| name.to_lowercase() == track.artist.to_lowercase())
    }) {
        for &name in *group {
            if name.to_lowercase() != track.artist.to_lowercase() {
                candidates.push(Track::new(name, &track.title));
            }
        }
    }

    candidates
}