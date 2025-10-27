use arboard::Clipboard;
use std::{thread, time::Duration};
use std::path::{Path, PathBuf};
use std::error::Error;
use regex::Regex;
use url::Url;
use base64::Engine;
use image::{ImageReader, ImageFormat};
use std::io::Cursor;

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

/// Check if text is a file path
pub fn is_file_path(text: &str) -> bool {
    let path = Path::new(text);
    path.exists() && path.is_file()
}

/// Compress file based on type and quality setting
pub fn compress_file(path: &Path, quality: crate::gui::app::CompressionQuality) -> Result<String, Box<dyn Error>> {
    // Check if it's an image
    if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
        match extension.to_lowercase().as_str() {
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" => {
                return compress_image(path, quality);
            }
            _ => {
                // For non-image files, just read and compress with zstd
                let file_data = std::fs::read(path)?;
                let compressed = crate::security::compress_data(&file_data)?;
                return Ok(base64::engine::general_purpose::STANDARD.encode(&compressed));
            }
        }
    }
    
    // Fallback: read file and compress
    let file_data = std::fs::read(path)?;
    let compressed = crate::security::compress_data(&file_data)?;
    Ok(base64::engine::general_purpose::STANDARD.encode(&compressed))
}

/// Compress and encode image based on quality setting with smart format selection
pub fn compress_image(path: &Path, quality: crate::gui::app::CompressionQuality) -> Result<String, Box<dyn Error>> {
    let img = ImageReader::open(path)?.decode()?;
    
    let mut buffer = Vec::new();
    let format = match quality {
        crate::gui::app::CompressionQuality::Default => {
            // Use original format for default
            if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
                match extension.to_lowercase().as_str() {
                    "png" => ImageFormat::Png,
                    "jpg" | "jpeg" => ImageFormat::Jpeg,
                    "webp" => ImageFormat::WebP,
                    _ => ImageFormat::Png, // Default fallback
                }
            } else {
                ImageFormat::Png
            }
        },
        crate::gui::app::CompressionQuality::Normal => ImageFormat::Png,
        crate::gui::app::CompressionQuality::High => ImageFormat::Jpeg,
    };
    
    img.write_to(&mut Cursor::new(&mut buffer), format)?;
    
    // Base64 encode the compressed image
    Ok(base64::engine::general_purpose::STANDARD.encode(&buffer))
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
                        let file_name = file_info.path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("Unknown file")
                            .to_string();
                        
                        // Handle different file types
                        match &file_info.file_type {
                            FileType::Image(_) => {
                                // Compress and encode image
                                match compress_image(&file_info.path, crate::gui::app::CompressionQuality::Default) {
                                    Ok(encoded_data) => {
                                        let display_text = format!("🖼️  {}", file_name);
                                        if let Err(e) = crate::storage::save_clipboard_entry(
                                            "image",
                                            Some(display_text),
                                            Some(encoded_data),
                                            Some(file_info.path.to_string_lossy().to_string()),
                                            json_path
                                        ) {
                                            eprintln!("Error saving image entry: {}", e);
                                        }
                                    }
                                    Err(e) => eprintln!("Error compressing image: {}", e),
                                }
                            }
                            FileType::Pdf | FileType::Document(_) => {
                                let display_text = match &file_info.file_type {
                                    FileType::Pdf => format!("📄 {}", file_name),
                                    FileType::Document(_) => format!("📝 {}", file_name),
                                    _ => format!("📁 {}", file_name),
                                };
                                
                                if let Err(e) = crate::storage::save_clipboard_entry(
                                    "document",
                                    Some(display_text),
                                    None,
                                    Some(file_info.path.to_string_lossy().to_string()),
                                    json_path
                                ) {
                                    eprintln!("Error saving document entry: {}", e);
                                }
                            }
                            FileType::Audio(_) | FileType::Video(_) => {
                                let display_text = match &file_info.file_type {
                                    FileType::Audio(_) => format!("🎵 {}", file_name),
                                    FileType::Video(_) => format!("🎬 {}", file_name),
                                    _ => format!("📁 {}", file_name),
                                };
                                
                                if let Err(e) = crate::storage::save_clipboard_entry(
                                    "media",
                                    Some(display_text),
                                    None,
                                    Some(file_info.path.to_string_lossy().to_string()),
                                    json_path
                                ) {
                                    eprintln!("Error saving media entry: {}", e);
                                }
                            }
                            _ => {
                                let display_text = format!("📁 {}", file_name);
                                if let Err(e) = crate::storage::save_clipboard_entry(
                                    "file",
                                    Some(display_text),
                                    None,
                                    Some(file_info.path.to_string_lossy().to_string()),
                                    json_path
                                ) {
                                    eprintln!("Error saving file entry: {}", e);
                                }
                            }
                        }
                        
                        println!("Detected file: {}", file_name);
                    } else {
                        // Check if text is a file path
                        if is_file_path(&current_content) {
                            let path = Path::new(&current_content);
                            let file_name = path.file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("Unknown file")
                                .to_string();
                            
                            // Compress the file
                            match compress_file(path, crate::gui::app::CompressionQuality::Default) {
                                Ok(encoded_data) => {
                                    let display_text = format!("📁 {}", file_name);
                                    if let Err(e) = crate::storage::save_clipboard_entry(
                                        "file",
                                        Some(display_text),
                                        Some(encoded_data),
                                        Some(current_content.clone()),
                                        json_path
                                    ) {
                                        eprintln!("Error saving file entry: {}", e);
                                    }
                                    println!("Detected and compressed file: {}", file_name);
                                }
                                Err(e) => {
                                    eprintln!("Error compressing file: {}", e);
                                    // Fallback to plain text
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
            }
            Err(_) => {
                // todo clipboard might be empty or contain non-text na -> just skip
                continue;
            }
        }

        thread::sleep(Duration::from_secs(1));
    }
}

