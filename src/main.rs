#![feature(iter_array_chunks)]

use aes_gcm::aead::rand_core::RngCore;
use std::fs;
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::io::Cursor;
use std::path::{Path, PathBuf};
use image::{ImageBuffer, ImageFormat, ImageReader};
use std::any::{type_name, Any};
use std::error::Error;
use image::DynamicImage;


fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

fn handle_client(mut stream: TcpStream) {
    let file: &str = "/home/aadhiishvar/Documents/rough-use/grabbit_proto_ui.html";
    match fs::read_to_string(file) {
        Ok(contents) => {
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                contents.len(),
                contents
            );
            use std::io::Write;
            let _ = stream.write_all(response.as_bytes());
        }
        Err(_) => {}
    }

} //streams the clip file

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    find_os();

    let json_file = "clipboard_history.json";

    // Start clipboard monitoring in background thread
    let json_file_clone = json_file.to_string();
    std::thread::spawn(move || {
        let _ = clipboard_monitor_txtloop(&json_file_clone);
    });

    // let _img_path = "/home/aadhiishvar/Downloads/robin.jpg";
    // convert_to_png(_img_path);
    //
    // let png_bytes = convert_to_png(_img_path);
    // let comp_byte = byte_compression(&png_bytes.unwrap());
    //
    // let _enc_byte_vec = aes_gcm_encrypt(comp_byte.unwrap());
    // let dec_byte_vec = aes_gcm_decrypt(enc_byte_vec.unwrap());

    // byte_decompression();




    let _ipa:&str = "0.0.0.0:7879";
    let ear = TcpListener::bind(_ipa)?;

    {
        for  stream in ear.incoming()
        {
            match stream {
                Ok(mut stream) => {
                    let peer = stream.peer_addr()?;
                    let mut reader = BufReader::new(&mut stream);

                    let mut l = String::new();
                    reader.read_line(&mut l)?;
                    // println!("Client Connected: {:?}", peer);
                    // println!("{}", l);

                    let mut request: Vec<_> = reader
                        .lines()
                        .map(|result| result.unwrap())
                        .take_while(|line| !line.is_empty())
                        .collect();
                    request.push(peer.to_string());
                    println!("{:#?}", request);
                    handle_client(stream);
                }
                Err(_) => {}
            }

        }
    }

    Ok(())
}


//it took me 1.5 hrs to write this fn (learned-> to look up the type of a variable and what kinda data that a fn returns, fr ), its too late
fn convert_to_png(img_path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    println!("jello");
    let image = ImageReader::open(img_path)?
        .with_guessed_format()?
        .decode()?;     // Decode to DynamicImage
    // image.save("rob_png.png");

    let image1 = ImageReader::open(img_path)?.with_guessed_format()?
        .decode()?;

    let _raw_byst = image1.into_bytes();
    println!("{} {}", _raw_byst.len(), " bytes of raw");

    let mut png_bytes: Vec<u8> = Vec::new();
    image.write_to(&mut Cursor::new(&mut png_bytes), ImageFormat::Png)?;

    println!("{} {}",png_bytes.len()," bytes of png");
    Ok(png_bytes)
}

// almost a day has gon understanding this shit and written on my own
use std::io::prelude::*;
use flate2::Compression;
use flate2::write::ZlibEncoder;
fn byte_compression(data: &Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
    // Create a buffer to store compressed bytes
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());

    // Write the input data into the encoder
    encoder.write_all(&data)?;

    // Finish compression and get the compressed Vec<u8>
    let compressed_bytes = encoder.finish()?;

    println!("Original size: {} bytes", data.len());
    println!("Compressed size: {} bytes", compressed_bytes.len());

    Ok(compressed_bytes) //send for encryption
}

use std::io::prelude::*;
use flate2::read::GzDecoder;
fn byte_decompression() -> Result<Vec<u8>, Box<dyn Error>>
{
    let mut d = GzDecoder::new("...".as_bytes());
    let mut s = String::new();
    d.read_to_string(&mut s).expect("Failed to read file");
    println!("{}", s);
    Ok(Vec::<u8>::new())
}

