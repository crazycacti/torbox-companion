#[cfg(feature = "ssr")]
use axum::{
    body::Body,
    extract::{ConnectInfo, Path, Query, Request},
    http::{HeaderMap, HeaderValue, Method, StatusCode},
    middleware::{self, Next},
    response::Response,
    Extension, Router,
};
#[cfg(feature = "ssr")]
use crate::api::endpoints::{MAIN_API_BASE, SEARCH_API_BASE, RELAY_API_BASE};
#[cfg(feature = "ssr")]
use once_cell::sync::Lazy;
#[cfg(feature = "ssr")]
use std::collections::HashMap;
#[cfg(feature = "ssr")]
use std::net::{IpAddr, SocketAddr};
#[cfg(feature = "ssr")]
use std::sync::Mutex;
#[cfg(feature = "ssr")]
use std::time::Instant;

#[cfg(feature = "ssr")]
// Default leaves room for torrent/NZB multipart uploads. Override with TORBOX_PROXY_MAX_BODY_MB.
#[cfg(feature = "ssr")]
fn max_proxy_body_bytes() -> usize {
    std::env::var("TORBOX_PROXY_MAX_BODY_MB")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .map(|mb| mb.saturating_mul(1024 * 1024).max(1024 * 1024))
        .unwrap_or(100 * 1024 * 1024)
}
#[cfg(feature = "ssr")]
// Dashboard polls multiple TorBox endpoints about every 10s; keep headroom for
// interactive actions and multi-tab use. Override with TORBOX_PROXY_RATE_LIMIT.
#[cfg(feature = "ssr")]
fn rate_limit_max_requests() -> usize {
    std::env::var("TORBOX_PROXY_RATE_LIMIT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(300)
}
#[cfg(feature = "ssr")]
const RATE_LIMIT_WINDOW_SECS: u64 = 60;

#[cfg(feature = "ssr")]
#[derive(Clone, Debug)]
struct ClientIdentity {
    rate_limit_key: String,
    user_ip: Option<String>,
}

#[cfg(feature = "ssr")]
struct RateLimitEntry {
    window_start: Instant,
    count: usize,
}

#[cfg(feature = "ssr")]
static RATE_LIMITER: Lazy<Mutex<HashMap<String, RateLimitEntry>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

#[cfg(feature = "ssr")]
fn is_trusted_proxy_peer(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => v4.is_loopback() || v4.is_private() || v4.is_link_local(),
        IpAddr::V6(v6) => {
            v6.is_loopback()
                || v6.is_unique_local()
                || v6.to_ipv4_mapped().is_some_and(|v4| {
                    v4.is_loopback() || v4.is_private() || v4.is_link_local()
                })
        }
    }
}

