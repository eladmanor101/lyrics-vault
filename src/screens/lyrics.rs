use std::time::Duration;

use iced::{
    Element, Length::*, Subscription, Task,
    futures::{Stream, sink::SinkExt},
    widget::{center, column, container, progress_bar, row, scrollable, space, text},
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
    pub fn view(&self, theme: iced::Theme) -> Element<'_, Message> {
        column![
            self.header(&theme),
            self.body(&theme),
            self.footer(&theme)
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
    fn header(&self, theme: &iced::Theme) -> Element<'_, Message> {
        let palette = theme.extended_palette();

        let title: Element<_> = match &self.current_track {
            Some(track) => row![
                text(track.artist.as_ref())
                    .size(16)
                    .color(palette.background.base.text)
                    .font(iced::Font {
                        weight: iced::font::Weight::Semibold,
                        ..iced::Font::default()
                    }),
                text("  —  ")
                    .size(16)
                    .color(palette.background.weak.text),
                text(track.title.as_ref())
                    .size(16)
                    .font(iced::Font {
                        weight: iced::font::Weight::Bold,
                        ..iced::Font::default()
                    })
                    .color(palette.background.strong.text),
            ]
            .into(),
            None => text("No track playing")
                .size(14)
                .color(palette.background.strong.color)
                .into(),
        };

        container(center(title).height(Fill))
            .width(Fill)
            .height(48)
            .style(|theme: &iced::Theme| container::Style {
                border: iced::Border {
                    width: 0.0,
                    ..Default::default()
                },
                background: Some(iced::Background::Color(
                    theme.extended_palette().background.weakest.color
                )),
                ..Default::default()
            })
            .into()
    }

    fn body(&self, theme: &iced::Theme) -> Element<'_, Message> {
        let palette = theme.extended_palette();

        match &self.lyrics_state {
            LyricsState::None => {
                space().into()
            }

            LyricsState::Loading => {
                container(
                    text("Fetching lyrics...")
                        .color(palette.background.strong.color)
                )
                .width(Fill)
                .height(Fill)
                .center(Fill)
                .into()
            }

            LyricsState::Error(e) => {
                container(
                    text(e.as_str())
                        .color(palette.danger.base.color)
                )
                .width(Fill)
                .height(Fill)
                .center(Fill)
                .into()
            }

            LyricsState::Loaded(lyrics) => {
                let active = lyrics.timestamps
                    .as_deref()
                    .map(|t| self.active_line_index(t));

                let lines = lyrics.lines
                    .iter()
                    .enumerate()
                    .map(|(i, line)| {
                        let is_active = active == Some(i);

                        if is_active {
                            text(line.as_str())
                                .size(20)
                                .font(iced::Font {
                                    weight: iced::font::Weight::Bold,
                                    ..iced::Font::default()
                                })
                                .color(palette.background.strongest.text)
                                .width(Fill)
                                .center()
                                .wrapping(text::Wrapping::Word)
                                .into()
                        } else {
                            text(line.as_str())
                                .size(17)
                                .color(palette.background.strong.text)
                                .width(Fill)
                                .center()
                                .wrapping(text::Wrapping::Word)
                                .into()
                        }
                    });

                container(
                    scrollable(
                        row![
                            space().width(FillPortion(1)),
                            column(lines)
                                .spacing(10)
                                .padding([16, 0])
                                .width(FillPortion(3)),
                            space().width(FillPortion(1)),
                        ]
                    )
                    .width(Fill)
                    .height(Fill)
                )
                .width(Fill)
                .height(Fill)
                .padding([12, 0])
                .into()
            }
        }
    }

    fn active_line_index(&self, timestamps: &[Duration]) -> usize {
        timestamps
            .iter()
            .position(|t| *t > self.track_position)
            .map(|i| i.saturating_sub(1))
            .unwrap_or(timestamps.len().saturating_sub(1))
    }

    fn footer(&self, theme: &iced::Theme) -> Element<'_, Message> {
        let palette = theme.extended_palette();

        let (position, duration) = match &self.lyrics_state {
            LyricsState::Loaded(lyrics) => (self.track_position, lyrics.duration),
            _ => (Duration::ZERO, Duration::ZERO),
        };

        let format = |d: Duration| {
            let secs = d.as_secs();
            format!("{:02}:{:02}", secs / 60, secs % 60)
        };

        let remaining = duration.saturating_sub(position);

        let progress = container(
            progress_bar(
                0.0..=duration.as_secs_f32().max(1.0),
                position.as_secs_f32()
            )
        )
        .height(6)
        .width(Fill);

        let timestamps = row![
            text(format(position))
                .size(12)
                .color(palette.secondary.base.color),
            space().width(Fill),
            text(format!("-{}", format(remaining)))
                .size(12)
                .color(palette.secondary.base.color),
        ]
        .padding([0, 4]);

        container(
            column![
                progress,
                timestamps,
            ]
            .spacing(4)
        )
        .width(Fill)
        .padding([8, 12])
        .style(|theme: &iced::Theme| container::Style {
            border: iced::Border {
                width: 1.0,
                color: theme.extended_palette().background.strong.color,
                ..Default::default()
            },
            background: Some(iced::Background::Color(
                theme.extended_palette().background.weakest.color
            )),
            ..Default::default()
        })
        .into()
    }
}