pub enum RequestType {
    Score,
    Standings,
    Stat,
}

pub struct Request {
    request_type: RequestType,
    http_method: HttpMethod,
    url: String,
}

struct HttpMethod(String);
impl Default for HttpMethod {
    fn default() -> Self {
        HttpMethod("GET".to_string())
    }
}

// define a Route, possibly a Url type
// maybe move the http plumbing to a separate mod
