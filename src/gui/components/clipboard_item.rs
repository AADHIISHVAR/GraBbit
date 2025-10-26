use iced::{Element, Length};
use iced::widget::{column, container, row, text};
use crate::gui::app::{ClipboardItem, Messages};
use crate::gui::styles;

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
        .style(styles::item_container)
        .into()
}