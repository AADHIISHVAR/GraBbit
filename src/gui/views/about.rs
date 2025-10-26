use iced::{Element, Length};
use iced::widget::{column, container, text};
use crate::gui::app::Messages;

pub fn view() -> Element<'static, Messages> {
    container(
        column![
            text("About GraBbit").size(32).font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),

            text("GraBbit securely syncs your clipboard and shared data across devices on the same Wi-Fi network.\nHost once — connect from anywhere locally.").size(16),

            text("Designed for seamless collaboration and productivity, GraBbit eliminates the need for cloud\nservices by keeping everything local, fast, and private.").size(16),

            text("Upcoming Features").size(24).font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),

            column![
                text("→  End-to-end encryption for all transfers").size(14),
                text("→  Automatic sync across all connected devices").size(14),
                text("→  File sharing with drag-and-drop support").size(14),
                text("→  Cross-platform support (Windows, macOS, Linux, Mobile)").size(14),
                text("→  History backup and restore").size(14),
                text("→  Custom sync filters and preferences").size(14),
            ]
            .spacing(10),

            text("Version 1.0.0 | Made with ❤️ for local collaboration").size(14),
        ]
            .spacing(30)
    )
        .width(Length::Fill)
        .padding(50)
        .into()
}