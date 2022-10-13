use hyper::{body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use std::collections::HashMap;

pub async fn download_image(url: String, headers: Option<HashMap<String, String>>) -> Vec<u8> {
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
    let body = res.body_mut();
    body::to_bytes(body).await.unwrap().to_vec()
}
