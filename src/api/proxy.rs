#[cfg(feature = "ssr")]
use axum::{
    body::Body,
    extract::{Path, Query},
    http::{HeaderMap, HeaderValue, Method, StatusCode},
    response::Response,
    Router,
};
#[cfg(feature = "ssr")]
use crate::api::endpoints::{MAIN_API_BASE, SEARCH_API_BASE, RELAY_API_BASE};

#[cfg(feature = "ssr")]
fn extract_user_ip(headers: &HeaderMap) -> Option<String> {
    if let Some(forwarded) = headers.get("x-forwarded-for") {
        if let Ok(forwarded_str) = forwarded.to_str() {
            if let Some(ip) = forwarded_str.split(',').next() {
                let ip = ip.trim().to_string();
                if !ip.is_empty() {
                    return Some(ip);
                }
            }
        }
    }
    
    if let Some(real_ip) = headers.get("x-real-ip") {
        if let Ok(ip_str) = real_ip.to_str() {
            let ip = ip_str.trim().to_string();
            if !ip.is_empty() {
                return Some(ip);
            }
        }
    }
    
    if let Some(client_ip) = headers.get("x-client-ip") {
        if let Ok(ip_str) = client_ip.to_str() {
            let ip = ip_str.trim().to_string();
            if !ip.is_empty() {
                return Some(ip);
            }
        }
    }
    
    None
}

#[cfg(feature = "ssr")]
async fn proxy_main_api_get(
    Path(path): Path<String>,
    Query(query): Query<std::collections::HashMap<String, String>>,
    headers: HeaderMap,
) -> Result<Response, StatusCode> {
    let target_path = if path.starts_with('/') {
        path
    } else {
        format!("/{}", path)
    };
    
    let target_url = format!("{}{}", MAIN_API_BASE, target_path);
    
    let mut query_params = query.clone();
    if !query_params.contains_key("user_ip") {
        if let Some(user_ip) = extract_user_ip(&headers) {
            query_params.insert("user_ip".to_string(), user_ip);
        }
    }
    
    let mut upstream_request = reqwest::Client::builder()
        .build()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .get(&target_url);
    
    for (key, value) in query_params.iter() {
        upstream_request = upstream_request.query(&[(key, value)]);
    }
    
    if let Some(auth_header) = headers.get("authorization") {
        upstream_request = upstream_request.header("authorization", auth_header);
    }
    upstream_request = upstream_request.header("User-Agent", "TorboxCompanion/1.0");
    
    let response = upstream_request
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;
    
    let status = response.status();
    let mut response_builder = Response::builder().status(status);
    
    let response_headers = response.headers();
    for (key, value) in response_headers.iter() {
        let header_name = key.as_str().to_lowercase();
        if !matches!(header_name.as_str(), "connection" | "transfer-encoding" | "content-encoding") {
            if let Ok(header_value) = HeaderValue::from_bytes(value.as_bytes()) {
                response_builder = response_builder.header(key, header_value);
            }
        }
    }
    
    response_builder = response_builder
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, POST, DELETE, PUT, OPTIONS")
        .header("Access-Control-Allow-Headers", "Authorization, Content-Type")
        .header("Access-Control-Allow-Credentials", "true");
    
    let body_bytes = response
        .bytes()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;
    
    Ok(response_builder.body(Body::from(body_bytes)).unwrap())
}

