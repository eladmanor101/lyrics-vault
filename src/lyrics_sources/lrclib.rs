use std::{sync::OnceLock, time::Duration};

use async_trait::async_trait;
use serde::Deserialize;

use regex::Regex;

use crate::{lyrics_sources::{LyricsSource, LyricsError}, models::{Lyrics, Track}};

static METADATA_RE: OnceLock<Regex> = OnceLock::new();
static TIMESTAMP_RE: OnceLock<Regex> = OnceLock::new();

pub struct LrcLibLyricsSource {
    client: reqwest::Client
}

#[async_trait]
impl LyricsSource for LrcLibLyricsSource {
    async fn fetch_lyrics(&self, track: Track) -> Result<Lyrics, LyricsError> {
        tracing::info!("fetching lyrics with url https://lrclib.net/api/get?artist_name={}&track_name={}", track.artist, track.title);

        let response = self.client
            .get("https://lrclib.net/api/get")
            .query(&[("artist_name", &*track.artist), ("track_name", &*track.title)])
            .send()
            .await?
            .error_for_status()?;

        let lyrics_data = response
            .json::<LrcResponse>()
            .await?;

        let duration = Duration::from_secs_f64(lyrics_data.duration);

        if let Some(raw_lyrics) = lyrics_data.synced_lyrics {
            Ok(self.process_lyrics(&raw_lyrics, track, duration, true))
        } else if let Some(raw_lyrics) = lyrics_data.plain_lyrics {
            tracing::warn!("synced lyrics not found for track {}, fetching plain lyrics instead", track);
            Ok(self.process_lyrics(&raw_lyrics, track, duration, false))
        } else {
            Err(LyricsError::NotFound(track))
        }
    }
}

impl LrcLibLyricsSource {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new()
        }
    }

    fn process_lyrics(&self, raw: &str, track: Track, duration: Duration, is_sync: bool) -> Lyrics {
        let metadata_re = get_metadata_re();

        if is_sync {
            let timestamp_re = get_timestamp_re();

            let mut parsed = raw.lines()
                .filter_map(|line| {
                    let line = line.trim();
                    if metadata_re.is_match(line) { return None; }

                    let caps = timestamp_re.captures(line)?;
                    let mins: u64 = caps["min"].parse().ok()?;
                    let secs: u64 = caps["sec"].parse().ok()?;
                    let ms: u64 = caps["ms"].parse().ok()?;
                    let position = Duration::from_mins(mins) + Duration::from_secs(secs) + Duration::from_millis(ms * 10);

                    let whole_match = caps.get(0).unwrap();
                    let text = kakasi::convert(line[whole_match.end()..].trim()).romaji;

                    Some((position, text))
                })
                .peekable();

            let intro = parsed
                .peek()
                .map_or(true, |(t, _)| *t > Duration::ZERO)
                .then(|| (Duration::ZERO, String::from("...")));

            let (timestamps, lines): (Vec<_>, Vec<_>) = intro.into_iter().chain(parsed).unzip();

            Lyrics {
                track,
                duration,
                lines: lines.into_boxed_slice(),
                timestamps: Some(timestamps.into_boxed_slice()),
            }
        } else {
            let lines = raw.lines()
                .filter_map(|line| {
                    let trimmed = line.trim();
                    if metadata_re.is_match(trimmed) { return None; }
                    Some(kakasi::convert(trimmed).romaji)
                })
                .collect::<Vec<_>>()
                .into_boxed_slice();

            Lyrics {
                track,
                duration,
                lines,
                timestamps: None
            }
        }
    }
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LrcResponse {
    id: u32,
    track_name: String,
    artist_name: String,
    album_name: String,
    duration: f64,
    instrumental: bool,
    plain_lyrics: Option<String>,
    synced_lyrics: Option<String>,
}

fn get_metadata_re() -> &'static Regex {
    METADATA_RE.get_or_init(|| Regex::new(r"^\[.*[a-zA-Z].*\]$").unwrap())
}

fn get_timestamp_re() -> &'static Regex {
    TIMESTAMP_RE.get_or_init(|| Regex::new(r"^\[(?<min>\d{2}):(?<sec>\d{2})\.(?<ms>\d{2,3})\]").unwrap())
}