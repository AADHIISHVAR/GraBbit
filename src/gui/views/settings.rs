use iced::{Element, Length};
use iced::widget::{button, column, container, pick_list, row, scrollable, text, text_input, toggler, Column};
use crate::gui::app::{CompressionQuality, ConnectionMode, DataRetention, Display, Messages};

pub fn view(app: &Display) -> Element<Messages> {
    scrollable(
        column![
            settings_box(app),
            connection_mode_box(app),
        ]
            .spacing(30)
    )
        .into()
}

fn settings_box(app: &Display) -> Element<Messages> {
    container(
        column![
            text("Settings").size(24),

            column![
                text("Theme").size(14),
                row![
                    text("Light"),
                    toggler(String::from(""), app.darkmode, |value| Messages::ToggleTheme),
                    text("Dark"),
                ]
                .spacing(10),
            ]
            .spacing(10),

            column![
                text("Image Compression Quality").size(14),
                text("Select compression level for image transfers").size(12),
                row![
                    compression_button(app, "Default", CompressionQuality::Default),
                    compression_button(app, "Normal", CompressionQuality::Normal),
                    compression_button(app, "High", CompressionQuality::High),
                ]
                .spacing(10),
            ]
            .spacing(10),

            column![
                text("Secure Transfer").size(14),
                row![
                    toggler(String::from(""), app.secure_transfer, Messages::SecureTransferToggled),
                    text("Enabled"),
                ]
                .spacing(10),
            ]
            .spacing(10),

            column![
                text("Data Retention").size(14),
                pick_list(
                    &DataRetention::ALL[..],
                    Some(app.data_retention.clone()),
                    Messages::DataRetentionChanged,
                )
                .padding(10)
                .width(Length::Fill),
            ]
            .spacing(10),
        ]
            .spacing(20)
    )
        .width(Length::Fill)
        .padding(30)
        .into()
}

fn compression_button<'a>(
    app: &Display,
    label: &'a str,
    quality: CompressionQuality
) -> button::Button<'a, Messages> {
    let is_active = app.compression_quality == quality;
    button(text(label).size(14))
        .padding(12)
        .width(Length::Fill)
        .on_press(Messages::CompressionChanged(quality))
}

fn connection_mode_box(app: &Display) -> Element<Messages> {
    container(
        column![
            text("Connection Mode").size(24),

            row![
                mode_button(app, "Host Mode", ConnectionMode::Host),
                mode_button(app, "Node Mode", ConnectionMode::Node),
            ]
            .spacing(10)
            .width(Length::Fill),

            if app.connection_mode == ConnectionMode::Host {
                host_mode_inputs(app)
            } else {
                node_mode_inputs(app)
            }
        ]
            .spacing(20)
    )
        .width(Length::Fill)
        .padding(30)
        .into()
}

fn mode_button<'a>(
    app: &Display,
    label: &'a str,
    mode: ConnectionMode
) -> button::Button<'a, Messages> {
    let is_active = app.connection_mode == mode;
    button(text(label).size(14))
        .padding(12)
        .width(Length::Fill)
        .on_press(Messages::ConnectionModeChanged(mode))
}

fn host_mode_inputs(app: &Display) -> Column<Messages> {
    column![
        column![
            text("Wi-Fi IP Address").size(14),
            text_input("", &app.host_ip)
                .on_input(Messages::HostIPChanged)
                .padding(12),
        ]
        .spacing(10),

        column![
            text("Port Number").size(14),
            text_input("", &app.port)
                .on_input(Messages::PortChanged)
                .padding(12),
        ]
        .spacing(10),

        column![
            text("Host Active Duration").size(14),
            text_input("", &app.host_duration)
                .on_input(Messages::HostDurationChanged)
                .padding(12),
        ]
        .spacing(10),

        button(text("Start Hosting").size(14))
            .padding(12)
            .on_press(Messages::StartHosting),
    ]
        .spacing(20)
}

fn node_mode_inputs(app: &Display) -> Column<Messages> {
    column![
        column![
            text("Host IP Address").size(14),
            text_input("", &app.host_ip)
                .on_input(Messages::HostIPChanged)
                .padding(12),
        ]
        .spacing(10),

        column![
            text("Port Number").size(14),
            text_input("", &app.port)
                .on_input(Messages::PortChanged)
                .padding(12),
        ]
        .spacing(10),

        button(text("Connect to Host").size(14))
            .padding(12)
            .on_press(Messages::ConnectToHost),
    ]
        .spacing(20)
}
