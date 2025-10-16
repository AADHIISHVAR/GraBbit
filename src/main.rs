#![feature(iter_array_chunks)]

use std::fs;
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::io::Cursor;
use std::path::Path;
use image::{ImageBuffer, ImageFormat, ImageReader};
use std::any::{type_name, Any};
use std::error::Error;
use image::DynamicImage;


fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

fn handle_client(mut stream: TcpStream) {
    let file: &str = "/home/aadhiishvar/Documents/rough-use/grabbit.txt";
    match fs::read_to_string(file) {
        Ok(contents) => {
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                contents.len(),
                contents
            );
            stream.write_all(response.as_bytes()).unwrap();
        }
        Err(_) => {}
    }

} //streams the clip file 

fn main() {
    println!("Hello, world!");
    find_os();

    let json_file = "clipboard_history.json";
    clipboard_monitor_txtloop(json_file);
    

    // let _img_path = "/home/aadhiishvar/Downloads/robin.jpg";
    // convert_to_png(_img_path);
    //
    // let png_bytes = convert_to_png(_img_path);
    // let comp_byte = byte_compression(&png_bytes.unwrap());
    //
    // let _enc_byte_vec = aes_gcm_encrypt(comp_byte.unwrap());
    // let dec_byte_vec = aes_gcm_decrypt(enc_byte_vec.unwrap());

    // byte_decompression();

    
    

    let _ipa:&str = "0.0.0.0:7878";
    let ear = TcpListener::bind(_ipa).unwrap();

    {
        for  stream in ear.incoming()
        {
            match stream {
                Ok(mut stream) => {
                    let peer = stream.peer_addr().unwrap();
                    let mut reader = BufReader::new(&mut stream);

                    let mut l = String::new();
                    reader.read_line(&mut l).unwrap();
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
                   println!("{:?}",extract_path_from_clipboard(&last_content));
                }
            }
            Err(_) => {
                // todo clipboard might be empty or contain non-text na -> just skip
                continue;
                println!("error in clipboard fn, might be in X or wayland , try again using a different one");
            }
        }

        // thread::sleep(Duration::from_secs(1));
    }
}


// a big buble that gona make our project pop

// fn extract_all_data_form_clip() ->Result<(),Box<dyn Error>> 
// {
// 
//     // Try the simple cross-platform checks using `arboard`.
//     // arboard can return text or an Image (RGBA + width/height).
//     //
//     // This will work on Windows, macOS, and common Linux setups (X11; for Wayland you
//     // might need a different backend).
//     let mut clipboard = arboard::Clipboard::new().map_err(|e| format!("open clipboard: {e}"))?;
// 
//     // 1) Try text
//     match clipboard.get_text() {
//         Ok(text) => {
//             println!("clipboard has text ({} bytes):\n{}\n", text.len(), text);
//         }
//         Err(_) => {
//             println!("no plain text or couldn't get text.");
//         }
//     }
// 
//     // 2) Try image
//     match clipboard.get_image() {
//         Ok(img) => {
//             println!(
//                 "clipboard has image: {}x{}, {} bytes (RGBA)",
//                 img.width,
//                 img.height,
//                 img.bytes.len()
//             );
//             // example: save to file for inspection
//             // convert RGBA to PNG using 'image' crate if you want to persist it:
//             // use image::{ImageBuffer, Rgba};
//             // let buf: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(img.width, img.height, img.bytes).unwrap();
//             // buf.save("pasted.png").unwrap();
//         }
//         Err(_) => println!("no image or couldn't get image."),
//     }
// 
//     // 3) If you want *every* format the OS offers, you must branch by OS and call
//     // platform APIs (see the notes below). We'll print a hint and return.
//     println!("\nTo enumerate all formats (detailed list) -> implement platform-specific enumeration.");
// 
//     Ok(())
// }


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
#[derive(Debug)]
struct FileInfo {
    name: String,
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
                            name: path.file_name()?.to_string_lossy().to_string(),
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
        let path = Path::new(path_str);
        return Some(FileInfo {
            name: path.file_name()?.to_string_lossy().to_string(),
            file_type: detect_file_type(path),
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







