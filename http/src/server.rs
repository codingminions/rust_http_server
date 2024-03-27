use crate::httpmods::Request;
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

// Declaring an array in Rust when the size is known;
// fn arr(a: [u8; 5]) {}
// if we don't know the size, better to pass the reference.
// fn arr(a: &[u8]) {} == also known as array slice.

impl Server {
    pub fn new(addr: String) -> Self {
        Server { addr }
    }

    pub fn run(self) {
        println!("Server is listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();
        // recoverable and unrecoverable errors.
        // Rust doesn't support Exceptions.

        // in case we want our inner loop to break the outer loop,
        // use labels:
        // 'outer: loop{
        //    loop{
        //        break 'outer;
        //    }
        // }
        loop {
            match listener.accept() {
                // Ok(_) => println!("Ok");
                Ok((mut stream, _)) => {
                    println!("Stream: {:?} OK", stream);
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("received a request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {}
                                Err(e) => {
                                    println!(
                                        "Request cannot be converted to readable format: {}",
                                        e
                                    );
                                }
                            }

                            // let res: &Result<Request, _> = &buffer[..].try_into();
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                // _ => catch all errors that we have not matched manually.
                Err(e) => println!("Error: {}", e),
            }
        }
        // in tcp connections, receiving connections means accepting it.
        //     let res = listener.accept();
        //     if res.is_err() {
        //         continue;
        //     }

        //     let (stream, addr) = res.unwrap();
        // }

        // let tup = (5, "a", listener);
    }
}
