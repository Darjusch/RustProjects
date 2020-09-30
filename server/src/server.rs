use std::net::TcpListener;
use std::io::{Read, Write};
use std::convert::TryFrom;
use std::convert::TryInto;
use crate::http::Request;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on: {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
          match listener.accept() {
            Ok((mut stream, _)) => {
              // Gives a Hello World back to the user!
              // match stream.write(b"HTTP/1.1 200 OK\nContent-Type: text/plain\nContent-Length: 12\n\nHello world!") {
              //   Ok(_) => {
              //     println!("Success");
              //   }
              //   Err(e) => {
              //     println!("Error: {}", e);
              //   }
              // }
              // find a way to increase buffer size || how to add the buffer to read it then complete
              let mut buffer = [0; 1024];
              match stream.read(&mut buffer) {
                Ok(_) => {
                  println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                  match Request::try_from(&buffer[..]) {
                    Ok(request) => {},
                    Err(e) => println!("Failed to parse a request:  {}", e),
                  }
                },
                Err(e) => println!("Failed to read from connection: {}", e),
              };
            }
            Err(e) => println!("Failed to establish a connection: {}", e),
          }
        }
    }
}
