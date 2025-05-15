use colored::Colorize;
use tokio::sync::mpsc;

mod receive_requests;
use receive_requests::receive_http_requests;

mod transmit_request;
use transmit_request::forward_http_requests;


#[tokio::main]
async fn main() {

    if let Ok(listener) = tokio::net::TcpListener::bind("127.0.0.1:3000").await {
        println!("Listening on port {}", "3000".cyan());
        let (tx, rx) = mpsc::channel::<Vec<u8>>(32);
        

        tokio::spawn(async move {
            forward_http_requests(rx).await
        });

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
