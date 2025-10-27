use iced::{Element, Length};
use iced::widget::{button, column, row, scrollable, text, text_input, Column};
use iced::alignment::Horizontal;
use crate::gui::app::{Display, Messages};
use crate::gui::components::{stat_card, clipboard_item};

pub fn view(app: &Display) -> Element<Messages> {
    column![
        stats_section(app),
        clipboard_section(app),
    ]
        .spacing(30)
        .into()
}

fn stats_section(app: &Display) -> Element<Messages> {
    row![
        stat_card::view(app.connected_devices, "Connected Devices"),
        stat_card::view(app.file_transfers, "File Transfers"),
        stat_card::view(app.clip_data.len() as u8, "Total Items"),
    ]
        .spacing(20)
        .width(Length::Fill)
        .into()
}

fn clipboard_section(app: &Display) -> Element<Messages> {
    column![
        row![
            text("Clipboard History").size(24),
            iced::widget::container(
                row![
                    button(text("🔄 Reload").size(14))
                        .padding(8)
                        .on_press(Messages::ReloadData),
                    button(text("Clear All").size(14))
                        .padding(8)
                        .on_press(Messages::ClearAll)
                ]
                .spacing(10)
            )
            .width(Length::Fill)
            .align_x(Horizontal::Right),
        ]
        .width(Length::Fill),

        text_input("Search by content, device, or user...", &app.search_text)
            .on_input(Messages::SearchChanged)
            .padding(12),

        scrollable(
            Column::with_children(
                app.clip_data
                    .iter()
                    .map(|item| clipboard_item::view(item))
                    .collect::<Vec<_>>()
            )
            .spacing(12)
        )
        .height(Length::Fixed(300.0)),
    ]
        .spacing(20)
        .into()
}