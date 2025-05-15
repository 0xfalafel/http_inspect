use tokio::sync::mpsc::Receiver;
use colored_hexdump::hexdump;


pub async fn forward_http_requests(mut rx: Receiver<Vec<u8>>) {
    
    while let Some(http_request) = rx.recv().await {
        let hexdata = hexdump(&http_request);
        println!("{}", hexdata);
    }
}