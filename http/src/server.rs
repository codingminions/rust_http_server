use crate::httpmods::request::ParseError;
use crate::httpmods::{Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse the request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Server is listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    println!("Stream: {:?} OK", stream);
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("received a request: {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }

                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                // _ => catch all errors that we have not matched manually.
                Err(e) => println!("Error: {}", e),
            }
        }
    }
}

// Declaring an array in Rust when the size is known;
// fn arr(a: [u8; 5]) {}
// if we don't know the size, better to pass the reference.
// fn arr(a: &[u8]) {} == also known as array slice.
// recoverable and unrecoverable errors.
// Rust doesn't support Exceptions.
// in case we want our inner loop to break the outer loop,
// use labels:
// 'outer: loop{
//    loop{
//        break 'outer;
//    }
// }
// in tcp connections, receiving connections means accepting it.
//     let res = listener.accept();
//     if res.is_err() {
//         continue;
//     }

//     let (stream, addr) = res.unwrap();
// }

// let tup = (5, "a", listener);
