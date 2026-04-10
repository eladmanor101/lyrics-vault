#[allow(unused_imports)]
use iced::{
    Length::{self, *}, font, widget::{button, column, container, row, space::Space, text}
};
use iced_anim::{AnimationBuilder, Easing};

use crate::Screen;

const FULL_WIDTH: f32 = 250.0;
const COLLAPSED_WIDTH: f32 = 48.0;

#[derive(Debug)]
pub struct Sidebar {
    collapsed: bool,
    width: f32
}

impl Default for Sidebar {
    fn default() -> Self {
        Self {
            collapsed: false,
            width: FULL_WIDTH
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Navigate(Screen),
    ToggleCollapse
}

pub enum Action {
    Navigate(Screen)
}

impl Sidebar {
    pub fn view(&self, theme: iced::Theme) -> iced::Element<'_, Message> {
        AnimationBuilder::new(self.width, move |width| {
            self.sidebar_content(width, &theme)
        })
        .animates_layout(true)
        .animation(Easing::EASE)
        .into()
    }

    pub fn update(&mut self, message: Message) -> Option<Action> {
        match message {
            Message::Navigate(screen) => Some(Action::Navigate(screen)),
            Message::ToggleCollapse => {
                self.collapsed = !self.collapsed;
                self.width = match self.collapsed {
                    true => COLLAPSED_WIDTH,
                    false => FULL_WIDTH
                };
                None
            }
        }
    }

    fn sidebar_content(&self, width: f32, theme: &iced::Theme) -> iced::Element<'_, Message> {
        let collapse_text = text(match self.collapsed {
            true => ">",
            false => "<"
        })
        .center()
        .font(iced::Font {
            family: font::Family::SansSerif,
            weight: font::Weight::ExtraBold,
            stretch: font::Stretch::default(),
            style: font::Style::Normal
        });

        let collapse_button = button(collapse_text)
            .on_press(Message::ToggleCollapse)
            .width(32)
            .height(32)
            .style(|theme: &iced::Theme, status| {
                let palette = theme.extended_palette();
                let color = match status {
                    button::Status::Hovered => palette.background.strongest.color,
                    button::Status::Pressed => palette.background.strong.color,
                    _ => palette.background.stronger.color,
                };
                button::Style {
                    background: Some(iced::Background::Color(color)),
                    border: iced::Border {
                        radius: 16.0.into(),
                        ..Default::default()
                    },
                    text_color: palette.background.base.text,
                    ..Default::default()
                }
            });

        let alpha = ((width - COLLAPSED_WIDTH) / (FULL_WIDTH - COLLAPSED_WIDTH)).clamp(0.0, 1.0).powi(3);

        container(column![
            row![
                Space::new().width(Fill),
                collapse_button
            ],
            Space::new().height(12),
            self.nav_button("Lyrics", Screen::Lyrics, theme, alpha),
            self.nav_button("Themes", Screen::Themes, theme, alpha),
        ])
        .style(|theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(
                theme.extended_palette().background.weakest.color
            )),
            border: iced::Border {
                width: 1.0,
                color: theme.extended_palette().background.strong.color,
                ..Default::default()
            },
            ..Default::default()
        })
        .width(width)
        .height(Fill)
        .padding([8, 8])
        .into()
    }

    fn nav_button(&self, label: &'static str, screen: Screen, theme: &iced::Theme, alpha: f32) -> iced::Element<'_, Message> {
        let palette = theme.extended_palette();

        container(
            button(
                text(label)
                    .color(iced::Color {
                        a: alpha,
                        ..palette.background.base.text
                    })
                    .size(13)
            )
            .on_press(Message::Navigate(screen))
            .width(Fill)
            .style(move |theme: &iced::Theme, status| {
                let palette = theme.extended_palette();
                let color = match status {
                    button::Status::Hovered => palette.background.stronger.color,
                    button::Status::Pressed => palette.background.strongest.color,
                    _ => iced::Color::TRANSPARENT,
                };
                button::Style {
                    background: Some(iced::Background::Color(iced::Color { a: color.a * alpha, ..color })),
                    border: iced::Border { radius: 6.0.into(), ..Default::default() },
                    text_color: iced::Color { a: alpha, ..palette.background.base.text },
                    ..Default::default()
                }
            })
        )
        .padding([2, 8])
        .width(Fill)
        .into()
    }
}