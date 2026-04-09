use std::time::Duration;

use iced::futures::{Stream, sink::SinkExt};
use iced::widget::{container, space};
use iced::{Subscription, widget::progress_bar};
#[allow(unused_imports)]
use iced::{
    Element, Length::*, Task, widget::{button, center, column, text}
};

use crate::{
    lyrics_sources::{LrcLibLyricsSource, LyricsSource},
    media_sources::{MediaSource, WindowsMediaSource},
    models::{Lyrics, PlaybackStatus, Track}
};

#[derive(Debug, Default)]
enum LyricsState {
    #[default]
    None,
    Loading,
    Loaded(Lyrics),
    Error(String),
}

#[derive(Debug, Default)]
pub struct LyricsScreen {
    current_track: Option<Track>,
    playback_status: PlaybackStatus,
    track_position: Duration,
    lyrics_state: LyricsState,
}

#[derive(Debug, Clone)]
pub enum Message {
    LyricsFetched(Lyrics),
    LyricsFetchError(String),

    TrackChanged(Track),
    PositionChanged(Duration),
    PlaybackStatusChanged(PlaybackStatus),
    MediaError(String)
}

impl LyricsScreen {
    pub fn view(&self) -> Element<'_, Message> {
        let progress = progress_bar(0.0..=300.0, self.track_position.as_secs_f32());

        column![
            self.header(),
            self.body(),
            progress,
        ]
        .into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::LyricsFetched(lyrics) => {
                self.lyrics_state = LyricsState::Loaded(lyrics);
                Task::none()
            }
            Message::LyricsFetchError(error) => {
                self.lyrics_state = LyricsState::Error(error);
                Task::none()
            }
            Message::TrackChanged(track) => {
                self.current_track = Some(track.clone());
                self.lyrics_state = LyricsState::Loading;
                self.fetch_lyrics(track)
            }
            Message::PositionChanged(duration) => {
                self.track_position = duration;
                Task::none()
            }
            Message::PlaybackStatusChanged(playback_status) => {
                self.playback_status = playback_status;
                Task::none()
            }
            Message::MediaError(error) => {
                self.lyrics_state = LyricsState::Error(error);
                Task::none()
            },
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::run(Self::media_worker)
    }

    fn fetch_lyrics(&self, track: Track) -> Task<Message> {
        Task::perform(
            async {
                let source = LrcLibLyricsSource::new();
                source.fetch_lyrics(track).await
            },
            |result| match result {
                Ok(lyrics) => Message::LyricsFetched(lyrics),
                Err(e) => Message::LyricsFetchError(e.to_string()),
            }
        )
    }

    fn media_worker() -> impl Stream<Item = Message> {
        iced::stream::channel(32, async |mut tx| {
            let mut media = match WindowsMediaSource::new().await {
                Ok(m) => m,
                Err(e) => {
                    tx.send(Message::MediaError(e.to_string())).await.ok();
                    return;
                }
            };

            let mut last_track = Track::default();
            let mut last_position = Duration::ZERO;
            let mut last_status = PlaybackStatus::default();

            loop {
                if let Err(e) = media.refresh().await {
                    tracing::warn!("failed to refresh media session: {e}");
                }

                if let Ok(track) = media.current_track().await {
                    if track != last_track {
                        last_track = track.clone();
                        tx.send(Message::TrackChanged(track)).await.ok();
                    }
                }

                if let Ok(position) = media.current_playback_position().await {
                    if position != last_position {
                        last_position = position;
                        tx.send(Message::PositionChanged(position)).await.ok();
                    }
                }

                if let Ok(status) = media.current_playback_status().await {
                    if status != last_status {
                        last_status = status;
                        tx.send(Message::PlaybackStatusChanged(status)).await.ok();
                    }
                }

                tokio::time::sleep(Duration::from_millis(50)).await;
            }
        })
    }
}

impl LyricsScreen {
    fn header(&self) -> Element<'_, Message> {
        let title = match &self.current_track {
            Some(track) => text(format!("{} — {}", track.artist, track.title))
                .size(16),
            None => text("No track playing")
                .size(16),
        };

        container(title)
            .width(Fill)
            .padding([12, 24])
            .style(|theme: &iced::Theme| container::Style {
                border: iced::Border {
                    width: 1.0,
                    color: theme.extended_palette().background.strong.color,
                    ..Default::default()
                },
                ..Default::default()
            })
            .into()
    }

    fn body(&self) -> Element<'_, Message> {
        let content: Element<'_, Message> = match &self.lyrics_state {
            LyricsState::None => space().into(),
            LyricsState::Loading => text("Fetching lyrics...").into(),
            LyricsState::Error(e) => text(e.as_str()).into(),
            LyricsState::Loaded(lyrics) => {
                let lines = lyrics.lines.iter().map(|line| text(line.as_str()).into());
                column(lines).spacing(4).into()
            }
        };

        center(content)
            .width(Fill)
            .height(Fill)
            .into()
    }
}