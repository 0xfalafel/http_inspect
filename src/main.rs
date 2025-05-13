use colored::Colorize;
use colored_hexdump::hexdump;
use tokio::io::AsyncReadExt;

fn get_http_request(buffer: &[u8]) {
    let something = &buffer[0..4];

    let string = String::from_utf8_lossy(something);
    println!("{}", string);

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
                        Ok(n) if n == 0 => {
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
    }
}
