
pub fn find_header<'a>(http_request: &'a [u8], header: &'a str) -> Option<&'a [u8]> {
    let header = header.as_bytes();

    if let Some(header_start) = http_request.windows(header.len()).position(|window| window == header) {
        
        if let Some(header_end) = http_request[header_start..].windows(2).position(|window| window == b"\r\n") {
            return Some(&http_request[header_start..header_start + header_end]);
        } else if let Some(header_end) = http_request[header_start..].windows(1).position(|window| window == b"\n") {
            return Some(&http_request[header_start..header_start + header_end]);
        }
    }
    None
}

/// Return the value of a Header. I.e
/// Host: example.com -> example.com
pub fn get_header<'a>(http_request: & [u8], header: & str) -> Option<String> {
    if let Some(header_bytes) = find_header(http_request, header) {
        // Split on the first ':'
        let mut parts = header_bytes.splitn(2, |byte| *byte == b':');
        parts.next(); // Skip the header title

        // Convert the header value to String
        if let Some (value_bytes) = parts.next() {
            let value = String::from_utf8_lossy(value_bytes);
            let value = value.strip_prefix(" ").unwrap_or(&value);
            return  Some(value.to_string());
        }
    }

    None
}