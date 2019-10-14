use log::{error, info};
use std::net::TcpListener;

fn main() {
    simple_logger::init().unwrap();
    info!("Starting server...");

    let ip = "127.0.0.1:8080";

    let listener = TcpListener::bind(ip).expect("Unable to create listener");

    info!("Server started on {} {}", "http://", ip);

    for stream in listener.incoming() {
        match stream {
            Ok(_) => info!("Request received!"),
            Err(error) => error!("Connection failed: {}", error),
        }
    }
}
