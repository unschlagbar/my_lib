use std::{io::{Read, Write}, net::{SocketAddr, TcpStream}, sync::{Arc, Mutex}, thread::JoinHandle, u16};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use sha1_smol::Sha1;

#[derive(Debug)]
pub struct WebSocket {
    stream: TcpStream,
    thread_handle: Option<JoinHandle<()>>,
}

#[allow(dead_code)]
impl WebSocket {

    pub fn try_connect<F, C>(mut stream: TcpStream, handshake_key: &str, onmessage: F, onclose: C) -> Option<Arc<Mutex<Self>>> 
        where F: Fn(Arc<Mutex<WebSocket>>, Vec<u8>) + Send + 'static, C: Fn(SocketAddr) + Send + 'static {

        let mut sha = Sha1::new();
        sha.update(handshake_key.as_bytes());
        sha.update(b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
        let umbrella = sha.digest().bytes();
        let mut encoded_key = [0; 28];
        STANDARD.encode_slice(umbrella, &mut encoded_key).unwrap();

        let response = format!("HTTP/1.1 101 Switching Protocols\r\nConnection: Upgrade\r\nUpgrade: websocket\r\nSec-WebSocket-Accept: {}\r\n\r\n", unsafe {String::from_utf8_unchecked(encoded_key.to_vec())});
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

        let ws = Arc::new(Mutex::new(WebSocket { stream: stream.try_clone().unwrap(), thread_handle: None }));
        ws.lock().unwrap().thread_handle = Some(WebSocket::run(ws.clone(), stream, onmessage, onclose));
        Some(ws)
    }

    pub fn new<F, C>(stream: TcpStream, onmessage: F, onclose: C) -> Arc<Mutex<Self>>
        where F: Fn(Arc<Mutex<WebSocket>>, Vec<u8>) + Send + 'static, C: Fn(SocketAddr) + Send + 'static {

            let ws = Arc::new(Mutex::new(WebSocket { stream: stream.try_clone().unwrap(), thread_handle: None }));
            ws.lock().unwrap().thread_handle = Some(WebSocket::run(ws.clone(), stream, onmessage, onclose));
            ws
    }

    fn run<F, C>(ws: Arc<Mutex<WebSocket>>, mut stream: TcpStream, onmessage: F, onclose: C) -> JoinHandle<()> 
        where F: Fn(Arc<Mutex<WebSocket>>, Vec<u8>) + Send + 'static, C: Fn(SocketAddr) + Send + 'static {

        let thread_handle = std::thread::spawn(move || {
            let mut buffer: [u8; 8192] = [0; 8192];
            let ip = stream.peer_addr().unwrap();
            loop {
                match stream.read(&mut buffer) {
                    Ok(n) if n > 0 => {
                        
                        let mask_bit = (buffer[1] & 0b10000000) != 0;
                        if mask_bit == false {
                            return;
                        }
                        let _fin = (buffer[0] & 0b10000000) != 0;
                        let opt_code = buffer[0] & 0b00001111;

                        //If the Message is a ping send back a pong

                        let len  = buffer[1] & 0b01111111;
                        let mut offset: u8 = 2;

                        let actual_len: u64 = match len {
                            126 => {
                                offset += 2;
                                ((buffer[2] as u64) << 8) | (buffer[3] as u64)
                            }
                            127 => {
                                offset += 8;
                                ((buffer[2] as u64) << 56) | ((buffer[3] as u64) << 48) |
                                ((buffer[4] as u64) << 40) | ((buffer[5] as u64) << 32) |
                                ((buffer[6] as u64) << 24) | ((buffer[7] as u64) << 16) |
                                ((buffer[8] as u64) << 8) | (buffer[9] as u64)
                            }
                            _ => len as _ 
                        };

                        let mask = [buffer[offset as usize], buffer[offset as usize + 1], buffer[offset as usize + 2], buffer[offset as usize + 3]];
                        offset += 4;

                        let mut data: Vec<u8> = Vec::with_capacity(actual_len as _);

                        data.extend_from_slice(&buffer[offset as usize..actual_len as usize + offset as usize]);

                        for i in 0..actual_len as usize {
                            data[i] = data[i] ^ mask[i % 4];
                        }

                        match opt_code {
                            8 => {
                                let mut close_m = Vec::with_capacity(data.len() + 2);
                                close_m.push(0b10001000);
                                close_m.push(len);
                                close_m.extend_from_slice(&data);
                                stream.write(&close_m).unwrap();
                                onclose(ip);
                                return;
                            }
                            9 => {
                                println!("ping");
                            },
                            10 => continue,
                            _ => (),
                        }

                        onmessage(ws.clone(), data);
                    }
                    Ok(_) | Err(_) => {
                        onclose(ip);
                        return;
                    }, // Verbindung geschlossen oder Fehler
                }
            }
        });
        thread_handle
    }

    pub fn send(&mut self, message: &[u8], msg_type: MessageDataType) {
        let mut m: Vec<u8> = Vec::with_capacity(message.len() + 14);
        let len = message.len();
        //let mask: [u8; 4] = [milis ^ 196, milis ^ 27, milis ^ 157, milis ^ 186];

        match len {
            0..=125 => {
                m.push(0b10000000 | msg_type as u8);
                m.push(len as u8);
                m.extend_from_slice(message);
            }
            0..=65535 => {
                m.push(0b10000000 | msg_type as u8);
                m.push(126);
                m.extend_from_slice(&(len as u16).to_be_bytes());
                m.extend_from_slice(message);
            },
            _ => {
                m.push(0b10000000 | msg_type as u8);
                m.push(127);
                m.extend_from_slice(&(len as u64).to_be_bytes());
                m.extend_from_slice(message);
            }
        }

        match self.stream.write(&m) {
            Ok(_) => (),
            Err(_) => self.close(),
        }
    }

    pub fn send_inmutable(&self, message: &[u8], msg_type: MessageDataType) {
        let mut m: Vec<u8> = Vec::with_capacity(message.len() + 14);
        let len = message.len();
        
        match len {
            0..=125 => {
                m.push(0b10000000 | msg_type as u8);
                m.push(len as u8);
                m.extend_from_slice(message);
            }
            _ => todo!()
        }

        self.stream.try_clone().unwrap().write(&m).unwrap();
    }

    pub fn close(&mut self) {
        if let Some(handle) = self.thread_handle.take() {
            handle.join().unwrap();
        }
    }

    pub fn ip(&self) -> SocketAddr {
        self.stream.peer_addr().unwrap()
    }
}

impl Clone for WebSocket {
    fn clone(&self) -> Self {
        Self { stream: self.stream.try_clone().unwrap(), thread_handle: None }
    }
}

pub enum MessageDataType {
    Continue,
    Text,
    Binary,
}