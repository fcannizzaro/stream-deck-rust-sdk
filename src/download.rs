use std::collections::HashMap;

use hyper::{body, Client, Method, Request};
use hyper_tls::HttpsConnector;

pub async fn download_image(
    url: String,
    headers: Option<HashMap<String, String>>,
) -> Option<Vec<u8>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let mut request = Request::builder().method(Method::GET).uri(url);
    if headers.is_some() {
        for (key, value) in headers.unwrap().iter() {
            request = request.header(key, value);
        }
    }
    let req = request.body::<hyper::Body>(hyper::Body::empty()).unwrap();
    let mut res = client.request(req).await.unwrap();
    if res.status().is_success() {
        let body = res.body_mut();
        let bytes = body::to_bytes(body).await.unwrap();
        Some(bytes.to_vec())
    } else {
        None
    }
}