use aes_gcm::*;
use aes_gcm::aead::OsRng;
use aes_gcm::aes::Aes256;
use aes_gcm::Nonce;
use typenum::U12;
// <-- clean alias for GenericArray<u8, U12>

fn aes_gcm_encrypt(data: Vec<u8>) -> (Vec<u8>,( Vec<u8>, Nonce<U12>)) {
    let key = Aes256Gcm::generate_key(&mut OsRng);
    // println!("{:?}",type_of(&key));
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let mut buffer = data.clone();
    // Return (encrypted data, (key bytes, nonce))
    cipher.encrypt_in_place(&nonce, b"", &mut buffer).unwrap();

    (buffer, (key.to_vec(), nonce))
}


fn aes_gcm_decrypt(key_vec:Vec<u8>,buff_data:&mut Vec<u8>,nonce: Nonce<U12>) -> Result<Vec<u8>,Box<dyn Error>>
{
    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_vec);
    let cipher = Aes256Gcm::new(key);
    // 5. Decrypt back
    let mut buff_data_dec = buff_data.clone();

    cipher.decrypt_in_place(&nonce, b"", &mut buff_data_dec);
    // assert_eq!(buff_data_dec, buff_data); // Decryption should restore original
    Ok(buff_data_dec)
}

//TODO 3 convertion an per teh mode chosen , default which is WEBp,normal for jepg/jpg, and best is PNG -> write it as a fn
//TODO encrypt cpmpressed vecv


//todo save the copied txt in an json file, with the timestamp, device_name, device_user_name

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct ClipboardEntry {
    timestamp: String,
    device_name: String,
    os_name: String,
    user_name: String,
    content: String,
}


use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use chrono::Local;
use whoami;


