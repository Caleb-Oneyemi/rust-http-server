use super::method::Method;
use super::resource::Resource;
use super::version::Version;

pub fn process_req_line(s: &str) -> (Method, Resource, Version) {
    // Parse the request line into individual chunks split by whitespace.
    let vec: Vec<&str> = s.split_whitespace().collect();

    // Extract the HTTP method at index 0
    let method = vec[0];
    // Extract the resource (URI/URL) at index 1
    let resource = vec[1];
    // Extract the HTTP version at index 2
    let version = vec[2];

    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}

pub fn process_header_line(s: &str) -> (String, String) {
    // Parse the header line into words split by separator
    let header_items: Vec<&str> = s.split(":").collect();

    let key = header_items[0].to_string();
    let value = header_items[1].to_string();

    (key, value)
}
