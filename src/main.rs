use colored::Colorize;
use colored_hexdump::hexdump;
use tokio::io::AsyncReadExt;

/// Extract an HTTP request from a byte slice
fn get_http_request(buffer: &[u8]) {
    let something = &buffer[0..4];

    // find the end of the HTTP header
    if let Some(end_header) = buffer.windows(4).position(|window| window == b"\x0d\x0a\x0d\x0a" ) {
        println!("end_header: {}", end_header);
        let header =&buffer[..end_header];
        get_content_length(header);

        // let hexdump = hexdump(header);
        // println!("{hexdump}");
    }

    let string = String::from_utf8_lossy(something);
    println!("{}", string);

}

/// Extract the Content-Length of an HTTP Request
/// `Content-Length: 24` will return 24
fn get_content_length(data: &[u8]) -> Option<usize>{
    let headers: Vec<&[u8]> = data.split(|&byte| byte == b'\n').collect();

    // For each header, we decode the bytes, and see if it's a Content-Length header
    for header_bytes in headers {

        // Header should be ASCII
        if let Ok(header) = String::from_utf8(header_bytes.to_vec()) {
            if header.to_lowercase().contains("content-length") {

                // Split and `:` and return the size as an `usize`
                if let Some(len) = header.split(':').nth(1) {
                    match usize::from_str_radix(len, 10) {
                        Ok(content_length) => return Some(content_length),
                        Err(_) => {},
                    }
                }
            }
        }
    }
    None
}


#[tokio::main]
async fn main() {

    if let Ok(listener) = tokio::net::TcpListener::bind("127.0.0.1:3000").await {
        println!("Listening on port {}", "3000".cyan());

        loop {
            if let Ok((mut socket, addr)) = listener.accept().await {

                println!("Accepted connection from {}", addr.to_string().green());

                let mut buffer = Vec::new();
                loop {
                    let mut temp_buffer = vec![0u8; 1024];

                    match socket.read(&mut temp_buffer).await {
                        Ok(0) => {
                            break;
                        }
                        Ok(n) => {
                            buffer.extend_from_slice(&temp_buffer[..n]);
                        }
                        Err(e) => {
                            eprintln!("Error reading from socket: {}", e);
                            break;
                        }
                    }
                    let hexdump = hexdump(&buffer);
                    println!("{}", hexdump);

                    get_http_request(&buffer);
                }                
            }
        }
    } else {
        eprintln!("{} to bind {}", "Failed".red(), "127.0.0.1:3000".red())
    }
}
