use iced::{Background, Border, Color, Theme};
use iced::widget::{button, container};

pub fn card_container(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb(0.1, 0.12, 0.16))),
        border: Border {
            color: Color::from_rgb(0.2, 0.25, 0.3),
            width: 1.0,
            radius: 12.0.into(),
        },
        ..Default::default()
    }
}

pub fn item_container(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb(0.1, 0.12, 0.16))),
        border: Border {
            color: Color::from_rgb(0.2, 0.25, 0.3),
            width: 1.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    }
}

pub fn tab_button(is_active: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme: &Theme, status| {
        match status {
            button::Status::Hovered => button::Style {
                background: Some(Background::Color(Color::from_rgb(0.3, 0.4, 0.7))),
                text_color: Color::WHITE,
                border: Border {
                    radius: 6.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            },
            _ => button::Style {
                background: if is_active {
                    Some(Background::Color(Color::from_rgb(0.4, 0.5, 0.8)))
                } else {
                    None
                },
                text_color: Color::WHITE,
                border: Border {
                    radius: 6.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        }
    }
}

pub fn active_button(is_active: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme: &Theme, status| {
        match status {
            button::Status::Hovered => button::Style {
                background: Some(Background::Color(Color::from_rgb(0.4, 0.5, 0.8))),
                text_color: Color::WHITE,
                border: Border {
                    radius: 6.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            },
            _ => button::Style {
                background: if is_active {
                    Some(Background::Color(Color::from_rgb(0.5, 0.6, 0.9)))
                } else {
                    Some(Background::Color(Color::from_rgb(0.15, 0.17, 0.2)))
                },
                text_color: Color::WHITE,
                border: Border {
                    color: Color::from_rgb(0.25, 0.3, 0.35),
                    width: 1.0,
                    radius: 6.0.into(),
                },
                ..Default::default()
            }
        }
    }
}

pub fn primary_button(_theme: &Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.5, 0.6, 1.0))),
            text_color: Color::WHITE,
            border: Border {
                radius: 6.0.into(),
                ..Default::default()
            },
            ..Default::default()
        },
        _ => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.4, 0.5, 0.9))),
            text_color: Color::WHITE,
            border: Border {
                radius: 6.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
