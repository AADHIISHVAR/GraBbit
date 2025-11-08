use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
use std::fs;
use std::error::Error;

/// Handle incoming client connection
pub fn handle_client(mut stream: TcpStream) {
    let file: &str = "/home/aadhiishvar/Documents/rough-use/grabbit_proto_ui.html";
    match fs::read_to_string(file) {
        Ok(contents) => {
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                contents.len(),
                contents
            );
            let _ = stream.write_all(response.as_bytes());
        }
        Err(_) => {}
    }
}

/// Start TCP server
pub fn start_server() -> Result<(), Box<dyn Error + Send + Sync>> {
    let ip = "0.0.0.0:7879";
    let listener = TcpListener::bind(ip)?;
    
    println!("Server listening on {}", ip);
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let peer = stream.peer_addr()?;
                let mut reader = BufReader::new(&mut stream);

                let mut l = String::new();
                reader.read_line(&mut l)?;

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

    Ok(())
}

/// Get local IP address
pub fn get_local_ip() -> Option<String> {
    // This does NOT actually connect to the internet, aslo need interned connection to the f wifi to fetch the local ip lol, need to figure a way beter that this pice of AI shit 
    if let Ok(socket) = std::net::UdpSocket::bind("0.0.0.0:0") {
        if socket.connect("8.8.8.8:80").is_ok() {
            if let Ok(local_addr) = socket.local_addr() {
                println!("{:?}",local_addr.ip().to_string());
                return Some(local_addr.ip().to_string());
            }
        }
    }

    None
}

/// Detect OS
pub fn detect_os() -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("Detecting OS and fetching clipboard data...\n");
    match std::env::consts::OS {
        "windows" => println!("the code base is working or WINDOWS based system"),
        "macos" => println!("the code base is working or MAC OS based system"),
        "linux" => println!("the code base is working or linux based system"),
        other => println!("Unsupported OS: {}", other),
    }
    Ok(())
}


