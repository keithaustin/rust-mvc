use std::fs;
use std::io::prelude::*;
use std::net::{ TcpListener, TcpStream };

use crate::ThreadPool;
use crate::request::{Request, Method};
//use webserver::response::Response;
use crate::router::Router;

pub struct Server {
    router: Router,
}

impl Server {
    pub fn new(router: Router) -> Server {
        Server { router }
    }

    pub fn run(&self, address: &str) {
        let listener = TcpListener::bind(address).unwrap();
        let pool = ThreadPool::new(4);

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            //pool.execute(|| {
                self.handle_connection(stream);
            //});
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let data = String::from_utf8_lossy(&buffer[..]);

        let request = Request::parse(data.to_string());

        let response = self.router.route_path(request.path);

        stream.write(response.format().as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}