#[cfg(feature = "ssr")]
fn trust_forwarded_headers(peer: Option<SocketAddr>) -> bool {
    if std::env::var("TORBOX_TRUST_PROXY")
        .map(|v| {
            matches!(
                v.to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(false)
    {
        return true;
    }
    peer.map(|addr| is_trusted_proxy_peer(addr.ip()))
        .unwrap_or(false)
}

#[cfg(feature = "ssr")]
fn extract_forwarded_ip(headers: &HeaderMap) -> Option<String> {
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
fn auth_rate_limit_key(headers: &HeaderMap) -> Option<String> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let raw = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .or_else(|| headers.get("x-api-key").and_then(|v| v.to_str().ok()))?;
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    let mut hasher = DefaultHasher::new();
    trimmed.hash(&mut hasher);
    Some(format!("auth:{:x}", hasher.finish()))
}

/// Resolve visitor identity for rate limiting and optional TorBox `user_ip`.
/// Behind a reverse proxy, use forwarded client headers when trusted.
/// Never attribute every visitor to a public CDN/proxy peer IP.
#[cfg(feature = "ssr")]
fn resolve_client_identity(headers: &HeaderMap, peer: Option<SocketAddr>) -> ClientIdentity {
    let forwarded = extract_forwarded_ip(headers);
    let trust = trust_forwarded_headers(peer);

    if trust {
        if let Some(ip) = forwarded {
            return ClientIdentity {
                rate_limit_key: ip.clone(),
                user_ip: Some(ip),
            };
        }
        // Trusted proxy hop but missing client headers.
        return ClientIdentity {
            rate_limit_key: auth_rate_limit_key(headers)
                .unwrap_or_else(|| "unidentified".to_string()),
            user_ip: None,
        };
    }

    // Forwarded headers present but peer is a public proxy/CDN and
    // TORBOX_TRUST_PROXY is unset: do not share one bucket on the proxy IP,
    // and do not trust spoofable headers for TorBox user_ip.
    if forwarded.is_some() {
        return ClientIdentity {
            rate_limit_key: auth_rate_limit_key(headers)
                .unwrap_or_else(|| "unidentified".to_string()),
            user_ip: None,
        };
    }

    // Direct exposure: peer address is the client.
    if let Some(addr) = peer {
        let ip = addr.ip().to_string();
        return ClientIdentity {
            rate_limit_key: ip.clone(),
            user_ip: Some(ip),
        };
    }

    ClientIdentity {
        rate_limit_key: auth_rate_limit_key(headers)
            .unwrap_or_else(|| "unknown".to_string()),
        user_ip: None,
    }
}

#[cfg(feature = "ssr")]
fn check_rate_limit_for_key(ip: &str) -> Result<(), StatusCode> {
    let mut map = RATE_LIMITER
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let now = Instant::now();
    // Drop expired windows so long-running hosts do not grow unbounded.
    map.retain(|_, entry| {
        now.duration_since(entry.window_start).as_secs() < RATE_LIMIT_WINDOW_SECS
    });
    let entry = map.entry(ip.to_string()).or_insert(RateLimitEntry {
        window_start: now,
        count: 0,
    });
    if now.duration_since(entry.window_start).as_secs() >= RATE_LIMIT_WINDOW_SECS {
        entry.window_start = now;
        entry.count = 0;
    }
    if entry.count >= rate_limit_max_requests() {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }
    entry.count += 1;
    Ok(())
}

#[cfg(feature = "ssr")]
async fn rate_limit_middleware(mut request: Request, next: Next) -> Result<Response, StatusCode> {
    let peer = request
        .extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|ConnectInfo(addr)| *addr);
    let identity = resolve_client_identity(request.headers(), peer);
    check_rate_limit_for_key(&identity.rate_limit_key)?;
    request.extensions_mut().insert(identity);
    Ok(next.run(request).await)
}

#[cfg(feature = "ssr")]
async fn read_proxy_body(body: Body) -> Result<Vec<u8>, StatusCode> {
    axum::body::to_bytes(body, max_proxy_body_bytes())
        .await
        .map(|bytes| bytes.to_vec())
        .map_err(|_| StatusCode::PAYLOAD_TOO_LARGE)
}

#[cfg(feature = "ssr")]
fn apply_cors_headers(
    response_builder: axum::http::response::Builder,
) -> axum::http::response::Builder {
    response_builder
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, POST, DELETE, PUT, OPTIONS")
        .header("Access-Control-Allow-Headers", "Authorization, Content-Type, X-Api-Key")
}

#[cfg(feature = "ssr")]
fn normalize_proxy_path(path: String) -> String {
    if path.starts_with('/') {
        path
    } else {
        format!("/{}", path)
    }
}

#[cfg(feature = "ssr")]
fn is_allowed_main_api_path(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    if path.contains("..")
        || lower.contains("%2e%2e")
        || lower.contains("%2e.")
        || lower.contains(".%2e")
        || lower.contains("%252e")
    {
        return false;
    }
    if path.is_empty() || path == "/" {
        return true;
    }
    if path.starts_with("/v1/api/") {
        return true;
    }
    let without_leading_slash = path.strip_prefix('/').unwrap_or(path);
    without_leading_slash.starts_with("v1/api/")
}

#[cfg(feature = "ssr")]
async fn proxy_main_api_get(
    Path(path): Path<String>,
    Query(query): Query<std::collections::HashMap<String, String>>,
    headers: HeaderMap,
    Extension(identity): Extension<ClientIdentity>,
) -> Result<Response, StatusCode> {
    let target_path = normalize_proxy_path(path);
    if !is_allowed_main_api_path(&target_path) {
        return Err(StatusCode::FORBIDDEN);
    }
    
    let target_url = format!("{}{}", MAIN_API_BASE, target_path);
    
    let mut query_params = query.clone();
    if !query_params.contains_key("user_ip") {
        if let Some(user_ip) = identity.user_ip {
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
    
    response_builder = apply_cors_headers(response_builder);
    
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
    let target_path = normalize_proxy_path(path);
    
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
    
    response_builder = apply_cors_headers(response_builder);
    
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
    let target_path = normalize_proxy_path(path);
    
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
    
    response_builder = apply_cors_headers(response_builder);
    
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
    client_ip: Extension<ClientIdentity>,
) -> Result<Response, StatusCode> {
    proxy_main_api_get(Path("".to_string()), Query(query), headers, client_ip).await
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
        .layer(middleware::from_fn(rate_limit_middleware))
}

#[cfg(feature = "ssr")]
async fn proxy_main_api_post(
    Path(path): Path<String>,
    Query(query): Query<std::collections::HashMap<String, String>>,
    headers: HeaderMap,
    client_ip: Extension<ClientIdentity>,
    body: axum::body::Body,
) -> Result<Response, StatusCode> {
    proxy_main_api_with_body(Method::POST, path, query, headers, Some(body), client_ip).await
}

#[cfg(feature = "ssr")]
async fn proxy_main_api_put(
    Path(path): Path<String>,
    Query(query): Query<std::collections::HashMap<String, String>>,
    headers: HeaderMap,
    client_ip: Extension<ClientIdentity>,
    body: axum::body::Body,
) -> Result<Response, StatusCode> {
    proxy_main_api_with_body(Method::PUT, path, query, headers, Some(body), client_ip).await
}

#[cfg(feature = "ssr")]
async fn proxy_main_api_delete(
    Path(path): Path<String>,
    Query(query): Query<std::collections::HashMap<String, String>>,
    headers: HeaderMap,
    client_ip: Extension<ClientIdentity>,
) -> Result<Response, StatusCode> {
    proxy_main_api_with_body(Method::DELETE, path, query, headers, None, client_ip).await
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
    Extension(identity): Extension<ClientIdentity>,
) -> Result<Response, StatusCode> {
    let target_path = normalize_proxy_path(path);
    if !is_allowed_main_api_path(&target_path) {
        return Err(StatusCode::FORBIDDEN);
    }
    
    let target_url = format!("{}{}", MAIN_API_BASE, target_path);
    
    let mut query_params = query.clone();
    if !query_params.contains_key("user_ip") {
        if let Some(user_ip) = identity.user_ip {
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
    
    if let Some(auth_header) = headers.get("authorization") {
        upstream_request = upstream_request.header("authorization", auth_header);
    }
    if let Some(content_type) = headers.get("content-type") {
        upstream_request = upstream_request.header("content-type", content_type);
    }
    upstream_request = upstream_request.header("User-Agent", "TorboxCompanion/1.0");
    
    if let Some(body) = body {
        let body_bytes = read_proxy_body(body).await?;
        upstream_request = upstream_request.body(body_bytes);
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
    
    response_builder = apply_cors_headers(response_builder);
    
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
    let target_path = normalize_proxy_path(path);
    
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
    
    if let Some(auth_header) = headers.get("authorization") {
        upstream_request = upstream_request.header("authorization", auth_header);
    }
    if let Some(content_type) = headers.get("content-type") {
        upstream_request = upstream_request.header("content-type", content_type);
    }
    upstream_request = upstream_request.header("User-Agent", "TorboxCompanion/1.0");
    
    if let Some(body) = body {
        let body_bytes = read_proxy_body(body).await?;
        upstream_request = upstream_request.body(body_bytes);
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
    
    response_builder = apply_cors_headers(response_builder);
    
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
    let target_path = normalize_proxy_path(path);
    
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
    
    if let Some(content_type) = headers.get("content-type") {
        upstream_request = upstream_request.header("content-type", content_type);
    }
    upstream_request = upstream_request.header("User-Agent", "TorboxCompanion/1.0");
    
    if let Some(body) = body {
        let body_bytes = read_proxy_body(body).await?;
        upstream_request = upstream_request.body(body_bytes);
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
    
    response_builder = apply_cors_headers(response_builder);
    
    let body_bytes = response
        .bytes()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;
    
    Ok(response_builder.body(Body::from(body_bytes)).unwrap())
}
