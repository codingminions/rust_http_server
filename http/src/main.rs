#![allow(dead_code)]
use server::Server;
use std::env;
use website_handler::WebsiteHandler;

mod httpmods;
mod server;
mod website_handler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("Public path is: {}", public_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}

// HTTP1.1 is L7 protocol and uses TCP.
// HTTP Server Design:
// 1. TCP Listener: Since messages are passed over TCP in HTTP1.1, we need
// a listener to get the bytes present in the request and then send it back.
// 2. Then we need a HTTP parser to actually read the request and validate.
// 3. If the request is parsed successfully, we send it to Handler which contains the
// logic for various routing functions. Once the logic has been implemented
// the handler sends the response back to the Parser which further passes it to
// TCP listener and subsequently, the client.
// The Server will run on a single thread, hence the server will only accept 1 request
// at a time.

/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
 */
