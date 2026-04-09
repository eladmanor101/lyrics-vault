use std::time::Duration;

use crate::{media_sources::{MediaSource, media_source::MediaError}, models::{PlaybackStatus, Track}};

#[allow(unused)]
pub struct SpotifyMediaSource;

impl MediaSource for SpotifyMediaSource {
    async fn refresh(&mut self) -> Result<(), MediaError> {
        Ok(())
    }

    async fn current_track(&self) -> Result<Track, MediaError> {
        todo!()
    }

    async fn current_playback_position(&self) -> Result<Duration, MediaError> {
        todo!()
    }

    async fn current_playback_status(&self) -> Result<PlaybackStatus, MediaError> {
        todo!()
    }
}