use std::{fs, path::PathBuf};




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

    pub fn format_content(path: PathBuf) -> Option<Vec<u8>> {
        let file_content = fs::read(&path);
        
        match file_content {
            Ok(content) => {
                let content_type: &[u8] = match path.extension() {
                    Some(extention) => match extention.as_encoded_bytes() {
                        b"apng" => b"image/apng",
                        b"png" => b"image/png",
                        b"webp" => b"image/webp",
                        b"gif" => b"image/gif",
                        b"jpeg" => b"image/jpeg",
                        b"svh" => b"image/svg+xml",
                        b"avif" => b"image/avif",
                        b"zip" => b"application/zip",
                        b"json" => b"text/json",
                        b"js" => b"text/javascript",
                        b"html" => b"text/html",
                        b"pdf" => b"application/pdf",
                        b"mp3" => b"audio/mpeg",
                        b"mp4" => b"audio/mp4",
                        b"ogg" => b"audio/ogg",
                        b"wav" => b"audio/wav",
                        _ => b"text/plain"
                    }
                    None => b"text/html",
                };
                return Some(HTTPS::format(content_type, &content));
            }
            Err(_) => return None,
        }
    }
}