use std::error::Error;
use gui::app::Display;
use iced::Sandbox;
use std::sync::atomic::{AtomicU8, Ordering};

pub mod gui;
pub mod storage;
pub mod security;
pub mod clipboard;
pub mod network;

// Global counters using atomics instead of Arc<Mutex>
static CONNECTED_USERS: AtomicU8 = AtomicU8::new(0);
static FILE_TRANSFERS: AtomicU8 = AtomicU8::new(0);

fn main() -> Result<(), Box<dyn Error>> {
    // Detect OS
    network::detect_os()?;
    
    // Get local IP automatically and update GUI
    let detected_ip = network::get_local_ip().unwrap_or_else(|| "127.0.0.1".to_string());
    println!("Detected local IP: {}", detected_ip);

    let json_file = "clipboard_history.json";

    // Start clipboard monitoring in background thread
    let json_file_clone = json_file.to_string();
    std::thread::spawn(move || {
        let _ = clipboard::monitor_clipboard(&json_file_clone);
    });

    // Start TCP server in background thread with user counter
    std::thread::spawn(move || {
        if let Err(e) = start_server_with_counter() {
            eprintln!("Server error: {}", e);
        }
    });

    // Run GUI application with detected IP
    let mut settings = iced::Settings::default();
    // We'll need to pass the IP to the GUI somehow
    Display::run(settings)?;

    Ok(())
}

fn start_server_with_counter() -> Result<(), Box<dyn Error>> {
    let ip = "0.0.0.0:7879";
    let listener = std::net::TcpListener::bind(ip)?;
    
    println!("Server listening on {}", ip);
    
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                // Increment connected users counter
                CONNECTED_USERS.fetch_add(1, Ordering::Relaxed);
                let count = CONNECTED_USERS.load(Ordering::Relaxed);
                println!("User connected. Total users: {}", count);
                
                // Handle client in separate thread
                std::thread::spawn(move || {
                    network::handle_client(_stream);
                });
            }
            Err(_) => {}
        }
    }

    Ok(())
}
