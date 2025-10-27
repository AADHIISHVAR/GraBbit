use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::error::Error;
use zstd::Encoder;
use chacha20poly1305::{XChaCha20Poly1305, aead::{Aead, KeyInit}, XNonce};
use rand::RngCore;
use base64::{engine::general_purpose, Engine as _};

const CHUNK_SIZE: usize = 1024 * 1024; // 1 MiB

/// Compress data using zstd
pub fn compress_data(data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut encoder = Encoder::new(Vec::new(), 3)?;
    encoder.write_all(data)?;
    encoder.finish().map_err(|e| e.into())
}

/// Convert hex string to 32-byte array
pub fn hex_key_to_bytes(s: &str) -> Result<[u8;32], Box<dyn Error>> {
    let v = hex::decode(s)?;
    if v.len() != 32 {
        return Err("key must be 32 bytes".into());
    }
    let mut a = [0u8;32];
    a.copy_from_slice(&v);
    Ok(a)
}

/// Process file: compress + encrypt + base64 encode  
pub fn process_file(path: &Path, key_hex: &str) -> Result<String, Box<dyn Error>> {
    println!("[SECURITY] Processing file: {}", path.display());
    
    // Parse key
    let key_bytes = hex_key_to_bytes(key_hex)?;
    
    // Step 1: Compress
    let temp_dir = std::env::temp_dir();
    let filename = path.file_name().unwrap().to_string_lossy();
    let compressed_path = temp_dir.join(format!("{}.zst", filename));
    
    let mut input = File::open(path)?;
    let output = File::create(&compressed_path)?;
    let mut encoder = Encoder::new(output, 3)?;
    std::io::copy(&mut input, &mut encoder)?;
    encoder.finish()?;
    
    // Step 2: Encrypt
    let encrypted_path = temp_dir.join(format!("{}.enc", filename));
    let cipher = XChaCha20Poly1305::new_from_slice(&key_bytes)
        .map_err(|e| format!("Key error: {:?}", e))?;
    
    let mut input = File::open(&compressed_path)?;
    let mut output = File::create(&encrypted_path)?;
    let mut buf = vec![0u8; CHUNK_SIZE];
    
    loop {
        let n = input.read(&mut buf)?;
        if n == 0 { break; }
        
        let plaintext = &buf[..n];
        
        // Generate random nonce
        let mut nonce_bytes = [0u8; 24];
        rand::rng().fill_bytes(&mut nonce_bytes);
        let nonce = XNonce::from_slice(&nonce_bytes);
        
        // Encrypt
        let ciphertext = cipher.encrypt(nonce, plaintext)
            .map_err(|e| format!("Encryption failed: {:?}", e))?;
        
        // Write nonce + ciphertext
        output.write_all(&nonce_bytes)?;
        output.write_all(&ciphertext)?;
    }
    
    output.flush()?;
    
    // Step 3: Base64 encode final output
    let mut final_data = Vec::new();
    File::open(&encrypted_path)?.read_to_end(&mut final_data)?;
    let b64 = general_purpose::STANDARD.encode(&final_data);
    
    println!("[SECURITY] File processed successfully");
    
    Ok(b64)
}
