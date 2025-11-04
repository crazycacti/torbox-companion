#[cfg(feature = "ssr")]
use axum::{
    extract::Query,
    http::{HeaderMap, StatusCode},
    response::Json,
    Router,
};
#[cfg(feature = "ssr")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use crate::api::TorboxClient;

#[cfg(feature = "ssr")]
fn extract_user_ip(headers: &HeaderMap) -> String {
    if let Some(forwarded) = headers.get("x-forwarded-for") {
        if let Ok(forwarded_str) = forwarded.to_str() {
            if let Some(ip) = forwarded_str.split(',').next() {
                return ip.trim().to_string();
            }
        }
    }
    
    if let Some(real_ip) = headers.get("x-real-ip") {
        if let Ok(ip_str) = real_ip.to_str() {
            return ip_str.to_string();
        }
    }
    
    if let Some(client_ip) = headers.get("x-client-ip") {
        if let Ok(ip_str) = client_ip.to_str() {
            return ip_str.to_string();
        }
    }
    
    "unknown".to_string()
}

#[cfg(feature = "ssr")]
#[derive(Deserialize)]
struct DownloadQuery {
    torrent_id: Option<i32>,
    web_id: Option<i32>,
    usenet_id: Option<i32>,
    file_id: Option<i32>,
    zip_link: Option<bool>,
}

#[cfg(feature = "ssr")]
#[derive(Serialize)]
struct DownloadResponse {
    success: bool,
    error: Option<String>,
    detail: String,
    data: Option<String>,
}

#[cfg(feature = "ssr")]
async fn handle_torrent_download(
    headers: HeaderMap,
    Query(params): Query<DownloadQuery>,
    api_key: String,
) -> Result<Json<DownloadResponse>, StatusCode> {
    let user_ip = extract_user_ip(&headers);
    
    let torrent_id = match params.torrent_id {
        Some(id) => id,
        None => return Ok(Json(DownloadResponse {
            success: false,
            error: Some("torrent_id is required".to_string()),
            detail: "Missing torrent_id parameter".to_string(),
            data: None,
        })),
    };
    
    let client = TorboxClient::new(api_key.clone());
    
    match client.request_download(
        api_key,
        torrent_id,
        params.file_id,
        params.zip_link,
        Some(user_ip),
        Some(false), // redirect: false for programmatic download
    ).await {
        Ok(response) => Ok(Json(DownloadResponse {
            success: response.success,
            error: response.error,
            detail: response.detail,
            data: response.data,
        })),
        Err(e) => {
            log::error!("Torrent download error: {:?}", e);
            Ok(Json(DownloadResponse {
                success: false,
                error: Some(format!("Download failed: {:?}", e)),
                detail: "Failed to request download".to_string(),
                data: None,
            }))
        }
    }
}

#[cfg(feature = "ssr")]
async fn handle_web_download(
    headers: HeaderMap,
    Query(params): Query<DownloadQuery>,
    api_key: String,
) -> Result<Json<DownloadResponse>, StatusCode> {
    let user_ip = extract_user_ip(&headers);
    
    let web_id = match params.web_id {
        Some(id) => id,
        None => return Ok(Json(DownloadResponse {
            success: false,
            error: Some("web_id is required".to_string()),
            detail: "Missing web_id parameter".to_string(),
            data: None,
        })),
    };
    
    let client = TorboxClient::new(api_key.clone());
    
    match client.request_web_download(
        api_key,
        web_id,
        params.file_id,
        params.zip_link,
        Some(user_ip),
        Some(false), // redirect: false for programmatic download
    ).await {
        Ok(response) => Ok(Json(DownloadResponse {
            success: response.success,
            error: response.error,
            detail: response.detail,
            data: response.data,
        })),
        Err(e) => {
            log::error!("Web download error: {:?}", e);
            Ok(Json(DownloadResponse {
                success: false,
                error: Some(format!("Download failed: {:?}", e)),
                detail: "Failed to request download".to_string(),
                data: None,
            }))
        }
    }
}

