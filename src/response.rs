use std::fs;

pub struct Response {
    status: i32,
    reason: String,
    body: String,
}

impl Response {
    pub fn new(status: i32, reason: String, body: String) -> Response {
        Response { status, reason, body }
    }

    pub fn clone(&self) -> Response {
        Response { status: self.status, reason: self.reason.clone(), body: self.body.clone() }
    }

    pub fn format(&self) -> String {
        let result = format!(
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status, self.reason, self.body.len(), self.body
        );

        result
    }
}