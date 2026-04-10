mod screens;
mod sidebar;
mod models;
mod lyrics_sources;
mod media_sources;
mod utils;

use std::time::Duration;

#[allow(unused_imports)]
use iced::{
    widget::{button, text},
    Length::*,
    Size,
    widget::{column, container, row}
};

use crate::screens::{
    lyrics, themes
};

fn main() -> iced::Result {
    let _guard = utils::setup_tracing();

    iced::application(App::default, App::update, App::view)
        .title("Iced App")
        .theme(App::theme)
        .window_size(Size::new(800.0, 800.0))
        .centered()
        .subscription(App::subscription)
        .run()
}

struct App {
    screen: Screen,
    theme: iced::Theme,

    sidebar: sidebar::Sidebar,
    lyrics_screen: lyrics::LyricsScreen,
    themes_screen: themes::ThemesScreen
}

impl Default for App {
    fn default() -> Self {
        Self {
            screen: Default::default(),
            theme: iced::Theme::Dark,
            sidebar: Default::default(),
            lyrics_screen: Default::default(),
            themes_screen: Default::default()
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Tick(std::time::Instant),

    SidebarMessage(sidebar::Message),
    LyricsMessage(lyrics::Message),
    ThemesMessage(themes::Message)
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
enum Screen {
    #[default]
    Lyrics,
    Themes
}

impl App {
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::Tick(instant) => {
                self.lyrics_screen.update(lyrics::Message::Tick(instant));
                iced::Task::none()
            }
            Message::SidebarMessage(message) => {
                match self.sidebar.update(message) {    
                    Some(action) => match action {
                        sidebar::Action::Navigate(screen) => self.screen = screen,
                    }
                    None => {}
                }
                iced::Task::none()
            },
            Message::LyricsMessage(message) => self.lyrics_screen.update(message).map(Message::LyricsMessage),
            Message::ThemesMessage(message) => {
                match self.themes_screen.update(message) {
                    themes::Action::ThemeChanged(theme) => {
                        self.theme = theme;
                    },
                }
                iced::Task::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        let screen_view = match self.screen {
            Screen::Lyrics => self.lyrics_screen.view(self.theme()).map(Message::LyricsMessage),
            Screen::Themes => self.themes_screen.view(self.theme()).map(Message::ThemesMessage),
        };

        row![
            self.sidebar.view(self.theme()).map(Message::SidebarMessage),
            screen_view
        ].into()
    }

    fn theme(&self) -> iced::Theme {
        self.theme.clone()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        iced::Subscription::batch([
            iced::time::every(Duration::from_millis(50)).map(Message::Tick),
            self.lyrics_screen.subscription().map(Message::LyricsMessage)
        ])
    }
}