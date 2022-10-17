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
        let mut parsed_version = Version::Uninitialized;
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "".to_string();

        for line in req.lines() {
            if line.contains("HTTP") {
                let (version, method, resource) = process_req_line(line);
                parsed_version = version;
                parsed_method = method;
                parsed_resource = resource;
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

fn process_req_line(line: &str) -> (Version, Method, Resource) {
    let mut iter = line.split_whitespace();
    let version = iter.next().unwrap();
    let method = iter.next().unwrap();
    let resource = iter.next().unwrap();
    (
        version.into(),
        method.into(),
        Resource::Path(resource.to_string()),
    )
}

fn process_header_line(line: &str) -> (String, String) {
    let key_value: Vec<&str> = line.split(":").collect();
    (key_value[0].to_string(), key_value[1].to_string())
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
        let http_request: HttpRequest = String::from("HTTP/1.1 GET /hello\n123\nfoo:bar").into();
        let mut headers = HashMap::new();
        headers.insert("foo".to_string(), "bar".to_string());
        assert_eq!(
            http_request,
            HttpRequest {
                method: Method::GET,
                version: Version::V1_1,
                msg_body: "123".to_string(),
                resource: Resource::Path(String::from("/hello")),
                headers: headers
            }
        )
    }
}
