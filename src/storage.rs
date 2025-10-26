use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use chrono::Local;
use whoami;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClipboardEntry {
    pub timestamp: String,
    pub device_name: String,
    pub os_name: String,
    pub user_name: String,
    pub content_type: String,
    pub text_content: Option<String>,
    pub encrypted_data: Option<String>,
    pub file_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClipboardItem {
    pub content: String,
    pub device: String,
    pub os: String,
    pub time: String,
    pub user: String,
}

impl From<ClipboardEntry> for ClipboardItem {
    fn from(entry: ClipboardEntry) -> Self {
        let content = entry.text_content
            .clone()
            .unwrap_or_else(|| "File data".to_string());
        
        let time = entry.timestamp
            .split('T')
            .next()
            .unwrap_or(&entry.timestamp)
            .to_string();
        
        ClipboardItem {
            content,
            device: entry.device_name,
            os: entry.os_name,
            time,
            user: entry.user_name,
        }
    }
}

pub fn save_clipboard_entry(
    content_type: &str,
    text_content: Option<String>,
    encrypted_data: Option<String>,
    file_path: Option<String>,
    json_path: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let mut entries: Vec<ClipboardEntry> = if std::path::Path::new(json_path).exists() {
        let mut file = File::open(json_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        if content.is_empty() { vec![] } else { serde_json::from_str(&content)? }
    } else {
        vec![]
    };

    let device_name = whoami::devicename_os();
    let user_name = whoami::username();
    let os_name = whoami::platform(); // get OS name

    use std::ffi::OsString;
    let entry = ClipboardEntry {
        timestamp: Local::now().to_rfc3339(),
        device_name: OsString::from(device_name).to_string_lossy().to_string(),
        os_name: os_name.to_string(),
        user_name,
        content_type: content_type.to_string(),
        text_content,
        encrypted_data,
        file_path,
    };

    entries.push(entry);

    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(json_path)?;
    file.write_all(serde_json::to_string_pretty(&entries)?.as_bytes())?;

    Ok(())
}

pub fn load_clipboard_history() -> Result<Vec<ClipboardItem>, Box<dyn Error>> {
    let json_path = "clipboard_history.json";
    
    if !std::path::Path::new(json_path).exists() {
        return Ok(Vec::new());
    }
    
    let mut file = File::open(json_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    
    if content.is_empty() {
        return Ok(Vec::new());
    }
    
    let entries: Vec<ClipboardEntry> = serde_json::from_str(&content)?;
    let items: Vec<ClipboardItem> = entries.into_iter().map(Into::into).collect();
    
    Ok(items)
}

pub fn clear_clipboard_history() -> Result<(), Box<dyn Error>> {
    let json_path = "clipboard_history.json";
    
    if std::path::Path::new(json_path).exists() {
        std::fs::remove_file(json_path)?;
    }
    
    Ok(())
}

