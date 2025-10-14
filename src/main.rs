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

    let json_file = "clipboard_history.json";
    clipboard_monitor_txtloop(json_file);
    
    let _img_path = "/home/aadhiishvar/Downloads/robin.jpg";
    convert_to_png(_img_path);
    
    let png_bytes = convert_to_png(_img_path);
    let comp_byte = byte_compression(&png_bytes.unwrap());

    let _enc_byte_vec = aes_gcm_encrypt(comp_byte.unwrap());
    // let dec_byte_vec = aes_gcm_decrypt(enc_byte_vec.unwrap());
    
    // byte_decompression();


    let _ipa:&str = "0.0.0.0:7878";
    let ear = TcpListener::bind(ipa).unwrap();

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
                if current_content != last_content {
                    save_clipboard_entry(&current_content, json_path)?;
                    last_content = current_content.clone();
                    println!("Saved clipboard: {}", &current_content);
                }
            }
            Err(_) => {
                // todo Clipboard might be empty or contain non-text; just skip
                println!("error in clipboard fn, might be in X or wayland , try again using a different one");
            }
        }

        thread::sleep(Duration::from_secs(1));
    }
}




