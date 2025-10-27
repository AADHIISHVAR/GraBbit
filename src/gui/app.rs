use base64::Engine;
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
    
    // Image decode
    pub image_decode_input: String,
    pub custom_save_dir: String,
    
    // Counters
    pub connected_users: u8,
    pub file_transfers: u8,
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
    ImageDecodeInputChanged(String),
    CustomSaveDirChanged(String),
    DecodeImage,
    ShowLatestImageData,
    DownloadFile(ClipboardItem),
    CopyEncryptedData(ClipboardItem),
}

impl Sandbox for Display {
    type Message = Messages;

    fn new() -> Self {
        let clip_data = match crate::storage::load_clipboard_history() {
            Ok(items) => items,
            Err(_) => Vec::new(),
        };
        
        // Get detected IP
        let detected_ip = crate::network::get_local_ip().unwrap_or_else(|| "127.0.0.1".to_string());
        
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
            host_ip: detected_ip, // Use detected IP
            port: String::from("7879"),
            host_duration: String::from("30 minutes"),
            
            // Image decode
            image_decode_input: String::new(),
            custom_save_dir: String::from("/home/aadhiishvar/Downloads"),
            
            // Counters
            connected_users: 0,
            file_transfers: 0,
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
            Messages::ImageDecodeInputChanged(input) => {
                self.image_decode_input = input;
            }
            Messages::CustomSaveDirChanged(dir) => {
                self.custom_save_dir = dir;
            }
            Messages::DecodeImage => {
                // Decode base64 image and save to custom directory
                if let Ok(decoded) = base64::engine::general_purpose::STANDARD.decode(&self.image_decode_input) {
                    let filename = format!("decoded_image_{}.png", chrono::Local::now().timestamp());
                    let save_path = std::path::Path::new(&self.custom_save_dir).join(filename);
                    
                    if let Err(e) = std::fs::write(&save_path, decoded) {
                        eprintln!("Error saving decoded image: {}", e);
                    } else {
                        println!("Image saved to: {}", save_path.display());
                        self.image_decode_input.clear();
                    }
                }
            }
            Messages::ShowLatestImageData => {
                // Find the latest image entry and show its encoded data
                if let Some(latest_image) = self.clip_data.iter()
                    .rev()
                    .find(|item| item.content.contains("🖼️")) {
                    // For now, just show a message that we found an image
                    self.image_decode_input = "Image data found - check console for details".to_string();
                    println!("Found latest image: {}", latest_image.content);
                } else {
                    println!("No image data found in clipboard history");
                }
            }
            Messages::DownloadFile(item) => {
                // Download and decode the file
                if let Some(encoded_data) = &item.encoded_data {
                    if let Ok(decoded) = base64::engine::general_purpose::STANDARD.decode(encoded_data) {
                        let filename = if let Some(file_path) = &item.file_path {
                            std::path::Path::new(file_path)
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("downloaded_file")
                                .to_string()
                        } else {
                            format!("downloaded_file_{}", chrono::Local::now().timestamp())
                        };
                        
                        let save_path = std::path::Path::new(&self.custom_save_dir).join(filename);
                        
                        if let Err(e) = std::fs::write(&save_path, decoded) {
                            eprintln!("Error saving downloaded file: {}", e);
                        } else {
                            println!("File downloaded to: {}", save_path.display());
                        }
                    }
                }
            }
            Messages::CopyEncryptedData(item) => {
                // Copy encoded data to clipboard and paste in decoder
                if let Some(encoded_data) = &item.encoded_data {
                    self.image_decode_input = encoded_data.clone();
                    println!("Copied encrypted data to decoder input");
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
