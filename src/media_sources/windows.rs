use std::time::Duration;

use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSession as WMediaSession,
    GlobalSystemMediaTransportControlsSessionManager as WSessionManager,
};

use crate::{
    media_sources::{MediaSource, media_source::MediaError},
    models::{PlaybackStatus, Track},
};

pub struct WindowsMediaSource {
    session_manager: WSessionManager,
    spotify_session: Option<WMediaSession>,
}

impl WindowsMediaSource {
    pub async fn new() -> Result<Self, MediaError> {
        let session_manager = WSessionManager::RequestAsync()?.await?;
        let mut this = Self { session_manager, spotify_session: None };
        this.refresh().await?;
        Ok(this)
    }

    fn extract_session(&self) -> Result<&WMediaSession, MediaError> {
        self.spotify_session.as_ref().ok_or(MediaError::NoSession)
    }
}

impl MediaSource for WindowsMediaSource {
    async fn refresh(&mut self) -> Result<(), MediaError> {
        self.spotify_session = self.session_manager
            .GetSessions()?
            .into_iter()
            .find(|s| {
                match s.SourceAppUserModelId() {
                    Ok(s) => s.to_string().contains("Spotify"),
                    Err(_) => false,
                }
            });

        Ok(())
    }

    async fn current_track(&self) -> Result<Track, MediaError> {
        let session = self.extract_session()?;

        let media_properties = session
            .TryGetMediaPropertiesAsync()
            .map_err(|_| MediaError::TrackInfoUnavailable)?
            .await
            .map_err(|_| MediaError::TrackInfoUnavailable)?;

        Ok(Track::new(
            &media_properties.Artist().map_err(|_| MediaError::TrackInfoUnavailable)?.to_string(),
            &media_properties.Title().map_err(|_| MediaError::TrackInfoUnavailable)?.to_string(),
        ))
    }

    async fn current_playback_position(&self) -> Result<Duration, MediaError> {
        let session = self.extract_session()?;

        let timeline = session
            .GetTimelineProperties()
            .map_err(|_| MediaError::PositionUnavailable)?;

        Ok(Duration::from_millis(
            (timeline.Position().map_err(|_| MediaError::PositionUnavailable)?.Duration / 10_000) as u64,
        ))
    }

    async fn current_playback_status(&self) -> Result<PlaybackStatus, MediaError> {
        let session = self.extract_session()?;

        Ok(PlaybackStatus::from(
            session.GetPlaybackInfo()
                .map_err(|_| MediaError::StatusUnavailable)?
                .PlaybackStatus()
                .map_err(|_| MediaError::StatusUnavailable)?,
        ))
    }
}