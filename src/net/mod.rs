mod web_socket;
mod https;
pub use web_socket::MessageDataType;
pub use web_socket::WebSocketInterface;
pub use web_socket::WebSocket;
pub use https::HTTPS;

mod tests {

    #[test]
    fn it_works() {
        println!("lol")
    }
}