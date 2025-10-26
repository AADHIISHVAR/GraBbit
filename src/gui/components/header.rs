use iced::{Element, Length};
use iced::widget::{button, container, row, text};
use iced::alignment::Horizontal;
use crate::gui::app::{Display, Messages, Page};

pub fn view(app: &Display) -> Element<Messages> {
    container(
        row![
            text("GraBbit").size(32),
            container(
                row![
                    tab_button(app, "Dashboard", Page::Dashboard),
                    tab_button(app, "Settings", Page::Settings),
                    tab_button(app, "About", Page::About),
                ]
                .spacing(10)
            )
            .width(Length::Fill)
            .align_x(Horizontal::Right),
        ]
        .width(Length::Fill)
    )
    .width(Length::Fill)
    .into()
}

fn tab_button<'a>(app: &Display, label: &'a str, page: Page) -> button::Button<'a, Messages> {
    button(text(label).size(16))
        .padding(12)
        .on_press(Messages::TabChanged(page.clone()))
}
