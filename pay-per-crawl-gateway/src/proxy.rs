use axum::{
    body::Body,
    extract::{Request, State},
    http::{HeaderValue, Response, StatusCode},
    response::IntoResponse,
};
use reqwest::Method;
use std::sync::Arc;
use http_body_util::BodyExt;

#[derive(Clone)]
pub struct ProxyState {
    pub client: reqwest::Client,
    pub target_origin: String,
}

pub async fn reverse_proxy_handler(
    State(state): State<Arc<ProxyState>>,
    req: Request<Body>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let path = req.uri().path();
    let query = req.uri().query().unwrap_or("");
    
    let target_url = if query.is_empty() {
        format!("{}{}", state.target_origin, path)
    } else {
        format!("{}{}?{}", state.target_origin, path, query)
    };

    let method = req.method().clone();
    let mut headers = req.headers().clone();
    
    // Remove headers that shouldn't be proxied blindly
    headers.remove(axum::http::header::HOST);

    // Read body
    let body_bytes = req.into_body().collect().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .to_bytes();

    let reqwest_method = Method::from_bytes(method.as_str().as_bytes()).unwrap();

    let mut proxy_req = state.client.request(reqwest_method, &target_url);
    for (name, value) in headers.iter() {
        proxy_req = proxy_req.header(name.as_str(), value.as_bytes());
    }
    
    let res = proxy_req.body(body_bytes).send().await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("Proxy error: {}", e)))?;

    let status = StatusCode::from_u16(res.status().as_u16()).unwrap();
    let mut response_builder = Response::builder().status(status);
    
    for (name, value) in res.headers().iter() {
        response_builder = response_builder.header(name.as_str(), value.as_bytes());
    }

    let response_bytes = res.bytes().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Body error: {}", e)))?;

    let response = response_builder.body(Body::from(response_bytes))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Builder error: {}", e)))?;

    Ok(response)
}
