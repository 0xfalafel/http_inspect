use std::error::Error;
use std::fmt::{self, Display};

use tokio::sync::mpsc::Receiver;
use colored_hexdump::hexdump;

use crate::http_utils::{find_header, get_header, remove_header};

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

fn extract_destination(req: &[u8]) -> Result<String, ProxyError> {
    // We try to see if there is a destination in the first line.
    // When contacting proxy, the first line will have the destination, like
    // `GET http://google.com/whoami/pwn HTTP/1.1` instead of the classic `GET /whoami/pwd HTTP/1.1`
    let first_line = req.split(|b| *b == b'\n').nth(0)
        .ok_or(ProxyError::NoDestination)?;
    let first_line_decoded = String::from_utf8(first_line.to_vec())
        .map_err(|_| ProxyError::NoDestination)?;

    let destination = first_line_decoded.split_whitespace().nth(1)
        .ok_or(ProxyError::NoDestination)?;

    // Remove the path at the end
    // http://google.com/whoami/pwd -> http://google.com
    let scheme_end = destination.find("://").map(|pos| pos +3).unwrap_or(0);
    let path_start = destination[scheme_end..].find('/').map(|pos| pos+scheme_end).unwrap_or(destination.len());

    let destination = &destination[..path_start];

    // If the app doesn't know it's talking to a proxy (transaprent proxy), i.e `GET /whoami/pwd HTTP/1.1`
    // We still need a destination (to contact the remote server). Let's extract the destination from the
    // Host header
    if destination.is_empty() {
        match get_header(&req, "Host") {
            Some(dest) => return Ok(dest),
            None => return Err(ProxyError::NoDestination),
        }
    } else {
        Ok(destination.to_string())
    }
}

/// Remove the destination in the first line of the request. 
/// GET http://google.com/whoami HTTP/1.1 become
/// GET /whoami HTTP/1.1
fn replace_proxy_destination(mut http_request: Vec<u8>, destination: &str) -> Vec<u8> {
    let proxy_destination = destination.as_bytes();

    if let Some(pos) = http_request.windows(proxy_destination.len()).position(|window| window == proxy_destination) {
        http_request.drain(pos..pos+proxy_destination.len());
    }

    http_request
}

enum Scheme {
    Http,
    Https,
}

/// http://google.com:3400 -> google.com 3400
fn extract_host(destination: &str) -> (Scheme, String) {
    // Handle the http(s) at the begining of `destination`
    // It might not be present
    let scheme = match destination {
        destination if destination.starts_with("http://") => Some(Scheme::Http),
        destination if destination.starts_with("https://") => Some(Scheme::Https),
        _ => None
    };

    let destination = match scheme {
        Some(Scheme::Http) => &destination[7..],
        Some(Scheme::Https) => &destination[8..],
        _ => destination,
    };

    // Remove path if present
    // There is already no path
    // let destination = destination.split('/').nth(0).unwrap_or(destination);
    
    let scheme = scheme.unwrap_or(Scheme::Http);
    
    // we need a port to connect to. Precise one if there are none specified
    let mut host = destination.to_owned();
    if !host.contains(':') {
        match scheme {
            Scheme::Http => host.push_str(":80"),
            Scheme::Https => host.push_str(":443"),
        }
    }

    (Scheme::Http, host)
}

pub async fn forward_http_requests(mut rx: Receiver<Vec<u8>>) {
    
    while let Some(http_request) = rx.recv().await {
        let hexdata = hexdump(&http_request);
        println!("{}", hexdata);
        
        let destination = match extract_destination(&http_request) {
            Ok(host) => host,
            Err(_) => return,
        };
        
        println!("Destination: {}", destination);
        
        let http_request = replace_proxy_destination(http_request, &destination);
        let http_request = remove_header(http_request, "Proxy-Connection");

        let (scheme, host) = extract_host(&destination);


        
        println!("Request to send:");
        let http_request_str = String::from_utf8(http_request).unwrap();
        println!("{}", http_request_str);
    }
}