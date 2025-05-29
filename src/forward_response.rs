use tokio::io::{AsyncWriteExt, WriteHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc::Receiver;

use crate::transmit_request::ProxyError;

pub async fn forward_response(mut rx: Receiver<Vec<u8>>, mut socket: WriteHalf<TcpStream>) -> Result<(), ProxyError> {

    while let Some(http_request) = rx.recv().await {
        match socket.write_all(&http_request).await {
            Ok(()) => {},
            Err(_) => return Err(ProxyError::FailedToWriteToListener),
        }
    }
    Ok(())
}