#[cfg(feature = "ssr")]
async fn proxy_search_api_get(
    Path(path): Path<String>,
    Query(query): Query<std::collections::HashMap<String, String>>,
    headers: HeaderMap,
    ) -> Result<Response, StatusCode> {
    let target_path = if path.starts_with('/') {
        path
    } else {
        format!("/{}", path)
    };
    
    let target_url = format!("{}{}", SEARCH_API_BASE, target_path);
    
    let full_url = if !query.is_empty() {
        let query_string: Vec<String> = query
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        format!("{}?{}", target_url, query_string.join("&"))
    } else {
        target_url
    };
    
    let mut upstream_request = reqwest::Client::builder()
        .build()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .get(&full_url);
    
    if let Some(auth_header) = headers.get("authorization") {
        upstream_request = upstream_request.header("authorization", auth_header);
    }
    upstream_request = upstream_request.header("User-Agent", "TorboxCompanion/1.0");
    
    let response = upstream_request
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;
    
    let status = response.status();
    let mut response_builder = Response::builder().status(status);
    
    let response_headers = response.headers();
    for (key, value) in response_headers.iter() {
        let header_name = key.as_str().to_lowercase();
        if !matches!(header_name.as_str(), "connection" | "transfer-encoding" | "content-encoding") {
            if let Ok(header_value) = HeaderValue::from_bytes(value.as_bytes()) {
                response_builder = response_builder.header(key, header_value);
            }
        }
    }
    
    response_builder = response_builder
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, POST, DELETE, PUT, OPTIONS")
        .header("Access-Control-Allow-Headers", "Authorization, Content-Type")
        .header("Access-Control-Allow-Credentials", "true");
    
    let body_bytes = response
        .bytes()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;
    
    Ok(response_builder.body(Body::from(body_bytes)).unwrap())
}

#[cfg(feature = "ssr")]
async fn proxy_relay_api_get(
    Path(path): Path<String>,
    Query(query): Query<std::collections::HashMap<String, String>>,
    headers: HeaderMap,
    ) -> Result<Response, StatusCode> {
    let target_path = if path.starts_with('/') {
        path
    } else {
        format!("/{}", path)
    };
    
    let target_url = format!("{}{}", RELAY_API_BASE, target_path);
    
    let full_url = if !query.is_empty() {
        let query_string: Vec<String> = query
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        format!("{}?{}", target_url, query_string.join("&"))
    } else {
        target_url
    };
    
    let mut upstream_request = reqwest::Client::builder()
        .build()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .get(&full_url);
    
    upstream_request = upstream_request.header("User-Agent", "TorboxCompanion/1.0");
    
    let response = upstream_request
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;
    
    let status = response.status();
    let mut response_builder = Response::builder().status(status);
    
    let response_headers = response.headers();
    for (key, value) in response_headers.iter() {
        let header_name = key.as_str().to_lowercase();
        if !matches!(header_name.as_str(), "connection" | "transfer-encoding" | "content-encoding") {
            if let Ok(header_value) = HeaderValue::from_bytes(value.as_bytes()) {
                response_builder = response_builder.header(key, header_value);
            }
        }
    }
    
    response_builder = response_builder
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, POST, DELETE, PUT, OPTIONS")
        .header("Access-Control-Allow-Headers", "Authorization, Content-Type")
        .header("Access-Control-Allow-Credentials", "true");
    
    let body_bytes = response
        .bytes()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;
    
    Ok(response_builder.body(Body::from(body_bytes)).unwrap())
}

#[cfg(feature = "ssr")]
async fn proxy_main_api_root(
    Query(query): Query<std::collections::HashMap<String, String>>,
    headers: HeaderMap,
) -> Result<Response, StatusCode> {
    proxy_main_api_get(Path("".to_string()), Query(query), headers).await
}

#[cfg(feature = "ssr")]
pub fn create_api_proxy_routes() -> Router<()> {
    Router::new()
        .route(
            "/api",
            axum::routing::get(proxy_main_api_root)
                .post(proxy_main_api_root)
                .put(proxy_main_api_root)
                .delete(proxy_main_api_root),
        )
        .route(
            "/api/",
            axum::routing::get(proxy_main_api_root)
                .post(proxy_main_api_root)
                .put(proxy_main_api_root)
                .delete(proxy_main_api_root),
        )
        .route(
            "/api/search/{*path}",
            axum::routing::get(proxy_search_api_get)
                .post(proxy_search_api_post),
        )
        .route(
            "/api/relay/{*path}",
            axum::routing::get(proxy_relay_api_get)
                .post(proxy_relay_api_post),
        )
        .route(
            "/api/{*path}",
            axum::routing::get(proxy_main_api_get)
                .post(proxy_main_api_post)
                .put(proxy_main_api_put)
                .delete(proxy_main_api_delete),
        )
}

