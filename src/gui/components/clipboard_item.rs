use crate::gui::app::Messages;
use iced::{Element, Length};
use iced::widget::{column, container, row, text, button};
use crate::storage::ClipboardItem;

pub fn view<'a>(item: &'a ClipboardItem) -> Element<'a, Messages> {
    let mut buttons = row![];
    
    // Add download button if it's a file with encoded data
    if item.encoded_data.is_some() && (item.content_type == "file" || item.content_type == "image") {
        buttons = buttons.push(
            button(text("📥 Download").size(12))
                .padding(8)
                .on_press(Messages::DownloadFile(item.clone()))
        );
    }
    
    // Add copy encrypted data button if there's encoded data
    if item.encoded_data.is_some() {
        buttons = buttons.push(
            button(text("📋 Copy Data").size(12))
                .padding(8)
                .on_press(Messages::CopyEncryptedData(item.clone()))
        );
    }
    
    container(
        row![
            text(&item.content).size(14).width(Length::Fill),
            column![
                text(&item.device).size(12),
                text(&item.os).size(12),
                buttons.spacing(5),
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