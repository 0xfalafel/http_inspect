use std::error::Error;
use std::fmt::{self, Display};

use tokio::sync::mpsc::Receiver;
use colored_hexdump::hexdump;

#[derive(Debug)]
enum ProxyError {
    NoDestination,
}
impl Error for ProxyError {}

impl Display for ProxyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            Self::NoDestination => "Could not read destination in the first line of the HTTP request."
        };
        write!(f, "{}",msg)
    }
}

fn extract_host(req: &[u8]) -> Result<String, ProxyError> {
    let first_line = req.split(|b| *b == b'\n').nth(0)
        .ok_or(ProxyError::NoDestination)?;
    let first_line_decoded = String::from_utf8(first_line.to_vec())
        .map_err(|_| ProxyError::NoDestination)?;

    let destination = first_line_decoded.split_whitespace().nth(1)
        .ok_or(ProxyError::NoDestination)?;
    println!("{}", destination);
    
    Ok(destination.to_string())
}


pub async fn forward_http_requests(mut rx: Receiver<Vec<u8>>) {
    
    while let Some(http_request) = rx.recv().await {
        let hexdata = hexdump(&http_request);
        println!("{}", hexdata);

        let dest = match extract_host(&http_request) {
            Ok(host) => host,
            Err(_) => return,
        };

        // let http_request_str = String::from_utf8(http_request).unwrap();
        // println!("{}", http_request_str);
    }
}