#[cfg(feature = "ssr")]
async fn handle_usenet_download(
    headers: HeaderMap,
    Query(params): Query<DownloadQuery>,
    api_key: String,
) -> Result<Json<DownloadResponse>, StatusCode> {
    let user_ip = extract_user_ip(&headers);
    
    let usenet_id = match params.usenet_id {
        Some(id) => id,
        None => return Ok(Json(DownloadResponse {
            success: false,
            error: Some("usenet_id is required".to_string()),
            detail: "Missing usenet_id parameter".to_string(),
            data: None,
        })),
    };
    
    let client = TorboxClient::new(api_key.clone());
    
    match client.request_usenet_download(
        api_key,
        usenet_id,
        params.file_id,
        params.zip_link,
        Some(user_ip),
        Some(false), // redirect: false for programmatic download
    ).await {
        Ok(response) => Ok(Json(DownloadResponse {
            success: response.success,
            error: response.error,
            detail: response.detail,
            data: response.data,
        })),
        Err(e) => {
            log::error!("Usenet download error: {:?}", e);
            Ok(Json(DownloadResponse {
                success: false,
                error: Some(format!("Download failed: {:?}", e)),
                detail: "Failed to request download".to_string(),
                data: None,
            }))
        }
    }
}

#[cfg(feature = "ssr")]
fn extract_api_key(headers: &HeaderMap, query_map: &std::collections::HashMap<String, String>) -> Option<String> {
    if let Some(auth_header) = headers.get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                return Some(auth_str[7..].to_string());
            }
        }
    }
    
    if let Some(api_key_header) = headers.get("x-api-key") {
        if let Ok(key_str) = api_key_header.to_str() {
            return Some(key_str.to_string());
        }
    }
    
    if let Some(key) = query_map.get("api_key") {
        return Some(key.clone());
    }
    
    None
}

#[cfg(feature = "ssr")]
pub fn create_download_routes() -> Router<()> {
    Router::new()
        .route("/api/torrents/download", axum::routing::get(handle_torrent_download_with_key))
        .route("/api/webdl/download", axum::routing::get(handle_web_download_with_key))
        .route("/api/usenet/download", axum::routing::get(handle_usenet_download_with_key))
}

#[cfg(feature = "ssr")]
async fn handle_torrent_download_with_key(
    headers: HeaderMap,
    Query(query): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<DownloadResponse>, StatusCode> {
    let api_key = match extract_api_key(&headers, &query) {
        Some(key) => key,
        None => {
            return Ok(Json(DownloadResponse {
                success: false,
                error: Some("API key is required".to_string()),
                detail: "Missing API key".to_string(),
                data: None,
            }));
        }
    };
    
    let download_params = DownloadQuery {
        torrent_id: query.get("torrent_id").and_then(|s| s.parse().ok()),
        web_id: None,
        usenet_id: None,
        file_id: query.get("file_id").and_then(|s| s.parse().ok()),
        zip_link: query.get("zip_link").and_then(|s| s.parse().ok()),
    };
    
    handle_torrent_download(headers, Query(download_params), api_key).await
}

#[cfg(feature = "ssr")]
async fn handle_web_download_with_key(
    headers: HeaderMap,
    Query(query): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<DownloadResponse>, StatusCode> {
    let api_key = match extract_api_key(&headers, &query) {
        Some(key) => key,
        None => {
            return Ok(Json(DownloadResponse {
                success: false,
                error: Some("API key is required".to_string()),
                detail: "Missing API key".to_string(),
                data: None,
            }));
        }
    };
    
    let download_params = DownloadQuery {
        torrent_id: None,
        web_id: query.get("web_id").and_then(|s| s.parse().ok()),
        usenet_id: None,
        file_id: query.get("file_id").and_then(|s| s.parse().ok()),
        zip_link: query.get("zip_link").and_then(|s| s.parse().ok()),
    };
    
    handle_web_download(headers, Query(download_params), api_key).await
}

#[cfg(feature = "ssr")]
async fn handle_usenet_download_with_key(
    headers: HeaderMap,
    Query(query): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<DownloadResponse>, StatusCode> {
    let api_key = match extract_api_key(&headers, &query) {
        Some(key) => key,
        None => {
            return Ok(Json(DownloadResponse {
                success: false,
                error: Some("API key is required".to_string()),
                detail: "Missing API key".to_string(),
                data: None,
            }));
        }
    };
    
    let download_params = DownloadQuery {
        torrent_id: None,
        web_id: None,
        usenet_id: query.get("usenet_id").and_then(|s| s.parse().ok()),
        file_id: query.get("file_id").and_then(|s| s.parse().ok()),
        zip_link: query.get("zip_link").and_then(|s| s.parse().ok()),
    };
    
    handle_usenet_download(headers, Query(download_params), api_key).await
}