#[cfg(feature = "ssr")]
async fn proxy_main_api_post(
    Path(path): Path<String>,
    Query(query): Query<std::collections::HashMap<String, String>>,
    headers: HeaderMap,
    body: axum::body::Body,
) -> Result<Response, StatusCode> {
    proxy_main_api_with_body(Method::POST, path, query, headers, Some(body)).await
}

#[cfg(feature = "ssr")]
async fn proxy_main_api_put(
    Path(path): Path<String>,
    Query(query): Query<std::collections::HashMap<String, String>>,
    headers: HeaderMap,
    body: axum::body::Body,
) -> Result<Response, StatusCode> {
    proxy_main_api_with_body(Method::PUT, path, query, headers, Some(body)).await
}

#[cfg(feature = "ssr")]
async fn proxy_main_api_delete(
    Path(path): Path<String>,
    Query(query): Query<std::collections::HashMap<String, String>>,
    headers: HeaderMap,
) -> Result<Response, StatusCode> {
    proxy_main_api_with_body(Method::DELETE, path, query, headers, None).await
}

#[cfg(feature = "ssr")]
async fn proxy_search_api_post(
    Path(path): Path<String>,
    Query(query): Query<std::collections::HashMap<String, String>>,
    headers: HeaderMap,
    body: axum::body::Body,
) -> Result<Response, StatusCode> {
    proxy_search_api_with_body(path, query, headers, Some(body)).await
}

#[cfg(feature = "ssr")]
async fn proxy_relay_api_post(
    Path(path): Path<String>,
    Query(query): Query<std::collections::HashMap<String, String>>,
    headers: HeaderMap,
    body: axum::body::Body,
) -> Result<Response, StatusCode> {
    proxy_relay_api_with_body(path, query, headers, Some(body)).await
}

#[cfg(feature = "ssr")]
async fn proxy_main_api_with_body(
    method: Method,
    path: String,
    query: std::collections::HashMap<String, String>,
    headers: HeaderMap,
    body: Option<axum::body::Body>,
) -> Result<Response, StatusCode> {
    let target_path = if path.starts_with('/') {
        path
    } else {
        format!("/{}", path)
    };
    
    let target_url = format!("{}{}", MAIN_API_BASE, target_path);
    
    let mut query_params = query.clone();
    if !query_params.contains_key("user_ip") {
        if let Some(user_ip) = extract_user_ip(&headers) {
            query_params.insert("user_ip".to_string(), user_ip);
        }
    }
    
    let mut upstream_request = reqwest::Client::builder()
        .build()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .request(method.clone(), &target_url);
    
    for (key, value) in query_params.iter() {
        upstream_request = upstream_request.query(&[(key, value)]);
    }
    
    // Forward relevant headers (especially Authorization)
    if let Some(auth_header) = headers.get("authorization") {
        upstream_request = upstream_request.header("authorization", auth_header);
    }
    if let Some(content_type) = headers.get("content-type") {
        upstream_request = upstream_request.header("content-type", content_type);
    }
    upstream_request = upstream_request.header("User-Agent", "TorboxCompanion/1.0");
    
    if let Some(body) = body {
        let body_bytes = axum::body::to_bytes(body, usize::MAX)
            .await
            .map_err(|_| StatusCode::BAD_REQUEST)?;
        upstream_request = upstream_request.body(body_bytes.to_vec());
    }
    
    let response = upstream_request
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;
    
    let status = response.status();
    let mut response_builder = Response::builder().status(status);
    
    let response_headers = response.headers();
    for (key, value) in response_headers.iter() {
        let header_name = key.as_str().to_lowercase();
        if !matches!(header_name.as_str(), "connection" | "transfer-encoding" | "content-encoding") {
            if let Ok(header_value) = HeaderValue::from_bytes(value.as_bytes()) {
                response_builder = response_builder.header(key, header_value);
            }
        }
    }
    
    response_builder = response_builder
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, POST, DELETE, PUT, OPTIONS")
        .header("Access-Control-Allow-Headers", "Authorization, Content-Type")
        .header("Access-Control-Allow-Credentials", "true");
    
    let body_bytes = response
        .bytes()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;
    
    Ok(response_builder.body(Body::from(body_bytes)).unwrap())
}

