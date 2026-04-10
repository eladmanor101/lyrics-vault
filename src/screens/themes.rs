use iced::{
    Element, Length::*,
    widget::{column, container, row, scrollable, text},
};

#[derive(Debug, Default)]
pub struct ThemesScreen;

#[derive(Debug, Clone)]
pub enum Message {
    ThemeSelected(iced::Theme),
}

pub enum Action {
    ThemeChanged(iced::Theme),
}

impl ThemesScreen {
    pub fn view(&self, theme: iced::Theme) -> Element<'_, Message> {
        column![
            self.header(&theme),
            self.body(&theme),
        ]
        .into()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::ThemeSelected(theme) => Action::ThemeChanged(theme),
        }
    }
}

impl ThemesScreen {
    fn header(&self, theme: &iced::Theme) -> Element<'_, Message> {
        let palette = theme.extended_palette();

        container(
            text("Themes")
                .size(16)
                .color(palette.background.base.text)
        )
        .width(Fill)
        .height(48)
        .padding([0, 24])
        .style(|theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(
                theme.extended_palette().background.weakest.color
            )),
            ..Default::default()
        })
        .center_y(48)
        .into()
    }

    fn body(&self, theme: &iced::Theme) -> Element<'_, Message> {
        let palette = theme.extended_palette();

        let section_label = text("Theme")
            .size(12)
            .color(palette.secondary.base.color);

        let swatches = iced::Theme::ALL
            .iter()
            .map(|t| self.theme_swatch(t, theme))
            .collect::<Vec<_>>();

        let grid = container(
            scrollable(
                column([
                    section_label.into(),
                    self.swatch_grid(swatches).into(),
                ])
                .spacing(12)
                .padding([16, 24])
            )
            .width(Fill)
            .height(Fill)
        )
        .width(Fill)
        .height(Fill);

        grid.into()
    }

    fn swatch_grid<'a>(&self, swatches: Vec<Element<'a, Message>>) -> Element<'a, Message> {
        let mut swatches = swatches.into_iter().peekable();
        let mut rows: Vec<Element<'_, Message>> = Vec::new();

        while swatches.peek().is_some() {
            let chunk: Vec<_> = swatches.by_ref().take(4).collect();
            rows.push(row(chunk).spacing(12).into());
        }

        column(rows).spacing(12).into()
    }

    fn theme_swatch<'a>(&self, t: &'a iced::Theme, current: &iced::Theme) -> Element<'a, Message> {
        let p = t.palette();
        let is_active = t == current;

        let swatch = row![
            container(text(""))
                .width(6)
                .height(Fill)
                .style(move |_: &iced::Theme| container::Style {
                    background: Some(iced::Background::Color(p.primary)),
                    ..Default::default()
                }),
            container(text(""))
                .width(Fill)
                .height(Fill)
                .style(move |_: &iced::Theme| container::Style {
                    background: Some(iced::Background::Color(p.background)),
                    ..Default::default()
                }),
        ]
        .height(48);

        let label = text(t.to_string())
            .size(11)
            .color(if is_active {
                current.extended_palette().primary.base.color
            } else {
                current.extended_palette().secondary.base.color
            });

        let inner = column![
            container(swatch)
                .width(Fill)
                .height(48)
                .style(move |_: &iced::Theme| container::Style {
                    border: iced::Border {
                        radius: 6.0.into(),
                        width: if is_active { 2.0 } else { 1.0 },
                        color: if is_active { p.primary } else { p.background },
                    },
                    ..Default::default()
                }),
            label,
        ]
        .spacing(4)
        .width(Fill);

        iced::widget::button(inner)
            .on_press(Message::ThemeSelected(t.clone()))
            .width(Fill)
            .style(|_, _| iced::widget::button::Style {
                background: None,
                ..Default::default()
            })
            .into()
    }
}