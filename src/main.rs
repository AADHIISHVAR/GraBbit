#![feature(iter_array_chunks)]

use std::fs;
use std::io::{BufRead, BufReader, Write};
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

    let _img_path = "/home/aadhiishvar/Downloads/robin.jpg";
    convert_to_png(_img_path);
    
    let png_bytes = convert_to_png(_img_path);
    let comp_byte = byte_compression(&png_bytes.unwrap());

    let _enc_byte_vec = aes_gcm_encrypt(comp_byte.unwrap());
    // let dec_byte_vec = aes_gcm_decrypt(enc_byte_vec.unwrap());
    
    byte_decompression();

    let ear = TcpListener::bind("0.0.0.0:7878").unwrap();

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
    d.read_to_string(&mut s).unwrap();
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
    println!("{:?}",type_of(&key));
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