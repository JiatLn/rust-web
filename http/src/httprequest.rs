use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => Method::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(v: &str) -> Self {
        match v {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _ => Version::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug, PartialEq)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_version = Version::Uninitialized;
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "".to_string();

        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.is_empty() {
                continue;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            } else {
                parsed_msg_body = line.to_string();
            }
        }

        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body,
        }
    }
}

fn process_req_line(line: &str) -> (Method, Resource, Version) {
    let mut iter = line.split_whitespace();
    let method = iter.next().unwrap();
    let resource = iter.next().unwrap();
    let version = iter.next().unwrap();
    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}

fn process_header_line(line: &str) -> (String, String) {
    let mut iter = line.split(":").into_iter();
    let mut key = String::from("");
    let mut value = String::from("");
    if let Some(s) = iter.next() {
        key = s.to_string()
    };
    if let Some(s) = iter.next() {
        value = s.to_string()
    };
    (key, value)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_method_into() {
        let method: Method = "GET".into();
        assert_eq!(method, Method::GET);
    }

    #[test]
    fn test_version_into() {
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v, Version::V1_1)
    }

    #[test]
    fn test_http_request_into() {
        let req: HttpRequest = String::from(
            "GET /api/user/1 HTTP/1.1
Accept: application/json, text/plain, */*
Cache-Control: no-cache
Connection: keep-alive
Host: 127.0.0.1",
        )
        .into();

        assert!(req.headers.contains_key("Accept"));
        assert!(req.headers.contains_key("Cache-Control"));
        assert!(req.headers.contains_key("Connection"));
        assert!(req.headers.contains_key("Host"));
        assert_eq!(Method::GET, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path(String::from("/api/user/1")), req.resource);
    }
}
