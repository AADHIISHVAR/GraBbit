use arboard::Clipboard;
use std::{thread, time::Duration};
use std::path::{Path, PathBuf};
use regex::Regex;
use url::Url;

/// File type enum
#[derive(Debug)]
pub enum FileType {
    Pdf,
    Image(String),   // png, jpg, etc.
    Text,
    Audio(String),
    Video(String),
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
        Some(ref ext) if ["png", "jpg", "jpeg", "bmp", "gif"].contains(&ext.as_str()) => {
            FileType::Image(ext.clone())
        }
        Some(ref ext) if ext == "txt" => FileType::Text,
        Some(ref ext) if ["mp3", "wav", "ogg"].contains(&ext.as_str()) => FileType::Audio(ext.clone()),
        Some(ref ext) if ["mp4"].contains(&ext.as_str()) => FileType::Video(ext.clone()),
        _ => FileType::Unknown,
    }
}

/// Monitor clipboard in a loop
pub fn monitor_clipboard(json_path: &str) -> Result<(), Box<dyn std::error::Error>> {
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
                        // For files, just save the filename
                        let file_name = file_info.path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("Unknown file")
                            .to_string();
                        
                        // Determine file type for display
                        let display_text = match &file_info.file_type {
                            FileType::Image(_) => format!("🖼️  {}", file_name),
                            FileType::Pdf => format!("📄 {}", file_name),
                            FileType::Audio(_) => format!("🎵 {}", file_name),
                            FileType::Video(_) => format!("🎬 {}", file_name),
                            FileType::Text => format!("📝 {}", file_name),
                            FileType::Unknown => format!("📁 {}", file_name),
                        };
                        
                        println!("Detected file: {}", file_name);
                        
                        if let Err(e) = crate::storage::save_clipboard_entry(
                            "file",
                            Some(display_text),
                            None,
                            Some(file_info.path.to_string_lossy().to_string()),
                            json_path
                        ) {
                            eprintln!("Error saving file entry: {}", e);
                        }
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

