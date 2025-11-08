use arboard::Clipboard;
use std::{thread, time::Duration};
use std::path::{Path, PathBuf};
use std::error::Error;
use regex::Regex;
use url::Url;
use base64::Engine;
use tokio;


/// File type enum
#[derive(Debug)]
pub enum FileType {
    Pdf,
    Image(String),   // png, jpg, etc.
    Text,
    Audio(String),
    Video(String),
    Document(String), // odt, docx, etc.
    Unknown,
}

/// File info structure
#[derive(Debug)]
pub struct FileInfo {
    pub path: PathBuf,
    pub file_type: FileType,
}

/// Extract path from clipboard text
pub fn extract_path_from_clipboard(text: &str) -> Option<FileInfo> {
    // 1) GNOME/Nautilus style file URIs
    if text.starts_with("copy\n") || text.starts_with("cut\n") {
        for line in text.lines().skip(1) {
            if let Ok(url) = Url::parse(line.trim()) {
                if url.scheme() == "file" {
                    if let Ok(path) = url.to_file_path() {
                        return Some(FileInfo {
                            path: path.clone(),  // Keep the full path
                            file_type: detect_file_type(&path),
                        });
                    }
                }
            }
        }
    }

    // 2) plain text paths
    let re = Regex::new(r"(?:(?:[A-Za-z]:)?[/\\][\w\s.\-/\\]+)").unwrap();
    if let Some(cap) = re.find(text) {
        let path_str = cap.as_str().trim_matches('"');
        let path = PathBuf::from(path_str);  // Convert to PathBuf directly
        return Some(FileInfo {
            path: path.clone(),  // Keep the full path
            file_type: detect_file_type(&path),
        });
    }

    None
}

/// Detect file type from extension
pub fn detect_file_type(path: &Path) -> FileType {
    match path.extension().and_then(|e| e.to_str()).map(|s| s.to_lowercase()) {
        Some(ref ext) if ext == "pdf" => FileType::Pdf,
        Some(ref ext) if ["png", "jpg", "jpeg", "bmp", "gif", "webp"].contains(&ext.as_str()) => {
            FileType::Image(ext.clone())
        }
        Some(ref ext) if ext == "txt" => FileType::Text,
        Some(ref ext) if ["mp3", "wav", "ogg", "flac"].contains(&ext.as_str()) => FileType::Audio(ext.clone()),
        Some(ref ext) if ["mp4", "avi", "mkv", "mov", "webm"].contains(&ext.as_str()) => FileType::Video(ext.clone()),
        Some(ref ext) if ["odt", "docx", "doc", "rtf"].contains(&ext.as_str()) => FileType::Document(ext.clone()),
        _ => FileType::Unknown,
    }
}

async fn process_file_async(file_info: FileInfo, json_path: String) {
    let file_name = file_info.path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Unknown file")
        .to_string();

    match &file_info.file_type {
        FileType::Image(_) => {
            let display_text = format!("🖼️  {}", file_name);
            let path_clone = file_info.path.clone();
            if let Ok(encoded_data) = tokio::task::spawn_blocking(move || compress_file(&path_clone, crate::gui::app::CompressionQuality::Default)).await.unwrap() {
                let _ = tokio::task::spawn_blocking(move || {
                    crate::storage::save_clipboard_entry(
                        "image",
                        Some(display_text),
                        Some(encoded_data),
                        Some(file_info.path.to_string_lossy().to_string()),
                        &json_path
                    )
                }).await;
            }
        }
        FileType::Pdf | FileType::Document(_) => {
            let display_text = match &file_info.file_type {
                FileType::Pdf => format!("📄 {}", file_name),
                FileType::Document(_) => format!("📝 {}", file_name),
                _ => format!("📁 {}", file_name),
            };
            let _ = tokio::task::spawn_blocking(move || {
                crate::storage::save_clipboard_entry(
                    "document",
                    Some(display_text),
                    None,
                    Some(file_info.path.to_string_lossy().to_string()),
                    &json_path
                )
            }).await;
        }
        FileType::Audio(_) | FileType::Video(_) => {
            let display_text = match &file_info.file_type {
                FileType::Audio(_) => format!("🎵 {}", file_name),
                FileType::Video(_) => format!("🎬 {}", file_name),
                _ => format!("📁 {}", file_name),
            };
            let _ = tokio::task::spawn_blocking(move || {
                crate::storage::save_clipboard_entry(
                    "media",
                    Some(display_text),
                    None,
                    Some(file_info.path.to_string_lossy().to_string()),
                    &json_path
                )
            }).await;
        }
        _ => {
            let display_text = format!("📁 {}", file_name);
            let path_clone = file_info.path.clone();
            if let Ok(encoded_data) = tokio::task::spawn_blocking(move || compress_file(&path_clone, crate::gui::app::CompressionQuality::Default)).await.unwrap() {
                let _ = tokio::task::spawn_blocking(move || {
                    crate::storage::save_clipboard_entry(
                        "file",
                        Some(display_text),
                        Some(encoded_data),
                        Some(file_info.path.to_string_lossy().to_string()),
                        &json_path
                    )
                }).await;
            }
        }
    }
}

/// Check if text is a file path
pub fn is_file_path(text: &str) -> bool {
    let path = Path::new(text);
    path.exists() && path.is_file()
}

/// Compress file: read raw bytes, compress with zstd, base64 encode
pub fn compress_file(path: &Path, _quality: crate::gui::app::CompressionQuality) -> Result<String, Box<dyn Error + Send + Sync>> {
    // Read raw bytes directly
    let file_data = std::fs::read(path)?;
    let compressed = crate::security::compress_data(&file_data)?;
    Ok(base64::engine::general_purpose::STANDARD.encode(&compressed))
}



/// Monitor clipboard in a loop
pub fn monitor_clipboard(json_path: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut clipboard = Clipboard::new()?;
    let mut last_content = String::new();

    loop {
        match clipboard.get_text() {
            Ok(current_content) => {
                if current_content != last_content
                {
                    last_content = current_content.clone();
                    println!("Saved clipboard: {}", &current_content);

                    if let Some(file_info) = extract_path_from_clipboard(&current_content) {
                        let json_file_clone = json_path.to_string();
                        let file_name = file_info.path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("Unknown file")
                            .to_string();
                        tokio::spawn(async move {
                            process_file_async(file_info, json_file_clone).await;
                        });
                        println!("Detected file: {}", file_name);
                    } else if is_file_path(&current_content) {
                        let path = Path::new(&current_content);
                        let file_info = FileInfo {
                            path: path.to_path_buf(),
                            file_type: detect_file_type(path),
                        };
                        let json_file_clone = json_path.to_string();
                        let file_name = path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("Unknown file")
                            .to_string();
                        tokio::spawn(async move {
                            process_file_async(file_info, json_file_clone).await;
                        });
                        println!("Detected and compressed file: {}", file_name);
                    } else {
                        // Plain text
                        if let Err(e) = crate::storage::save_clipboard_entry(
                            "text",
                            Some(current_content.clone()),
                            None,
                            None,
                            json_path
                        ) {
                            eprintln!("Error saving text entry: {}", e);
                        }
                    }

                }
            }
            Err(_) => {
                // todo clipboard might be empty or contain non-text na -> just skip
                continue;
            }
        }

        thread::sleep(Duration::from_secs(1));
    }
}

