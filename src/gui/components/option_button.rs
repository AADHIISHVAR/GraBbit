
use iced::{Element, Length};
use iced::widget::{button, text};
use crate::gui::styles;

pub fn view<'a, T: 'a, F>(
    label: &'a str,
    is_active: bool,
    message: T,
    on_press: F,
) -> button::Button<'a, T>
where
    F: Fn(T) -> T + 'a,
    T: Clone,
{
    button(text(label).size(14))
        .padding(12)
        .width(Length::Fill)
        .style(styles::active_button(is_active))
        .on_press(message)
}