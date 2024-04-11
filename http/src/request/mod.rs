use std::collections::HashMap;

mod method;
pub mod resource;
mod utils;
mod version;

use method::Method;
use resource::Resource;
use utils::{process_header_line, process_req_line};
use version::Version;

pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> HttpRequest {
        let mut method = Method::INIT;
        let mut version = Version::V1_1;
        let mut resource = Resource::Path("".to_string());
        let mut headers = HashMap::new();
        let mut body = "";

        // Read each line in incoming HTTP request
        for line in req.lines() {
            if line.contains("HTTP") {
                // Line contains request details
                let (parsed_method, parsed_resource, parsed_version) = process_req_line(line);

                method = parsed_method;
                resource = parsed_resource;
                version = parsed_version;
            } else if line.contains(":") {
                // Line contains header details
                let (key, value) = process_header_line(line);

                headers.insert(key, value);
            } else if line.len() == 0 {
                // Line is a separator line so we do nothing
            } else {
                // Line contains request body
                body = line;
            }
        }

        HttpRequest {
            method,
            version,
            resource,
            headers,
            body: body.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read() {
        let s: String = String::from("GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n");
        let req: HttpRequest = s.into();

        let mut headers_expected = HashMap::new();
        headers_expected.insert("Host".into(), " localhost".into());
        headers_expected.insert("Accept".into(), " */*".into());
        headers_expected.insert("User-Agent".into(), " curl/7.64.1".into());

        assert_eq!(Method::GET, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        assert_eq!(headers_expected, req.headers);
    }
}