fn save_clipboard_entry(new_content: &str, json_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut entries: Vec<ClipboardEntry> = if std::path::Path::new(json_path).exists() {
        let mut file = File::open(json_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        if content.is_empty() { vec![] } else { serde_json::from_str(&content)? }
    } else {
        vec![]
    };

    // Skip if the content is same as last entry
    if entries.last().map(|e| e.content.as_str()) == Some(new_content) {
        return Ok(());
    }

    let device_name = whoami::devicename_os();
    let user_name = whoami::username();
    let os_name = whoami::platform(); // get OS name

    use std::ffi::OsString;
    let entry = ClipboardEntry {
        timestamp: Local::now().to_rfc3339(),
        device_name: OsString::from(device_name).to_string_lossy().to_string(),
        os_name: os_name.to_string(),
        user_name,
        content: new_content.to_string(),
    };

    entries.push(entry);

    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(json_path)?;
    file.write_all(serde_json::to_string_pretty(&entries)?.as_bytes())?;

    Ok(())
}



use arboard::Clipboard;
use std::{thread, time::Duration};
use std::ffi::OsString;
use std::thread::current;

fn clipboard_monitor_txtloop(json_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut clipboard = Clipboard::new()?;
    let mut last_content = String::new();

    loop {
        match clipboard.get_text() {
            Ok(current_content) => {
                if current_content != last_content
                {
                    save_clipboard_entry(&current_content, json_path)?;
                    last_content = current_content.clone();
                    println!("Saved clipboard: {}", &current_content);

                    if let Some(file_info) = extract_path_from_clipboard(&current_content) {
                        // Hardcoded 32-byte hex key (64 hex characters = 32 bytes)
                        let key_hex = "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2";
                        if let Err(e) = process_file(&file_info.path, key_hex) {
                            eprintln!("Error processing file: {}", e);
                        }
                    }

                }
            }
            Err(_) => {
                // todo clipboard might be empty or contain non-text na -> just skip
                continue;
                // println!("error in clipboard fn, might be in X or wayland , try again using a different one");
            }
        }

        // thread::sleep(Duration::from_secs(1));
    }
}





use regex::Regex;
use url::Url;


/// Extracts a file name and extension from clipboard text if it contains any path or file URI.

/// define enum for file type
#[derive(Debug)]
enum FileType {
    Pdf,
    Image(String),   // png, jpg, etc.
    Text,
    Audio(String),
    Video(String),
    Unknown,
}
/// struct to hold the file info

/// extract path and determine type
#[derive(Debug)]
struct FileInfo {
    path: PathBuf,
    file_type: FileType,
}

/// extract path and determine type
fn extract_path_from_clipboard(text: &str) -> Option<FileInfo> {
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

/// detect file type from extension
fn detect_file_type(path: &Path) -> FileType {
    match path.extension().and_then(|e| e.to_str()).map(|s| s.to_lowercase()) {
        Some(ref ext) if ext == "pdf" => FileType::Pdf,
        Some(ref ext) if ["png", "jpg", "jpeg", "bmp", "gif"].contains(&ext.as_str()) => {
            FileType::Image(ext.clone())
        }
        Some(ref ext) if ext == "txt" => FileType::Text,
        Some(ref ext) if ["mp3", "wav", "ogg"].contains(&ext.as_str()) =>FileType::Audio(ext.clone()),
        Some(ref ext) if ["mp4"].contains(&ext.as_str()) =>FileType::Video(ext.clone()),

        // todo more type
        _ => FileType::Unknown,
    }
}


use sha2::{Sha256, Digest};
use zstd::Encoder;

use chacha20poly1305::{XChaCha20Poly1305, aead::{Aead, KeyInit}, XNonce};
use hex::encode as hex_encode;
use base64::{engine::general_purpose, Engine as _};

const CHUNK_SIZE: usize = 1024 * 1024; // 1 MiB

fn hex_key_to_bytes(s: &str) -> anyhow::Result<[u8;32]> {
    let v = hex::decode(s)?;
    if v.len() != 32 {
        println!("key must eb 32 bytes, {}",v.len());
        anyhow::bail!("key must be 32 bytes");
    }
    let mut a = [0u8;32];
    a.copy_from_slice(&v);
    println!("[DEBUG] Key bytes (hex): {}", hex_encode(&a));
    println!("[DEBUG] Key bytes (raw): {:?}", a);
    Ok(a)
}

fn compute_sha256(path: &Path) -> anyhow::Result<String> {
    let mut f = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 8192];

    println!("\n[HASH] Computing SHA256 for: {}", path.display());
    loop {
        let n = f.read(&mut buf)?;
        if n == 0 { break; }
        hasher.update(&buf[..n]);
    }

    let hash = hex_encode(hasher.finalize());
    println!("[HASH] SHA256: {}", hash);
    Ok(hash)
}

fn compress_file(src: &Path, dest: &Path) -> anyhow::Result<()> {
    println!("\n[COMPRESS] Starting compression...");
    println!("[COMPRESS] Source: {}", src.display());
    println!("[COMPRESS] Dest: {}", dest.display());

    let mut input = File::open(src)?;
    let output = File::create(dest)?;
    let mut encoder = Encoder::new(output, 3)?;

    let bytes_copied = std::io::copy(&mut input, &mut encoder)?;
    encoder.finish()?;

    let compressed_size = std::fs::metadata(dest)?.len();
    println!("[COMPRESS] Original size: {} bytes", bytes_copied);
    println!("[COMPRESS] Compressed size: {} bytes", compressed_size);
    println!("[COMPRESS] Compression ratio: {:.2}%", (compressed_size as f64 / bytes_copied as f64) * 100.0);

    Ok(())
}

fn encrypt_file(src: &Path, dest: &Path, key_bytes: &[u8;32]) -> anyhow::Result<()> {
    println!("\n[ENCRYPT] Initializing encryption...");
    println!("[ENCRYPT] Algorithm: XChaCha20-Poly1305");

    let cipher = XChaCha20Poly1305::new_from_slice(key_bytes)
        .map_err(|e| anyhow::anyhow!("Key error: {:?}", e))?;

    let mut input = File::open(src)?;
    let mut output = File::create(dest)?;
    let mut buf = vec![0u8; CHUNK_SIZE];
    let mut chunk_idx = 0;

    loop {
        let n = input.read(&mut buf)?;
        if n == 0 { break; }

        let plaintext = &buf[..n];

        // Generate random nonce
        let mut nonce_bytes = [0u8; 24];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = XNonce::from_slice(&nonce_bytes);

        println!("\n[ENCRYPT] === Chunk {} ===", chunk_idx);
        println!("[ENCRYPT] Plaintext size: {} bytes", n);
        println!("[ENCRYPT] Nonce (hex): {}", hex_encode(&nonce_bytes));
        println!("[ENCRYPT] Nonce (base64): {}", general_purpose::STANDARD.encode(&nonce_bytes));
        println!("[ENCRYPT] Plaintext preview (first 32 bytes): {:?}", &plaintext[..n.min(32)]);

        // Encrypt
        let ciphertext = cipher.encrypt(nonce, plaintext)
            .map_err(|e| anyhow::anyhow!("Encryption failed: {:?}", e))?;

        println!("[ENCRYPT] Ciphertext size: {} bytes (includes 16 byte auth tag)", ciphertext.len());
        println!("[ENCRYPT] Ciphertext (hex): {}", hex_encode(&ciphertext));
        println!("[ENCRYPT] Ciphertext (base64): {}", general_purpose::STANDARD.encode(&ciphertext));
        println!("[ENCRYPT] Ciphertext preview (first 32 bytes): {:?}", &ciphertext[..ciphertext.len().min(32)]);

        // Write nonce + ciphertext
        output.write_all(&nonce_bytes)?;
        output.write_all(&ciphertext)?;

        chunk_idx += 1;
    }

    output.flush()?;
    println!("\n[ENCRYPT] Total chunks encrypted: {}", chunk_idx);

    Ok(())
}

pub fn process_file(path: &Path, key_hex: &str) -> anyhow::Result<()> {
    println!("==============================================");
    println!("     FILE ENCRYPTION PIPELINE");
    println!("==============================================");
    println!("Input file: {}", path.display());

    println!("hi");
    // Parse key
    let key_bytes = hex_key_to_bytes(key_hex)?;
    println!("key hex done");

    // Step 1: Hash original file

    let _original_hash = compute_sha256(path)?;
    println!("sha done");
    // Step 2: Compress
    let temp_dir = std::env::temp_dir();
    let filename = path.file_name().unwrap().to_string_lossy();
    let compressed_path = temp_dir.join(format!("{}.zst", filename));
    compress_file(path, &compressed_path)?;
    println!("srep 2 completed");

    // Step 3: Encrypt
    let encrypted_path = temp_dir.join(format!("{}.enc", filename));
    encrypt_file(&compressed_path, &encrypted_path, &key_bytes)?;

    println!("step 3 done");

    // Step 4: Base64 encode final output
    println!("\n[BASE64] Encoding final encrypted file...");
    let mut final_data = Vec::new();
    File::open(&encrypted_path)?.read_to_end(&mut final_data)?;
    println!("step 4 done");

    let b64 = general_purpose::STANDARD.encode(&final_data);
    println!("[BASE64] Final size: {} bytes", final_data.len());
    println!("[BASE64] Base64 length: {} chars", b64.len());

    // println!("\n=== BASE64 OUTPUT START ===");
    // println!("{}", b64);
    // println!("=== BASE64 OUTPUT END ===");

    println!("\n[DONE] Temp files:");
    println!("  Compressed: {}", compressed_path.display());
    println!("  Encrypted: {}", encrypted_path.display());
    println!("==============================================");

    Ok(())
}

fn find_os() -> Result<(),Box<dyn Error>>
{
    println!("Detecting OS and fetching clipboard data...\n");
    match std::env::consts::OS {
        "windows" => println!("the code base is working or WINDOWS based system"),
        "macos" => println!("the code base is working or MAC OS based system"),
        "linux" => println!("the code base is working or linux based system"),
        other => println!("Unsupported OS: {}", other),
    }
    Ok(())
}

//use the ai tools wiselt , i did some RnD abt the teckstack b4 cv on it , i aint cokmpleetely depend in a ai



