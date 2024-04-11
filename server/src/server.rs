use http::request::HttpRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

use crate::router::Router;

pub struct Server<'a> {
    address: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(address: &'a str) -> Self {
        Server { address }
    }

    pub fn run(&self) {
        // Start a server listening on socket address
        let listener = TcpListener::bind(self.address).unwrap();
        println!("Running on {}", self.address);

        // Continuously listen to incoming connections
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();

            println!("Connection established");

            let mut buffer = [0; 90];
            stream.read(&mut buffer).unwrap();

            // Convert stream to HttpRequest struct instance
            let req: HttpRequest = String::from_utf8(buffer.to_vec()).unwrap().into();

            Router::route(req, &mut stream);
        }
    }
}
