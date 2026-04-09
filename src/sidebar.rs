#[allow(unused_imports)]
use iced::{
    Border, Element, Font, Length::{self, *}, font, widget::{button, column, container, row, space::Space, text}
};
use iced_anim::{AnimationBuilder, Easing};

use crate::Screen;

const FULL_WIDTH: f32 = 250.0;
const COLLAPSED_WIDTH: f32 = 30.0;

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
    pub fn view(&self) -> Element<'_, Message> {
        AnimationBuilder::new(self.width, |width| {
            self.sidebar_content(width)
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

    fn sidebar_content(&self, width: f32) -> Element<'_, Message> {
        let collapse_text = text(match self.collapsed {
            true => ">",
            false => "<"
        })
        .center()
        .font(Font {
            family: font::Family::SansSerif,
            weight: font::Weight::ExtraBold,
            stretch: font::Stretch::default(),
            style: font::Style::Normal
        });

        let collapse_button = button(collapse_text)
            .on_press(Message::ToggleCollapse)
            .width(32)
            .height(32)
            .style(|_theme, _status| button::Style {
                border: Border {
                    radius: 16.0.into(),
                    ..Default::default()
                },
                ..button::Style::default()
            });
        
        container(column![
            row![
                Space::new().width(Fill),
                collapse_button
            ],
            self.nav_button("Lyrics", Screen::Lyrics),

        ])
        .style(|theme| container::secondary(theme))
        .width(width)
        .height(Fill)
        .into()
    }

    fn nav_button(&self, label: &'static str, screen: Screen) -> Option<Element<'_, Message>> {
        (!self.collapsed).then_some(
            container(button(label)
                .on_press(Message::Navigate(screen))
                .width(Fill))
                .padding([0, 10])
                .center_x(Fill)
                .into()
        )
    }
}