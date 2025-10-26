use iced::{Element, Sandbox, Theme};
use crate::gui::views::{dashboard, settings, about};
use crate::gui::components::header;
use crate::storage::ClipboardItem;

#[derive(Debug, Clone)]
pub enum Page {
    Dashboard,
    Settings,
    About,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompressionQuality {
    Default,
    Normal,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataRetention {
    Manual,
    OneDay,
    OneWeek,
    OneMonth,
}

impl DataRetention {
    pub const ALL: &'static [DataRetention] = &[
        DataRetention::Manual,
        DataRetention::OneDay,
        DataRetention::OneWeek,
        DataRetention::OneMonth,
    ];
}

impl std::fmt::Display for DataRetention {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataRetention::Manual => write!(f, "Until Manual Deletion"),
            DataRetention::OneDay => write!(f, "1 Day"),
            DataRetention::OneWeek => write!(f, "1 Week"),
            DataRetention::OneMonth => write!(f, "1 Month"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionMode {
    Host,
    Node,
}

#[derive(Debug, Clone)]
pub struct Display {
    pub current_page: Page,
    pub clip_data: Vec<ClipboardItem>,
    pub search_text: String,
    pub connected_devices: u8,
    pub active_transfers: u8,
    
    // Settings
    pub darkmode: bool,
    pub compression_quality: CompressionQuality,
    pub secure_transfer: bool,
    pub data_retention: DataRetention,
    
    // Connection
    pub connection_mode: ConnectionMode,
    pub host_ip: String,
    pub port: String,
    pub host_duration: String,
}

#[derive(Debug, Clone)]
pub enum Messages {
    TabChanged(Page),
    SearchChanged(String),
    ClearAll,
    ToggleTheme,
    SecureTransferToggled(bool),
    CompressionChanged(CompressionQuality),
    DataRetentionChanged(DataRetention),
    ConnectionModeChanged(ConnectionMode),
    HostIPChanged(String),
    PortChanged(String),
    HostDurationChanged(String),
    StartHosting,
    ConnectToHost,
    ReloadData,
}

impl Sandbox for Display {
    type Message = Messages;

    fn new() -> Self {
        let clip_data = match crate::storage::load_clipboard_history() {
            Ok(items) => items,
            Err(_) => Vec::new(),
        };
        
        Display {
            current_page: Page::Dashboard,
            clip_data,
            search_text: String::new(),
            connected_devices: 0,
            active_transfers: 0,
            darkmode: true,
            compression_quality: CompressionQuality::Default,
            secure_transfer: true,
            data_retention: DataRetention::Manual,
            connection_mode: ConnectionMode::Host,
            host_ip: String::new(),
            port: String::from("7879"),
            host_duration: String::new(),
        }
    }

    fn title(&self) -> String {
        String::from("GraBbit")
    }

    fn theme(&self) -> Theme {
        if self.darkmode {
            Theme::Dark
        } else {
            Theme::Light
        }
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Messages::TabChanged(page) => {
                let page_clone = page.clone();
                self.current_page = page;
                // Reload data when switching to dashboard
                if matches!(page_clone, Page::Dashboard) {
                    if let Ok(items) = crate::storage::load_clipboard_history() {
                        self.clip_data = items;
                    }
                }
            }
            Messages::SearchChanged(text) => {
                self.search_text = text;
            }
            Messages::ClearAll => {
                if let Err(e) = crate::storage::clear_clipboard_history() {
                    eprintln!("Error clearing clipboard history: {}", e);
                }
                self.clip_data.clear();
            }
            Messages::ToggleTheme => {
                self.darkmode = !self.darkmode;
            }
            Messages::SecureTransferToggled(enabled) => {
                self.secure_transfer = enabled;
            }
            Messages::CompressionChanged(quality) => {
                self.compression_quality = quality;
            }
            Messages::DataRetentionChanged(retention) => {
                self.data_retention = retention;
            }
            Messages::ConnectionModeChanged(mode) => {
                self.connection_mode = mode;
            }
            Messages::HostIPChanged(ip) => {
                self.host_ip = ip;
            }
            Messages::PortChanged(port) => {
                self.port = port;
            }
            Messages::HostDurationChanged(duration) => {
                self.host_duration = duration;
            }
            Messages::StartHosting => {
                println!("Starting hosting on {}:{}", self.host_ip, self.port);
                self.connected_devices += 1;
            }
            Messages::ConnectToHost => {
                println!("Connecting to host at {}:{}", self.host_ip, self.port);
                self.active_transfers += 1;
            }
            Messages::ReloadData => {
                // Reload clipboard history from JSON file
                if let Ok(items) = crate::storage::load_clipboard_history() {
                    self.clip_data = items;
                    println!("Reloaded {} clipboard entries", self.clip_data.len());
                }
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        iced::widget::column![
            header::view(self),
            self.page_content(),
        ]
        .spacing(30)
        .padding(20)
        .into()
    }
}

impl Display {
    fn page_content(&self) -> Element<Messages> {
        match self.current_page {
            Page::Dashboard => dashboard::view(self),
            Page::Settings => settings::view(self),
            Page::About => about::view(),
        }
    }
}
