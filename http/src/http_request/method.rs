#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
    INIT,
}

impl From<&str> for Method {
    fn from(s: &str) -> Method {
        match s {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => Method::INIT,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_implementation() {
        let get: Method = "GET".into();
        let post: Method = "POST".into();
        let init: Method = "blah".into();

        assert_eq!(get, Method::GET);
        assert_eq!(post, Method::POST);
        assert_eq!(init, Method::INIT);
    }
}
