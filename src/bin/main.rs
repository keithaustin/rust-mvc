use std::fs;

use webserver::ThreadPool;
use webserver::request::{Request, Method};
use webserver::response::Response;
use webserver::router::Router;
use webserver::server::Server;

fn main() {
    // Create Router and add home route
    let mut router = Router::new();
    router.get(
        "/", 
        Response::new(
            200, 
            String::from("OK"), 
            fs::read_to_string("view/hello.html").unwrap()
        ),
    );
    router.get(
        "/test", 
        Response::new(
            200,
            String::from("OK"),
            fs::read_to_string("view/test.html").unwrap()
        ),
    );
    // Create server
    Server::new(router).run("127.0.0.1:7878");
}
