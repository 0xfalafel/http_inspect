use colored::Colorize;

mod receive_requests;
use receive_requests::receive_http_requests;
use tokio::sync::mpsc;


#[tokio::main]
async fn main() {

    if let Ok(listener) = tokio::net::TcpListener::bind("127.0.0.1:3000").await {
        println!("Listening on port {}", "3000".cyan());
        let (tx, mut _rx) = mpsc::channel::<Vec<u8>>(32);
        
        loop {
            if let Ok((socket, addr)) = listener.accept().await {
                println!("Accepted connection from {}", addr.to_string().green());

                let tx = tx.clone();
                tokio::spawn(async move {
                    receive_http_requests(socket, tx).await
                });

                
            }
        }
    } else {
        eprintln!("{} to bind {}", "Failed".red(), "127.0.0.1:3000".red())
    }
}
