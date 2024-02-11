use std::{
    error::Error,
    io::Read,
    net::{TcpListener, TcpStream},
    str::from_utf8,
};

use crate::http::request::{Request, RequestError};

/// Server strcuture
pub struct Server {
    /// Address
    address: String,
}

impl Server {
    /// Create new server instance
    pub fn new(address: &str) -> Self {
        Server {
            address: address.to_string(),
        }
    }

    /// Read data from the stream
    fn handle_stream(&self, stream: &mut TcpStream) -> Result<(), RequestError> {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(bytes) => {
                let raw_request = &buffer[..bytes];
                println!("bytes: {}", bytes);
                println!("Buffer: {:?}", from_utf8(raw_request).unwrap());
                let parsed_request = Request::try_from(raw_request)?;
                println!("parsed_request: {:?}", parsed_request);
            }
            Err(_) => println!("Failed to read from stream"),
        }
        Ok(())
    }

    /// Run the server to listen to connections from client
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        println!("Listening to connection on {}", &self.address);
        // Create a TCP Listener
        let listener = TcpListener::bind(&self.address)?;
        // Listen to stream
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => self.handle_stream(&mut stream)?,
                Err(err) => println!("Failed to listen: {}", err),
            }
        }

        Ok(())
    }
}
