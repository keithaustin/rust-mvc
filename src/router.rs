use crate::response::Response;
use std::fs;

pub struct Route {
    path: String,
    response: Response,
}

/// Router takes in requests and returns results
pub struct Router {
    pub routes: Vec<Route>,
}

impl Router {
    /// Creates a new Router
    pub fn new() -> Router {
        Router { routes: Vec::new() }
    }

    pub fn get(&mut self, path: &str, response: Response) {
        let route = Route { path: path.to_string(), response };
        self.routes.push(route);
    }

    pub fn route_path(&self, path: String) -> Response {
        for route in self.routes.iter() {
            if route.path == path {
                println!("Path {} matched {}", route.path, path);
                let result = route.response.clone();
                return result;
            }
        }

        Response::new(404, String::from("NOT FOUND"), fs::read_to_string("view/404.html").unwrap())
    }
}