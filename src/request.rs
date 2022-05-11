use std::collections::HashMap;

pub enum Method {
    Get,
    Post
}

pub struct Request {
    pub path: String,
    pub method: Method,
    pub params: HashMap<String, String>,
}

impl Request {
    pub fn new(path: String, method: Method, params: HashMap<String, String>) -> Request {
        Request { path, method, params }
    }

    pub fn parse(data: String) -> Request {
        let mut parts: Vec<&str> = data.split("\r\n").collect();
        
        let head: Vec<&str> = parts[0].split_whitespace().collect();

        let method = match head[0] {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Get
        };

        let path = head[1].to_string();

        parts.remove(0);

        let mut params = HashMap::new();

        for param in parts.iter() {
            let p_vec: Vec<&str> = param.split(": ").collect();

            if p_vec.len() == 2 {
                params.insert(p_vec[0].to_string(), p_vec[1].to_string());
            }
            
        }

        Request { path, method, params }
    }

    pub fn print(&self) {
        println!("Path = {}", self.path);
        match self.method {
            Method::Get => println!("Method = GET"),
            Method::Post => println!("Method = POST"),
            _ => println!("No Method?"),
        }
    }
}
