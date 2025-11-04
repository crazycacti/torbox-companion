#[cfg(feature = "ssr")]
use axum::{
    extract::Request,
    http::{HeaderMap, HeaderValue, StatusCode},
    middleware::Next,
    response::Response,
};
#[cfg(feature = "ssr")]
use leptos::logging::log;
#[cfg(feature = "ssr")]
use std::time::Instant;
#[cfg(feature = "ssr")]
use chrono::Local;

#[cfg(feature = "ssr")]
/// Logging middleware that logs request information according to privacy policy
/// Logs: method, path, status code, response time, timestamp
/// Does NOT log: API keys, request bodies, user data, personal information
pub async fn logging_middleware(request: Request, next: Next) -> Response {
    let start = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();
    let path = uri.path();
    let query = uri.query();

    let headers = request.headers();
    let sanitized_headers = sanitize_headers(headers);

    let response = next.run(request).await;
    let status = response.status();
    let duration = start.elapsed();

    let log_enabled = std::env::var("ENABLE_LOGGING")
        .unwrap_or_else(|_| "true".to_string())
        .parse::<bool>()
        .unwrap_or(true);

    let log_level = std::env::var("LOG_LEVEL")
        .unwrap_or_else(|_| "errors".to_string())
        .to_lowercase();

    let should_log = if !log_enabled {
        false
    } else if status >= StatusCode::BAD_REQUEST {
        true
    } else if log_level == "full" {
        true
    } else {
        false
    };

    if should_log {
        let timestamp = Local::now().format("%H:%M:%S");
        
        let duration_str = if duration.as_millis() > 1000 {
            format!("{:.2}s", duration.as_secs_f64())
        } else {
            format!("{}ms", duration.as_millis())
        };

        let status_icon = match status.as_u16() {
            200..=299 => "[OK]",
            300..=399 => "[REDIRECT]",
            400..=499 => "[CLIENT_ERR]",
            500..=599 => "[SERVER_ERR]",
            _ => "[?]",
        };

        let mut log_parts = vec![
            format!("[{}]", timestamp),
            format!("{}", method),
            path.to_string(),
        ];

        if let Some(query) = query {
            let query_display = if query.len() > 50 {
                format!("{}...", &query[..50])
            } else {
                query.to_string()
            };
            log_parts.push(format!("?{}", query_display));
        }

        log_parts.push(format!("{} {}", status_icon, status.as_u16()));

        log_parts.push(format!("({})", duration_str));

        if status >= StatusCode::BAD_REQUEST {
            log!("{} [ERROR]", log_parts.join(" "));
            if !sanitized_headers.is_empty() && sanitized_headers != "none" {
                let header_preview = if sanitized_headers.len() > 100 {
                    format!("{}...", &sanitized_headers[..100])
                } else {
                    sanitized_headers
                };
                log!("  -> Headers: {}", header_preview);
            }
        } else {
            log!("{}", log_parts.join(" "));
        }
    }

    response
}

#[cfg(feature = "ssr")]
/// Sanitize headers to remove sensitive information
/// Removes: authorization, api-key, cookie, and any header containing "key" or "token"
fn sanitize_headers(headers: &HeaderMap<HeaderValue>) -> String {
    let mut sanitized = Vec::new();

    for (name, _value) in headers.iter() {
        let name_lower = name.as_str().to_lowercase();
        
        if name_lower.contains("authorization")
            || name_lower.contains("api-key")
            || name_lower.contains("apikey")
            || name_lower.contains("cookie")
            || name_lower.contains("token")
            || name_lower.contains("x-api-key")
            || name_lower.contains("key")
        {
            sanitized.push(format!("{}: [REDACTED]", name));
        } else {
            sanitized.push(format!("{}: [present]", name));
        }
    }

    if sanitized.is_empty() {
        "none".to_string()
    } else {
        sanitized.join(", ")
    }
}

