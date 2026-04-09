use std::time::Duration;

use crate::models::{PlaybackStatus, Track};

#[derive(Debug, thiserror::Error)]
pub enum MediaError {
    #[error("no active media session")]
    NoSession,

    #[error("failed to read track info")]
    TrackInfoUnavailable,

    #[error("failed to read playback position")]
    PositionUnavailable,

    #[error("failed to read playback status")]
    StatusUnavailable,

    #[error("windows api error: {0}")]
    Windows(#[from] windows::core::Error),
}

pub trait MediaSource {
    async fn refresh(&mut self) -> Result<(), MediaError>;

    async fn current_track(&self) -> Result<Track, MediaError>;
    async fn current_playback_position(&self) -> Result<Duration, MediaError>;
    async fn current_playback_status(&self) -> Result<PlaybackStatus, MediaError>;
}