#[cfg(feature = "ssr")]
async fn proxy_search_api_with_body(
    path: String,
    query: std::collections::HashMap<String, String>,
    headers: HeaderMap,
    body: Option<axum::body::Body>,
) -> Result<Response, StatusCode> {
    let target_path = if path.starts_with('/') {
        path
    } else {
        format!("/{}", path)
    };
    
    let target_url = format!("{}{}", SEARCH_API_BASE, target_path);
    
    let full_url = if !query.is_empty() {
        let query_string: Vec<String> = query
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        format!("{}?{}", target_url, query_string.join("&"))
    } else {
        target_url
    };
    
    let mut upstream_request = reqwest::Client::builder()
        .build()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .post(&full_url);
    
    // Forward Authorization header (required for torrents/usenet search, optional for metadata)
    if let Some(auth_header) = headers.get("authorization") {
        upstream_request = upstream_request.header("authorization", auth_header);
    }
    upstream_request = upstream_request.header("User-Agent", "TorboxCompanion/1.0");
    
    if let Some(body) = body {
        let body_bytes = axum::body::to_bytes(body, usize::MAX)
            .await
            .map_err(|_| StatusCode::BAD_REQUEST)?;
        upstream_request = upstream_request.body(body_bytes.to_vec());
    }
    
    let response = upstream_request
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;
    
    let status = response.status();
    let mut response_builder = Response::builder().status(status);
    
    let response_headers = response.headers();
    for (key, value) in response_headers.iter() {
        let header_name = key.as_str().to_lowercase();
        if !matches!(header_name.as_str(), "connection" | "transfer-encoding" | "content-encoding") {
            if let Ok(header_value) = HeaderValue::from_bytes(value.as_bytes()) {
                response_builder = response_builder.header(key, header_value);
            }
        }
    }
    
    response_builder = response_builder
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, POST, DELETE, PUT, OPTIONS")
        .header("Access-Control-Allow-Headers", "Authorization, Content-Type")
        .header("Access-Control-Allow-Credentials", "true");
    
    let body_bytes = response
        .bytes()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;
    
    Ok(response_builder.body(Body::from(body_bytes)).unwrap())
}

#[cfg(feature = "ssr")]
async fn proxy_relay_api_with_body(
    path: String,
    query: std::collections::HashMap<String, String>,
    headers: HeaderMap,
    body: Option<axum::body::Body>,
) -> Result<Response, StatusCode> {
    let target_path = if path.starts_with('/') {
        path
    } else {
        format!("/{}", path)
    };
    
    let target_url = format!("{}{}", RELAY_API_BASE, target_path);
    
    let full_url = if !query.is_empty() {
        let query_string: Vec<String> = query
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        format!("{}?{}", target_url, query_string.join("&"))
    } else {
        target_url
    };
    
    let mut upstream_request = reqwest::Client::builder()
        .build()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .post(&full_url);
    
    upstream_request = upstream_request.header("User-Agent", "TorboxCompanion/1.0");
    
    if let Some(body) = body {
        let body_bytes = axum::body::to_bytes(body, usize::MAX)
            .await
            .map_err(|_| StatusCode::BAD_REQUEST)?;
        upstream_request = upstream_request.body(body_bytes.to_vec());
    }
    
    let response = upstream_request
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;
    
    let status = response.status();
    let mut response_builder = Response::builder().status(status);
    
    let response_headers = response.headers();
    for (key, value) in response_headers.iter() {
        let header_name = key.as_str().to_lowercase();
        if !matches!(header_name.as_str(), "connection" | "transfer-encoding" | "content-encoding") {
            if let Ok(header_value) = HeaderValue::from_bytes(value.as_bytes()) {
                response_builder = response_builder.header(key, header_value);
            }
        }
    }
    
    response_builder = response_builder
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, POST, DELETE, PUT, OPTIONS")
        .header("Access-Control-Allow-Headers", "Authorization, Content-Type")
        .header("Access-Control-Allow-Credentials", "true");
    
    let body_bytes = response
        .bytes()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;
    
    Ok(response_builder.body(Body::from(body_bytes)).unwrap())
}

