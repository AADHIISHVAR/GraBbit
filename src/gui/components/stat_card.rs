use iced::{Element, Length};
use iced::widget::{column, container, text};

pub fn view<'a>(value: u8, label: &'a str) -> Element<'a, crate::gui::app::Messages> {
    container(
        column![
            text(value.to_string()).size(32),
            text(label).size(14),
        ]
        .spacing(8)
    )
    .width(Length::Fill)
    .padding(20)
    .into()
}
