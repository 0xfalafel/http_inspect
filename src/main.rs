use colored_hexdump::hexdump;
use tokio::io::AsyncReadExt;


#[tokio::main]
async fn main() {

    if let Ok(listener) = tokio::net::TcpListener::bind("127.0.0.1:3000").await {
        println!("Listening on port 3000");

        loop {
            if let Ok((mut socket, addr)) = listener.accept().await {

                println!("Accepted connection from {}", addr);

                let mut buffer = Vec::new();
                loop {
                    let mut temp_buffer = vec![0u8; 1024];

                    match socket.read(&mut temp_buffer).await {
                        Ok(n) if n == 0 => {
                            let hexdump = hexdump(&buffer);
                            println!("{}", hexdump);
                            break;
                        }
                        Ok(n) => {
                            buffer.extend_from_slice(&temp_buffer[..n]);
                        }
                        Err(e) => {
                            eprintln!("Error reading from socket: {}", e);
                            let hexdump = hexdump(&buffer);
                            println!("{}", hexdump);
                            break;
                        }
                    }

                }                
            }
        }
    }
}
