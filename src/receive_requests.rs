use colored_hexdump::hexdump;
use tokio::io::ReadHalf;
use tokio::{io::AsyncReadExt, net::TcpStream};
use tokio::sync::mpsc::Sender;

/// This function tells us if we have a complete HTTP request in the buffer.
/// * If it return None, the request is incomplete, and we should wait for the end of the request.
/// * If it returns Some(len), we can extract a request of size `len` from the buffer.
///
/// This function doesn't check if the HTTP is correct, it just use '\n\r\n\r' and the Content-Length header
fn http_request_size(buffer: &[u8]) -> Option<usize> {
    // find the end of the HTTP header
    if let Some(end_header) = buffer.windows(4).position(|window| window == b"\x0d\x0a\x0d\x0a" ) {
        // println!("end_header: 0x{:x}", end_header);
        let header =&buffer[..end_header];

        if let Some(content_length) = get_content_length(header) {
            // println!("Content-Length: {}", content_length);
            let end_req_with_data = end_header + 4 + content_length;

            if buffer.len() >= end_req_with_data {
                return Some(end_req_with_data)
            }
        } else {
            return Some(end_header+4)
        }
    }
    
    None
}

/// Extract the Content-Length of an HTTP header
/// `Content-Length: 24` will return 24
fn get_content_length(data: &[u8]) -> Option<usize>{

    // We take bytes as input, and look at each line, to make sure our function
    // doesn't fail if there are non-utf8 chars in the header
    let headers: Vec<&[u8]> = data.split(|&byte| byte == b'\n').collect();

    // For each header, we decode the bytes, and see if it's a Content-Length header
    for header_bytes in headers {

        // Content-Length should be ASCII
        if let Ok(header) = String::from_utf8(header_bytes.to_vec()) {
            if header.to_lowercase().contains("content-length") {

                // Split and `:` and return the size as an `usize`
                if let Some(len) = header.split(':').nth(1) {

                    match usize::from_str_radix(len.trim(), 10) {
                        Ok(content_length) => return Some(content_length),
                        Err(_) => {},
                    }
                }
            }
        }
    }
    None
}

pub async fn receive_http_requests(mut socket: ReadHalf<TcpStream>, tx: Sender<Vec<u8>>) {
    let mut buffer: Vec<u8> = Vec::new();
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

        while let Some(end_of_request) = http_request_size(&buffer) {
            let req = &buffer[..end_of_request];

            // push the data somewhere
            let res = tx.send(req.to_vec()).await;
            if let Err(e) = res {
                eprintln!("Failed to transmit http request: {}", e);
                eprintln!("{}", hexdump(req));
            }

            // Remove the request we have consumed from the buffer
            buffer = buffer.drain(end_of_request..).collect();
        }
    }                

}

