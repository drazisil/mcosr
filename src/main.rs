use std::{net::{TcpListener, TcpStream}, io::Read};
use hex::encode;

pub struct ProtocolVersion {
    major: u8,
    minor: u8,
}


fn handle_client(mut stream: TcpStream) {
    let mut buf = vec![0; 512];
    match  stream.read(&mut buf) {
        Ok(n) => {
            println!("{} bytes read", n);
            let first_byte: u8 = buf[0];
            let second_byte: u8 = buf[1];

            let tls_type;
            
            if first_byte == 0x00 || second_byte > 0x03 {
                tls_type = "SSL";
            } else {
                tls_type = "TLS";
            }

            println!("TLS type: {}", tls_type);

            let header_length: u8;
            
            if first_byte == 0x00 {
                header_length = 2;
            } else {
                header_length = 3;
            }
            println!("Message header is {} bytes long", header_length);

            let message_length: u16 = ((first_byte as u16 & 0x3f) << 8) 
            | (second_byte as u16);

            println!("Message length is {} bytes", message_length);

            println!("Request: {}", encode(&buf[..]));
        },
        Err(e) => println!("Error: {}", e),      
    }
}

fn main() -> std::io::Result<()> {
    const CURRENT_VERSION: ProtocolVersion = ProtocolVersion { major: 1, minor: 1 };
    let listener = TcpListener::bind("0.0.0.0:443")?;

    println!("Server listening on port 443");
    println!("Current version: {}.{}", CURRENT_VERSION.major, CURRENT_VERSION.minor);
    
    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}