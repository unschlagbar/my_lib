


pub struct HTTPS {
    pub http_verion: Option<String>,
    pub host: Option<String>,
    pub status: Option<String>
}

impl HTTPS {
    pub fn format(content_type: &[u8], content: &[u8]) -> Vec<u8> {
        let content_len = content.len();
        let mut format = Vec::with_capacity(content_len + 100);
        format.extend_from_slice(b"HTTP/1.1 200 OK\r\n");
        format.extend_from_slice(b"Content-Type: ");
        format.extend_from_slice(content_type);
        format.extend_from_slice(b"\r\ncharset=UTF-8\r\nContent-Length: ");
        format.extend_from_slice(content_len.to_string().as_bytes());
        format.extend_from_slice(b"\r\n\r\n");
        format.extend_from_slice(content);
        format
    }
}