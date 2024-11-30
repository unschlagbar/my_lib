mod web_socket;
mod https;
pub use web_socket::WebSocket;
pub use https::HTTPS;
pub use web_socket::MessageDataType;

mod tests {

    #[test]
    fn it_works() {
        println!("lol")
    }
}