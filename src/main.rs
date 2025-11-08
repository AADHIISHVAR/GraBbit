use std::error::Error;
use gui::app::Display;
use iced::Sandbox;
use std::sync::atomic::{AtomicU8, Ordering};
use std::net::TcpStream;
use std::io::Write;

pub mod gui;
pub mod storage;
pub mod security;
pub mod clipboard;
pub mod network;

// Global counters using atomics instead of Arc<Mutex>
static CONNECTED_USERS: AtomicU8 = AtomicU8::new(0);
static FILE_TRANSFERS: AtomicU8 = AtomicU8::new(0);

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
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

fn start_server_with_counter() -> Result<(), Box<dyn Error + Send + Sync>> {
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

                // Stream clipboard history to client
                std::thread::spawn(move || {
                    let _ = stream_clipboard_history(_stream);
                });
            }
            Err(_) => {}
        }
    }

    Ok(())
}

fn stream_clipboard_history(mut stream: TcpStream) -> Result<(), Box<dyn Error + Send + Sync>> {
    match std::fs::read_to_string("clipboard_history.json") {
        Ok(contents) => {
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                contents.len(),
                contents
            );
            let _ = stream.write_all(response.as_bytes());
        }
        Err(_) => {}
    }
    Ok(())
}
