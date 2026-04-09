mod screens;
mod sidebar;
mod models;
mod lyrics_sources;
mod media_sources;
mod utils;

#[allow(unused_imports)]
use iced::{
    widget::{button, text},
    Length::*,
    Size,
    widget::{column, container, row}
};

use crate::screens::{
    lyrics
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

#[derive(Default)]
struct App {
    screen: Screen,

    sidebar: sidebar::Sidebar,
    lyrics_screen: lyrics::LyricsScreen
}

#[derive(Debug, Clone)]
enum Message {
    SidebarMessage(sidebar::Message),
    LyricsMessage(lyrics::Message)
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
enum Screen {
    #[default]
    Lyrics
}

impl App {
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
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
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        let screen_view = match self.screen {
            Screen::Lyrics => self.lyrics_screen.view().map(Message::LyricsMessage)
        };

        row![
            self.sidebar.view().map(Message::SidebarMessage),
            screen_view
        ].into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::TokyoNight
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        self.lyrics_screen.subscription().map(Message::LyricsMessage)
    }
}