#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    INIT,
}

impl From<&str> for Version {
    fn from(s: &str) -> Version {
        match s {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::INIT,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_implementation() {
        let v1: Version = "HTTP/1.1".into();
        let v2: Version = "HTTP/2.0".into();

        assert_eq!(v1, Version::V1_1);
        assert_eq!(v2, Version::INIT);
    }
}
