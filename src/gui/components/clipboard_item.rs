use iced::{Element, Length};
use iced::widget::{column, container, row, text};
use crate::storage::ClipboardItem;
use crate::gui::app::Messages;

pub fn view<'a>(item: &'a ClipboardItem) -> Element<'a, Messages> {
    container(
        row![
            text(&item.content).size(14).width(Length::Fill),
            column![
                text(&item.device).size(12),
                text(&item.os).size(12),
            ]
            .spacing(4),
            text(&item.time).size(12),
        ]
            .spacing(20)
    )
        .width(Length::Fill)
        .padding(16)
        .into()
}