// Main entry point for GraBbit
// Wiring up GUI and background services

use std::error::Error;
use gui::app::Display;
use iced::Sandbox;

pub mod gui;
pub mod storage;
pub mod security;
pub mod clipboard;
pub mod network;

fn main() -> Result<(), Box<dyn Error>> {
    // Detect OS
    network::detect_os()?;

    // Get local IP
    network::get_local_ip();

    let json_file = "clipboard_history.json";

    // Start clipboard monitoring in background thread
    let json_file_clone = json_file.to_string();
    std::thread::spawn(move || {
        let _ = clipboard::monitor_clipboard(&json_file_clone);
    });

    // Start TCP server in background thread
    std::thread::spawn(move || {
        if let Err(e) = network::start_server() {
            eprintln!("Server error: {}", e);
        }
    });

    // Run GUI application  
    Display::run(iced::Settings::default())?;

    Ok(())
}
