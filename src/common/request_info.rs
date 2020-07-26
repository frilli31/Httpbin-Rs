use std::collections::HashMap;

use actix_web::HttpRequest;
use qstring::QString;
use serde::Serialize;

#[derive(Serialize)]
pub struct RequestInfo {
    origin: String,
    url: String,
    args: HashMap<String, String>,
    headers: HashMap<String, String>,
}

impl RequestInfo {
    pub fn new(req: HttpRequest) -> RequestInfo {
        RequestInfo {
            origin: req.peer_addr().unwrap().to_string(),
            url: RequestInfo::get_url(&req),
            args: RequestInfo::get_args(&req),
            headers: RequestInfo::get_headers(&req),
        }
    }

    fn get_url(req: &HttpRequest) -> String {
        format!("{scheme}://{host}{uri}",
                scheme = req.connection_info().scheme(),
                host = req.connection_info().host(),
                uri = req.path())
    }

    fn get_args(req: &HttpRequest) -> HashMap<String, String> {
        let mut args = HashMap::new();
        for (key, value) in QString::from(req.query_string()).into_iter() {
            args.insert(key, value);
        }
        args
    }

    fn get_headers(req: &HttpRequest) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        for (key, value) in req.headers() {
            headers.insert(key.to_string(), value.to_str().unwrap().to_string());
        }
        headers
    }
}

#[cfg(test)]
mod tests {
    use std::net::{SocketAddr, SocketAddrV4};

    use actix_web::test;

    use super::*;

    #[actix_rt::test]
    async fn request_info_test() {
        let req = test::TestRequest::get()
            .uri("/hello/test?paramname=paramvalue")
            .header("headername", "headervalue")
            .peer_addr(SocketAddr::from_str("127.0.0.2:9090").unwrap())
            .to_http_request();

        let req_info = RequestInfo::new(req.clone());

        assert_eq!(req_info.origin, req.peer_addr().unwrap().to_string(), "Peer address is equal");
        assert_eq!(req_info.headers.get("headername").unwrap(), "headervalue", "The headers contain the right value");
        assert_eq!(req_info.headers.len(), req.headers().len(), "Request headers has the right size");
        assert_eq!(req_info.args.len(), 1, "Request parameters has the right size");
        assert_eq!(req_info.args.get("paramname"), Some(&"paramvalue".to_string()), "The parameters contain the right value");
    }
}