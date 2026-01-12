use crate::api::types::*;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct TorboxClient {
    client: Client,
    config: ApiConfig,
}

impl TorboxClient {
    pub fn new(api_key: String) -> Self {
        let mut config = ApiConfig::default();
        config.api_key = api_key;
        
        let client = Client::builder()
            .build()
            .expect("Failed to create HTTP client");

        Self { client, config }
    }

    pub fn with_config(config: ApiConfig) -> Self {
        let client = Client::builder()
            .build()
            .expect("Failed to create HTTP client");

        Self { client, config }
    }

    #[deprecated(note = "User IP is handled via URL parameters, use new() instead")]
    pub fn with_user_ip(api_key: String, _user_ip: Option<String>) -> Self {
        Self::new(api_key)
    }
    #[cfg(target_arch = "wasm32")]
    fn get_window_origin() -> String {
        web_sys::window()
            .and_then(|w| w.location().origin().ok())
            .expect("Failed to get window origin - window.location.origin() returned an error")
    }
    
    fn build_api_url(&self, path: &str) -> String {
        #[cfg(target_arch = "wasm32")]
        {
            let origin = Self::get_window_origin();
            if path.starts_with('/') {
                format!("{}/api{}", origin, path)
            } else {
                format!("{}/api/{}", origin, path)
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            if path.starts_with('/') {
                format!("{}{}", self.config.main_api_base, path)
            } else {
                format!("{}/{}", self.config.main_api_base, path)
            }
        }
    }
    
    fn build_search_api_url(&self, path: &str) -> String {
        #[cfg(target_arch = "wasm32")]
        {
            let origin = Self::get_window_origin();
            if path.starts_with('/') {
                format!("{}/api/search{}", origin, path)
            } else {
                format!("{}/api/search/{}", origin, path)
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            if path.starts_with('/') {
                format!("{}{}", self.config.search_api_base, path)
            } else {
                format!("{}/{}", self.config.search_api_base, path)
            }
        }
    }
    
    fn build_relay_api_url(&self, path: &str) -> String {
        #[cfg(target_arch = "wasm32")]
        {
            let origin = Self::get_window_origin();
            if path.starts_with('/') {
                format!("{}/api/relay{}", origin, path)
            } else {
                format!("{}/api/relay/{}", origin, path)
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            if path.starts_with('/') {
                format!("{}{}", self.config.relay_api_base, path)
            } else {
                format!("{}/{}", self.config.relay_api_base, path)
            }
        }
    }
    
    async fn request<T>(&self, method: reqwest::Method, url: String, body: Option<&T>) -> Result<Response, ApiError>
    where
        T: Serialize,
    {
        let mut request = self.client.request(method, &url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .header("User-Agent", "TorboxCompanion/1.0");

        if let Some(body_data) = body {
            request = request.json(body_data);
        }

        request.send().await.map_err(|_| {
            ApiError::NetworkError
        })
    }

    async fn handle_response<T>(response: Response) -> Result<ApiResponse<T>, ApiError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let status = response.status();
        let status_code = status.as_u16();
        
        if status.is_success() {
            let text = response.text().await.map_err(|e| {
                ApiError::HttpError { 
                    status_code, 
                    message: format!("Failed to read response: {}", e) 
                }
            })?;
            
            let api_response: ApiResponse<T> = serde_json::from_str(&text).map_err(|e| {
                ApiError::HttpError { 
                    status_code, 
                    message: format!("JSON parse error: {}", e) 
                }
            })?;
            
            if api_response.success {
                Ok(api_response)
            } else {
                let error_msg = api_response.error
                    .unwrap_or_else(|| api_response.detail.clone());
                Err(ApiError::HttpError { 
                    status_code, 
                    message: error_msg
                })
            }
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            
            match status_code {
                401 => Err(ApiError::HttpError { 
                    status_code, 
                    message: format!("Authentication failed: {}", error_text) 
                }),
                404 => Err(ApiError::HttpError { 
                    status_code, 
                    message: format!("Resource not found: {}", error_text) 
                }),
                422 => Err(ApiError::HttpError { 
                    status_code, 
                    message: format!("Validation error: {}", error_text) 
                }),
                429 => Err(ApiError::HttpError { 
                    status_code, 
                    message: format!("Rate limit exceeded: {}", error_text) 
                }),
                _ => Err(ApiError::HttpError { 
                    status_code, 
                    message: format!("Server error ({}): {}", status_code, error_text) 
                }),
            }
        }
    }

    async fn handle_search_response<T>(response: Response) -> Result<SearchApiResponse<T>, ApiError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let status = response.status();
        let status_code = status.as_u16();
        
        if status.is_success() {
            let text = response.text().await.map_err(|e| {
                ApiError::HttpError { 
                    status_code, 
                    message: format!("Failed to read response: {}", e) 
                }
            })?;
            
            let search_response: SearchApiResponse<T> = serde_json::from_str(&text).map_err(|e| {
                ApiError::HttpError { 
                    status_code, 
                    message: format!("JSON parse error: {}", e) 
                }
            })?;
            
            if search_response.success {
                Ok(search_response)
            } else {
                Err(ApiError::HttpError { 
                    status_code, 
                    message: search_response.message.clone()
                })
            }
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            
            match status_code {
                401 => Err(ApiError::HttpError { 
                    status_code, 
                    message: format!("Authentication failed: {}", error_text) 
                }),
                404 => Err(ApiError::HttpError { 
                    status_code, 
                    message: format!("Resource not found: {}", error_text) 
                }),
                422 => Err(ApiError::HttpError { 
                    status_code, 
                    message: format!("Validation error: {}", error_text) 
                }),
                429 => Err(ApiError::HttpError { 
                    status_code, 
                    message: format!("Rate limit exceeded: {}", error_text) 
                }),
                _ => Err(ApiError::HttpError { 
                    status_code, 
                    message: format!("Server error ({}): {}", status_code, error_text) 
                }),
            }
        }
    }

    pub async fn get_user(&self, settings: Option<bool>) -> Result<ApiResponse<User>, ApiError> {
        let mut url = self.build_api_url("/v1/api/user/me");
        if let Some(include_settings) = settings {
            url.push_str(&format!("?settings={}", include_settings));
        }
        
        let response = self.request(reqwest::Method::GET, url, None::<&()>).await?;
        Self::handle_response(response).await
    }

    pub async fn check_status(&self) -> Result<ApiResponse<()>, ApiError> {
        let url = self.build_api_url("/");
        let response = self.request(reqwest::Method::GET, url, None::<&()>).await?;
        Self::handle_response(response).await
    }

    pub async fn refresh_token(&self, session_token: String) -> Result<ApiResponse<String>, ApiError> {
        let url = self.build_api_url("/v1/api/user/refreshtoken");
        let body = serde_json::json!({ "session_token": session_token });
        
        let response = self.request(reqwest::Method::POST, url, Some(&body)).await?;
        Self::handle_response(response).await
    }

    pub async fn get_confirmation_code(&self) -> Result<ApiResponse<String>, ApiError> {
        let url = self.build_api_url("/v1/api/user/getconfirmation");
        
        let response = self.request(reqwest::Method::GET, url, None::<&()>).await?;
        Self::handle_response(response).await
    }

    pub async fn add_referral(&self, referral: String) -> Result<ApiResponse<String>, ApiError> {
        let url = format!("{}?referral={}", self.build_api_url("/v1/api/user/addreferral"), referral);
        
        let response = self.request(reqwest::Method::POST, url, None::<&()>).await?;
        Self::handle_response(response).await
    }

    pub async fn get_referral_data(&self) -> Result<ApiResponse<serde_json::Value>, ApiError> {
        let url = self.build_api_url("/v1/api/user/referraldata");
        
        let response = self.request(reqwest::Method::GET, url, None::<&()>).await?;
        Self::handle_response(response).await
    }

    pub async fn get_subscriptions(&self) -> Result<ApiResponse<serde_json::Value>, ApiError> {
        let url = self.build_api_url("/v1/api/user/subscriptions");
        
        let response = self.request(reqwest::Method::GET, url, None::<&()>).await?;
        Self::handle_response(response).await
    }

    pub async fn get_transactions(&self) -> Result<ApiResponse<serde_json::Value>, ApiError> {
        let url = self.build_api_url("/v1/api/user/transactions");
        
        let response = self.request(reqwest::Method::GET, url, None::<&()>).await?;
        Self::handle_response(response).await
    }

    // Torrent Management Methods
    pub async fn create_torrent(&self, request: CreateTorrentRequest) -> Result<ApiResponse<serde_json::Value>, ApiError> {
        let url = self.build_api_url("/v1/api/torrents/createtorrent");
        
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            use web_sys::{FormData, window};
            
            let form_data = FormData::new().map_err(|_| ApiError::NetworkError)?;
            
            if let Some(magnet) = &request.magnet {
                form_data.append_with_str("magnet", magnet).map_err(|_| ApiError::NetworkError)?;
            }
            
            if let Some(name) = &request.name {
                form_data.append_with_str("name", name).map_err(|_| ApiError::NetworkError)?;
            }
            
            if let Some(seed) = request.seed {
                form_data.append_with_str("seed", &seed.to_string()).map_err(|_| ApiError::NetworkError)?;
            }
            
            if let Some(allow_zip) = request.allow_zip {
                form_data.append_with_str("allow_zip", &allow_zip.to_string()).map_err(|_| ApiError::NetworkError)?;
            }
            
            if let Some(as_queued) = request.as_queued {
                form_data.append_with_str("as_queued", &as_queued.to_string()).map_err(|_| ApiError::NetworkError)?;
            }
            
            if let Some(add_only_if_cached) = request.add_only_if_cached {
                form_data.append_with_str("add_only_if_cached", &add_only_if_cached.to_string()).map_err(|_| ApiError::NetworkError)?;
            }
            
            let window = window().ok_or(ApiError::NetworkError)?;
            let mut init = web_sys::RequestInit::new();
            init.set_method("POST");
            let headers = web_sys::Headers::new().map_err(|_| ApiError::NetworkError)?;
            headers.append("Authorization", &format!("Bearer {}", self.config.api_key)).map_err(|_| ApiError::NetworkError)?;
            headers.append("User-Agent", "TorboxCompanion/1.0").map_err(|_| ApiError::NetworkError)?;
            init.set_headers(&headers);
            init.set_body(form_data.as_ref() as &wasm_bindgen::JsValue);
            let promise = window.fetch_with_str_and_init(&url, &init);
            
            let resp = wasm_bindgen_futures::JsFuture::from(promise)
                .await
                .map_err(|_| ApiError::NetworkError)?;
            let resp: web_sys::Response = resp.dyn_into().map_err(|_| ApiError::NetworkError)?;
            
            let text_promise = resp.text().map_err(|_| ApiError::NetworkError)?;
            let text = wasm_bindgen_futures::JsFuture::from(text_promise)
                .await
                .map_err(|_| ApiError::NetworkError)?;
            let text = text.as_string().ok_or(ApiError::NetworkError)?;
            
            let status = resp.status();
            let api_response: ApiResponse<serde_json::Value> = serde_json::from_str(&text).map_err(|e| {
                ApiError::HttpError { 
                    status_code: status, 
                    message: format!("JSON parse error: {}", e) 
                }
            })?;
            
            if api_response.success {
                Ok(api_response)
            } else {
                Err(ApiError::HttpError { 
                    status_code: status, 
                    message: api_response.detail.clone()
                })
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            use reqwest::multipart;
            
            let mut form = multipart::Form::new();
            
            if let Some(magnet) = &request.magnet {
                form = form.text("magnet", magnet.clone());
            }
            
            if let Some(name) = &request.name {
                form = form.text("name", name.clone());
            }
            
            if let Some(seed) = request.seed {
                form = form.text("seed", seed.to_string());
            }
            
            if let Some(allow_zip) = request.allow_zip {
                form = form.text("allow_zip", allow_zip.to_string());
            }
            
            if let Some(as_queued) = request.as_queued {
                form = form.text("as_queued", as_queued.to_string());
            }
            
            if let Some(add_only_if_cached) = request.add_only_if_cached {
                form = form.text("add_only_if_cached", add_only_if_cached.to_string());
            }
            
            let response = self.client.post(&url)
                .header("Authorization", format!("Bearer {}", self.config.api_key))
                .header("User-Agent", "TorboxCompanion/1.0")
                .multipart(form)
                .send()
                .await
                .map_err(|e| ApiError::NetworkError)?;
            
            Self::handle_response(response).await
        }
    }

    pub async fn async_create_torrent(&self, request: CreateTorrentRequest) -> Result<ApiResponse<serde_json::Value>, ApiError> {
        let url = self.build_api_url("/v1/api/torrents/asynccreatetorrent");
        
        let response = self.request(reqwest::Method::POST, url, Some(&request)).await?;
        Self::handle_response(response).await
    }

    pub async fn control_torrent(&self, operation: String, torrent_id: i32, all: bool) -> Result<ApiResponse<String>, ApiError> {
        let url = self.build_api_url("/v1/api/torrents/controltorrent");
        let body = ControlRequest { operation, torrent_id, all };
        
        let response = self.request(reqwest::Method::POST, url, Some(&body)).await?;
        Self::handle_response(response).await
    }

    pub async fn get_torrent_list(&self, id: Option<i32>, bypass_cache: Option<bool>, offset: Option<i32>, limit: Option<i32>) -> Result<ApiResponse<Vec<Torrent>>, ApiError> {
        let mut url = self.build_api_url("/v1/api/torrents/mylist");
        let mut params = Vec::new();
        
        if let Some(torrent_id) = id {
            params.push(format!("id={}", torrent_id));
        }
        if let Some(bypass) = bypass_cache {
            params.push(format!("bypass_cache={}", bypass));
        }
        if let Some(off) = offset {
            params.push(format!("offset={}", off));
        }
        if let Some(lim) = limit {
            params.push(format!("limit={}", lim));
        }
        
        if !params.is_empty() {
            url.push_str(&format!("?{}", params.join("&")));
        }
        
        let response = self.request(reqwest::Method::GET, url, None::<&()>).await?;
        Self::handle_response(response).await
    }

    #[deprecated(note = "This endpoint is deprecated. Use get_queued_downloads with type='torrent' instead.")]
    pub async fn get_queued_torrents(&self) -> Result<ApiResponse<Vec<Torrent>>, ApiError> {
        let url = self.build_api_url("/v1/api/torrents/getqueued");
        
        let response = self.request(reqwest::Method::GET, url, None::<&()>).await?;
        Self::handle_response(response).await
    }

    fn build_download_url(&self, base_path: &str, token: String, id: i32, file_id: Option<i32>, zip_link: Option<bool>, user_ip: Option<String>, redirect: Option<bool>) -> String {
        let mut url = format!("{}?token={}", self.build_api_url(base_path), token);
        
        if base_path.contains("/torrents/") {
            url.push_str(&format!("&torrent_id={}", id));
        } else if base_path.contains("/webdl/") {
            url.push_str(&format!("&web_id={}", id));
        } else if base_path.contains("/usenet/") {
            url.push_str(&format!("&usenet_id={}", id));
        }
        
        if let Some(fid) = file_id {
            url.push_str(&format!("&file_id={}", fid));
        }
        if let Some(zip) = zip_link {
            url.push_str(&format!("&zip_link={}", zip));
        }
        if let Some(ip) = user_ip {
            url.push_str(&format!("&user_ip={}", ip));
        }
        if let Some(redir) = redirect {
            url.push_str(&format!("&redirect={}", redir));
        }
        url
    }

    pub async fn request_download(&self, token: String, torrent_id: i32, file_id: Option<i32>, zip_link: Option<bool>, user_ip: Option<String>, redirect: Option<bool>) -> Result<ApiResponse<String>, ApiError> {
        let url = self.build_download_url("/v1/api/torrents/requestdl", token, torrent_id, file_id, zip_link, user_ip, redirect);
        let response = self.request(reqwest::Method::GET, url, None::<&()>).await?;
        Self::handle_response(response).await
    }

    pub async fn request_web_download(&self, token: String, web_id: i32, file_id: Option<i32>, zip_link: Option<bool>, user_ip: Option<String>, redirect: Option<bool>) -> Result<ApiResponse<String>, ApiError> {
        let url = self.build_download_url("/v1/api/webdl/requestdl", token, web_id, file_id, zip_link, user_ip, redirect);
        let response = self.request(reqwest::Method::GET, url, None::<&()>).await?;
        Self::handle_response(response).await
    }

    pub async fn request_usenet_download(&self, token: String, usenet_id: i32, file_id: Option<i32>, zip_link: Option<bool>, user_ip: Option<String>, redirect: Option<bool>) -> Result<ApiResponse<String>, ApiError> {
        let url = self.build_download_url("/v1/api/usenet/requestdl", token, usenet_id, file_id, zip_link, user_ip, redirect);
        let response = self.request(reqwest::Method::GET, url, None::<&()>).await?;
        Self::handle_response(response).await
    }

    pub async fn create_web_download(&self, request: CreateWebDownloadRequest) -> Result<ApiResponse<serde_json::Value>, ApiError> {
        let url = self.build_api_url("/v1/api/webdl/createwebdownload");
        
        let response = self.request(reqwest::Method::POST, url, Some(&request)).await?;
        Self::handle_response(response).await
    }

    pub async fn async_create_web_download(&self, request: CreateWebDownloadRequest) -> Result<ApiResponse<serde_json::Value>, ApiError> {
        let url = self.build_api_url("/v1/api/webdl/asynccreatewebdownload");
        
        let response = self.request(reqwest::Method::POST, url, Some(&request)).await?;
        Self::handle_response(response).await
    }

    pub async fn control_web_download(&self, operation: String, webdl_id: i32, all: bool) -> Result<ApiResponse<String>, ApiError> {
        let url = self.build_api_url("/v1/api/webdl/controlwebdownload");
        let body = ControlWebDownloadRequest { operation, webdl_id, all };
        
        let response = self.request(reqwest::Method::POST, url, Some(&body)).await?;
        Self::handle_response(response).await
    }

    pub async fn get_web_download_list(&self, id: Option<i32>, bypass_cache: Option<bool>, offset: Option<i32>, limit: Option<i32>) -> Result<ApiResponse<Vec<WebDownload>>, ApiError> {
        let mut url = self.build_api_url("/v1/api/webdl/mylist");
        let mut params = Vec::new();
        
        if let Some(web_id) = id {
            params.push(format!("id={}", web_id));
        }
        if let Some(bypass) = bypass_cache {
            params.push(format!("bypass_cache={}", bypass));
        }
        if let Some(off) = offset {
            params.push(format!("offset={}", off));
        }
        if let Some(lim) = limit {
            params.push(format!("limit={}", lim));
        }
        
        if !params.is_empty() {
            url.push_str(&format!("?{}", params.join("&")));
        }
        
        let response = self.request(reqwest::Method::GET, url, None::<&()>).await?;
        Self::handle_response(response).await
    }

    pub async fn create_usenet_download(&self, request: CreateUsenetDownloadRequest) -> Result<ApiResponse<serde_json::Value>, ApiError> {
        let url = self.build_api_url("/v1/api/usenet/createusenetdownload");
        
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            use web_sys::{FormData, window};
            
            let form_data = FormData::new().map_err(|_| ApiError::NetworkError)?;
            
            if let Some(link) = &request.link {
                form_data.append_with_str("link", link).map_err(|_| ApiError::NetworkError)?;
            }
            
            if let Some(name) = &request.name {
                form_data.append_with_str("name", name).map_err(|_| ApiError::NetworkError)?;
            }
            
            if let Some(password) = &request.password {
                form_data.append_with_str("password", password).map_err(|_| ApiError::NetworkError)?;
            }
            
            if let Some(post_processing) = request.post_processing {
                form_data.append_with_str("post_processing", &post_processing.to_string()).map_err(|_| ApiError::NetworkError)?;
            }
            
            if let Some(as_queued) = request.as_queued {
                form_data.append_with_str("as_queued", &as_queued.to_string()).map_err(|_| ApiError::NetworkError)?;
            }
            
            if let Some(add_only_if_cached) = request.add_only_if_cached {
                form_data.append_with_str("add_only_if_cached", &add_only_if_cached.to_string()).map_err(|_| ApiError::NetworkError)?;
            }
            
            let window = window().ok_or(ApiError::NetworkError)?;
            let mut init = web_sys::RequestInit::new();
            init.set_method("POST");
            let headers = web_sys::Headers::new().map_err(|_| ApiError::NetworkError)?;
            headers.append("Authorization", &format!("Bearer {}", self.config.api_key)).map_err(|_| ApiError::NetworkError)?;
            headers.append("User-Agent", "TorboxCompanion/1.0").map_err(|_| ApiError::NetworkError)?;
            init.set_headers(&headers);
            init.set_body(form_data.as_ref() as &wasm_bindgen::JsValue);
            let promise = window.fetch_with_str_and_init(&url, &init);
            
            let resp = wasm_bindgen_futures::JsFuture::from(promise)
                .await
                .map_err(|_| ApiError::NetworkError)?;
            let resp: web_sys::Response = resp.dyn_into().map_err(|_| ApiError::NetworkError)?;
            
            let text_promise = resp.text().map_err(|_| ApiError::NetworkError)?;
            let text = wasm_bindgen_futures::JsFuture::from(text_promise)
                .await
                .map_err(|_| ApiError::NetworkError)?;
            let text = text.as_string().ok_or(ApiError::NetworkError)?;
            
            let status = resp.status();
            let api_response: ApiResponse<serde_json::Value> = serde_json::from_str(&text).map_err(|e| {
                ApiError::HttpError { 
                    status_code: status, 
                    message: format!("JSON parse error: {}", e) 
                }
            })?;
            
            if api_response.success {
                Ok(api_response)
            } else {
                Err(ApiError::HttpError { 
                    status_code: status, 
                    message: api_response.detail.clone()
                })
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            use reqwest::multipart;
            
            let mut form = multipart::Form::new();
            
            if let Some(link) = &request.link {
                form = form.text("link", link.clone());
            }
            
            if let Some(name) = &request.name {
                form = form.text("name", name.clone());
            }
            
            if let Some(password) = &request.password {
                form = form.text("password", password.clone());
            }
            
            if let Some(post_processing) = request.post_processing {
                form = form.text("post_processing", post_processing.to_string());
            }
            
            if let Some(as_queued) = request.as_queued {
                form = form.text("as_queued", as_queued.to_string());
            }
            
            if let Some(add_only_if_cached) = request.add_only_if_cached {
                form = form.text("add_only_if_cached", add_only_if_cached.to_string());
            }
            
            let response = self.client.post(&url)
                .header("Authorization", format!("Bearer {}", self.config.api_key))
                .header("User-Agent", "TorboxCompanion/1.0")
                .multipart(form)
                .send()
                .await
                .map_err(|e| ApiError::NetworkError)?;
            
            Self::handle_response(response).await
        }
    }

    pub async fn async_create_usenet_download(&self, request: CreateUsenetDownloadRequest) -> Result<ApiResponse<serde_json::Value>, ApiError> {
        let url = self.build_api_url("/v1/api/usenet/asynccreateusenetdownload");
        
        let response = self.request(reqwest::Method::POST, url, Some(&request)).await?;
        Self::handle_response(response).await
    }

    pub async fn control_usenet_download(&self, operation: String, usenet_id: i32, all: bool) -> Result<ApiResponse<String>, ApiError> {
        let url = self.build_api_url("/v1/api/usenet/controlusenetdownload");
        let body = ControlUsenetRequest { operation, usenet_id, all };
        
        let response = self.request(reqwest::Method::POST, url, Some(&body)).await?;
        Self::handle_response(response).await
    }

    pub async fn get_usenet_download_list(&self, id: Option<i32>, bypass_cache: Option<bool>, offset: Option<i32>, limit: Option<i32>) -> Result<ApiResponse<Vec<UsenetDownload>>, ApiError> {
        let mut url = self.build_api_url("/v1/api/usenet/mylist");
        let mut params = Vec::new();
        
        if let Some(usenet_id) = id {
            params.push(format!("id={}", usenet_id));
        }
        if let Some(bypass) = bypass_cache {
            params.push(format!("bypass_cache={}", bypass));
        }
        if let Some(off) = offset {
            params.push(format!("offset={}", off));
        }
        if let Some(lim) = limit {
            params.push(format!("limit={}", lim));
        }
        
        if !params.is_empty() {
            url.push_str(&format!("?{}", params.join("&")));
        }
        
        let response = self.request(reqwest::Method::GET, url, None::<&()>).await?;
        Self::handle_response(response).await
    }

    pub async fn add_rss_feed(&self, request: CreateRssFeedRequest) -> Result<ApiResponse<serde_json::Value>, ApiError> {
        let url = self.build_api_url("/v1/api/rss/addrss");
        
        let response = self.request(reqwest::Method::POST, url, Some(&request)).await?;
        Self::handle_response(response).await
    }

    pub async fn get_rss_feeds(&self, id: Option<i32>) -> Result<ApiResponse<Vec<RssFeed>>, ApiError> {
        let mut url = self.build_api_url("/v1/api/rss/getfeeds");
        if let Some(feed_id) = id {
            url.push_str(&format!("?id={}", feed_id));
        }
        
        let response = self.request(reqwest::Method::GET, url, None::<&()>).await?;
        Self::handle_response(response).await
    }

    pub async fn control_rss_feed(&self, operation: String, rss_feed_id: i32) -> Result<ApiResponse<String>, ApiError> {
        let url = self.build_api_url("/v1/api/rss/controlrss");
        let body = serde_json::json!({
            "operation": operation,
            "rss_feed_id": rss_feed_id
        });
        
        let response = self.request(reqwest::Method::POST, url, Some(&body)).await?;
        Self::handle_response(response).await
    }

    pub async fn create_stream(&self, request: CreateStreamRequest) -> Result<ApiResponse<Stream>, ApiError> {
        let mut url = format!("{}?id={}", self.build_api_url("/v1/api/stream/createstream"), request.id);
        
        if let Some(file_id) = request.file_id {
            url.push_str(&format!("&file_id={}", file_id));
        }
        if let Some(stream_type) = request.r#type {
            url.push_str(&format!("&type={}", stream_type));
        }
        if let Some(subtitle_index) = request.chosen_subtitle_index {
            url.push_str(&format!("&chosen_subtitle_index={}", subtitle_index));
        }
        if let Some(audio_index) = request.chosen_audio_index {
            url.push_str(&format!("&chosen_audio_index={}", audio_index));
        }
        
        let response = self.request(reqwest::Method::GET, url, None::<&()>).await?;
        Self::handle_response(response).await
    }

    pub async fn get_stream_data(&self, token: String, presigned_token: String, chosen_subtitle_index: Option<i32>, chosen_audio_index: Option<i32>) -> Result<ApiResponse<Stream>, ApiError> {
        let mut url = format!("{}?token={}&presigned_token={}", self.build_api_url("/v1/api/stream/getstreamdata"), token, presigned_token);
        
        if let Some(subtitle_index) = chosen_subtitle_index {
            url.push_str(&format!("&chosen_subtitle_index={}", subtitle_index));
        }
        if let Some(audio_index) = chosen_audio_index {
            url.push_str(&format!("&chosen_audio_index={}", audio_index));
        }
        
        let response = self.request(reqwest::Method::GET, url, None::<&()>).await?;
        Self::handle_response(response).await
    }

    pub async fn get_metadata(&self, id_type: String, id: String) -> Result<SearchApiResponse<SearchMetadata>, ApiError> {
        let url = self.build_search_api_url(&format!("/meta/{}:{}", id_type, id));
        
        let response = self.client.get(&url)
            .header("User-Agent", "TorboxCompanion/1.0")
            .send()
            .await
            .map_err(|_| ApiError::NetworkError)?;
        Self::handle_search_response(response).await
    }

    pub async fn get_torrents_by_imdb(&self, imdb_id: String) -> Result<SearchApiResponse<SearchTorrentsResponse>, ApiError> {
        let url = self.build_search_api_url(&format!("/torrents/imdb:{}", imdb_id));
        
        let response = self.client.get(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("User-Agent", "TorboxCompanion/1.0")
            .send()
            .await
            .map_err(|_| ApiError::NetworkError)?;
        Self::handle_search_response(response).await
    }

    pub async fn get_usenet_by_imdb(&self, imdb_id: String) -> Result<SearchApiResponse<SearchUsenetResponse>, ApiError> {
        let url = self.build_search_api_url(&format!("/usenet/imdb:{}", imdb_id));
        
        let response = self.client.get(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("User-Agent", "TorboxCompanion/1.0")
            .send()
            .await
            .map_err(|_| ApiError::NetworkError)?;
        Self::handle_search_response(response).await
    }

    pub async fn search_torrents(&self, query: String, metadata: Option<bool>, check_cache: Option<bool>, check_owned: Option<bool>, search_user_engines: Option<bool>) -> Result<SearchApiResponse<SearchTorrentsResponse>, ApiError> {
        let mut url = self.build_search_api_url(&format!("/torrents/search/{}", query));
        let mut params = Vec::new();
        
        if let Some(m) = metadata {
            params.push(format!("metadata={}", m));
        }
        if let Some(c) = check_cache {
            params.push(format!("check_cache={}", c));
        }
        if let Some(o) = check_owned {
            params.push(format!("check_owned={}", o));
        }
        if let Some(s) = search_user_engines {
            params.push(format!("search_user_engines={}", s));
        }
        
        if !params.is_empty() {
            url.push_str(&format!("?{}", params.join("&")));
        }
        
        let response = self.client.get(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("User-Agent", "TorboxCompanion/1.0")
            .send()
            .await
            .map_err(|_| ApiError::NetworkError)?;
        Self::handle_search_response(response).await
    }

    pub async fn search_usenet(&self, query: String, metadata: Option<bool>, season: Option<i32>, episode: Option<i32>, check_cache: Option<bool>, check_owned: Option<bool>, search_user_engines: Option<bool>) -> Result<SearchApiResponse<SearchUsenetResponse>, ApiError> {
        let mut url = self.build_search_api_url(&format!("/usenet/search/{}", query));
        let mut params = Vec::new();
        
        if let Some(m) = metadata {
            params.push(format!("metadata={}", m));
        }
        if let Some(s) = season {
            params.push(format!("season={}", s));
        }
        if let Some(e) = episode {
            params.push(format!("episode={}", e));
        }
        if let Some(c) = check_cache {
            params.push(format!("check_cache={}", c));
        }
        if let Some(o) = check_owned {
            params.push(format!("check_owned={}", o));
        }
        if let Some(s) = search_user_engines {
            params.push(format!("search_user_engines={}", s));
        }
        
        if !params.is_empty() {
            url.push_str(&format!("?{}", params.join("&")));
        }
        
        let response = self.client.get(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("User-Agent", "TorboxCompanion/1.0")
            .send()
            .await
            .map_err(|_| ApiError::NetworkError)?;
        Self::handle_search_response(response).await
    }

    pub async fn search_metadata(&self, query: String) -> Result<SearchApiResponse<Vec<SearchMetadata>>, ApiError> {
        let url = self.build_search_api_url(&format!("/search/{}", query));
        
        let response = self.client.get(&url)
            .header("User-Agent", "TorboxCompanion/1.0")
            .send()
            .await
            .map_err(|_| ApiError::NetworkError)?;
        Self::handle_search_response(response).await
    }

    pub async fn get_relay_status(&self) -> Result<RelayStatus, ApiError> {
        let url = self.build_relay_api_url("/");
        
        let response = self.client.get(&url)
            .header("User-Agent", "TorboxCompanion/1.0")
            .send()
            .await
            .map_err(|_| ApiError::NetworkError)?;
        response.json().await.map_err(|_| ApiError::ServerError)
    }

    pub async fn request_torrent_update(&self, user_id: String, torrent_id: i32) -> Result<ApiResponse<serde_json::Value>, ApiError> {
        let url = self.build_relay_api_url(&format!("/v1/inactivecheck/torrent/{}/{}", user_id, torrent_id));
        
        let response = self.client.get(&url)
            .header("User-Agent", "TorboxCompanion/1.0")
            .send()
            .await
            .map_err(|_| ApiError::NetworkError)?;
        Self::handle_response(response).await
    }

    pub async fn get_notifications(&self) -> Result<ApiResponse<Vec<Notification>>, ApiError> {
        let url = self.build_api_url("/v1/api/notifications/mynotifications");
        
        let response = self.request(reqwest::Method::GET, url, None::<&()>).await?;
        Self::handle_response(response).await
    }

    pub async fn clear_all_notifications(&self) -> Result<ApiResponse<String>, ApiError> {
        let url = self.build_api_url("/v1/api/notifications/clear");
        
        let response = self.request(reqwest::Method::POST, url, None::<&()>).await?;
        Self::handle_response(response).await
    }

    pub async fn clear_notification(&self, notification_id: String) -> Result<ApiResponse<String>, ApiError> {
        let url = format!("{}/{}", self.build_api_url("/v1/api/notifications/clear"), notification_id);
        
        let response = self.request(reqwest::Method::POST, url, None::<&()>).await?;
        Self::handle_response(response).await
    }

    pub async fn test_notification(&self) -> Result<ApiResponse<String>, ApiError> {
        let url = self.build_api_url("/v1/api/notifications/test");
        
        let response = self.request(reqwest::Method::POST, url, None::<&()>).await?;
        Self::handle_response(response).await
    }

    pub async fn upload_to_google_drive(&self, request: CloudUpload) -> Result<ApiResponse<serde_json::Value>, ApiError> {
        let url = self.build_api_url("/v1/api/integration/googledrive");
        
        let response = self.request(reqwest::Method::POST, url, Some(&request)).await?;
        Self::handle_response(response).await
    }

    pub async fn upload_to_dropbox(&self, request: CloudUpload) -> Result<ApiResponse<serde_json::Value>, ApiError> {
        let url = self.build_api_url("/v1/api/integration/dropbox");
        
        let response = self.request(reqwest::Method::POST, url, Some(&request)).await?;
        Self::handle_response(response).await
    }

    pub async fn upload_to_onedrive(&self, request: CloudUpload) -> Result<ApiResponse<serde_json::Value>, ApiError> {
        let url = self.build_api_url("/v1/api/integration/onedrive");
        
        let response = self.request(reqwest::Method::POST, url, Some(&request)).await?;
        Self::handle_response(response).await
    }

    pub async fn get_transfer_jobs(&self) -> Result<ApiResponse<Vec<TransferJob>>, ApiError> {
        let url = self.build_api_url("/v1/api/integration/jobs");
        
        let response = self.request(reqwest::Method::GET, url, None::<&()>).await?;
        Self::handle_response(response).await
    }

    pub async fn cancel_transfer_job(&self, job_id: i32) -> Result<ApiResponse<String>, ApiError> {
        let url = format!("{}/{}", self.build_api_url("/v1/api/integration/job"), job_id);
        
        let response = self.request(reqwest::Method::DELETE, url, None::<&()>).await?;
        Self::handle_response(response).await
    }

    pub fn get_config(&self) -> &ApiConfig {
        &self.config
    }

    pub fn update_api_key(&mut self, api_key: String) {
        self.config.api_key = api_key;
    }

    pub async fn get_speedtest_files(&self, user_ip: Option<String>, region: Option<String>, test_length: Option<String>) -> Result<ApiResponse<String>, ApiError> {
        let mut url = self.build_api_url("/v1/api/speedtest");
        let mut params = Vec::new();
        
        if let Some(ip) = user_ip {
            params.push(format!("user_ip={}", ip));
        }
        if let Some(reg) = region {
            params.push(format!("region={}", reg));
        }
        if let Some(length) = test_length {
            params.push(format!("test_length={}", length));
        }
        
        if !params.is_empty() {
            url.push_str(&format!("?{}", params.join("&")));
        }
        
        let response = self.request(reqwest::Method::GET, url, None::<&()>).await?;
        Self::handle_response(response).await
    }

    pub async fn get_queued_downloads(&self, download_type: Option<String>, id: Option<i32>, bypass_cache: Option<bool>, offset: Option<i32>, limit: Option<i32>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
        let mut url = self.build_api_url("/v1/api/queued/getqueued");
        let mut params = Vec::new();
        
        if let Some(dl_type) = download_type {
            params.push(format!("type={}", dl_type));
        }
        if let Some(queue_id) = id {
            params.push(format!("id={}", queue_id));
        }
        if let Some(bypass) = bypass_cache {
            params.push(format!("bypass_cache={}", bypass));
        }
        if let Some(off) = offset {
            params.push(format!("offset={}", off));
        }
        if let Some(lim) = limit {
            params.push(format!("limit={}", lim));
        }
        
        if !params.is_empty() {
            url.push_str(&format!("?{}", params.join("&")));
        }
        
        let response = self.request(reqwest::Method::GET, url, None::<&()>).await?;
        Self::handle_response(response).await
    }

    pub async fn control_queued_downloads(&self, operation: String, queued_id: Option<i32>, all: Option<bool>) -> Result<ApiResponse<String>, ApiError> {
        let url = self.build_api_url("/v1/api/queued/controlqueued");
        let body = serde_json::json!({
            "operation": operation,
            "queued_id": queued_id,
            "all": all.unwrap_or(false)
        });
        
        let response = self.request(reqwest::Method::POST, url, Some(&body)).await?;
        Self::handle_response(response).await
    }

    // Check cached endpoints
    pub async fn check_torrent_cached(&self, hashes: Vec<String>, format: Option<String>, list_files: Option<bool>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
        let mut url = self.build_api_url("/v1/api/torrents/checkcached");
        let mut params = Vec::new();
        
        if !hashes.is_empty() {
            for hash in &hashes {
                params.push(format!("hash={}", hash));
            }
        }
        if let Some(fmt) = format {
            params.push(format!("format={}", fmt));
        }
        if let Some(list) = list_files {
            params.push(format!("list_files={}", list));
        }
        
        if !params.is_empty() {
            url.push_str(&format!("?{}", params.join("&")));
        }
        
        let response = self.request(reqwest::Method::GET, url, None::<&()>).await?;
        Self::handle_response(response).await
    }

    pub async fn check_torrent_cached_post(&self, hashes: Vec<String>, format: Option<String>, list_files: Option<bool>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
        let url = self.build_api_url("/v1/api/torrents/checkcached");
        let mut body = serde_json::json!({ "hashes": hashes });
        
        if let Some(fmt) = format {
            body.as_object_mut().unwrap().insert("format".to_string(), serde_json::json!(fmt));
        }
        if let Some(list) = list_files {
            body.as_object_mut().unwrap().insert("list_files".to_string(), serde_json::json!(list));
        }
        
        let response = self.request(reqwest::Method::POST, url, Some(&body)).await?;
        Self::handle_response(response).await
    }

    pub async fn check_webdl_cached(&self, hashes: Vec<String>, format: Option<String>, list_files: Option<bool>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
        let mut url = self.build_api_url("/v1/api/webdl/checkcached");
        let mut params = Vec::new();
        
        if !hashes.is_empty() {
            for hash in &hashes {
                params.push(format!("hash={}", hash));
            }
        }
        if let Some(fmt) = format {
            params.push(format!("format={}", fmt));
        }
        if let Some(list) = list_files {
            params.push(format!("list_files={}", list));
        }
        
        if !params.is_empty() {
            url.push_str(&format!("?{}", params.join("&")));
        }
        
        let response = self.request(reqwest::Method::GET, url, None::<&()>).await?;
        Self::handle_response(response).await
    }

    pub async fn check_webdl_cached_post(&self, hashes: Vec<String>, format: Option<String>, list_files: Option<bool>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
        let url = self.build_api_url("/v1/api/webdl/checkcached");
        let mut body = serde_json::json!({ "hashes": hashes });
        
        if let Some(fmt) = format {
            body.as_object_mut().unwrap().insert("format".to_string(), serde_json::json!(fmt));
        }
        if let Some(list) = list_files {
            body.as_object_mut().unwrap().insert("list_files".to_string(), serde_json::json!(list));
        }
        
        let response = self.request(reqwest::Method::POST, url, Some(&body)).await?;
        Self::handle_response(response).await
    }

    pub async fn check_usenet_cached(&self, hashes: Vec<String>, format: Option<String>, list_files: Option<bool>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
        let mut url = self.build_api_url("/v1/api/usenet/checkcached");
        let mut params = Vec::new();
        
        if !hashes.is_empty() {
            for hash in &hashes {
                params.push(format!("hash={}", hash));
            }
        }
        if let Some(fmt) = format {
            params.push(format!("format={}", fmt));
        }
        if let Some(list) = list_files {
            params.push(format!("list_files={}", list));
        }
        
        if !params.is_empty() {
            url.push_str(&format!("?{}", params.join("&")));
        }
        
        let response = self.request(reqwest::Method::GET, url, None::<&()>).await?;
        Self::handle_response(response).await
    }

    pub async fn check_usenet_cached_post(&self, hashes: Vec<String>, format: Option<String>, list_files: Option<bool>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
        let url = self.build_api_url("/v1/api/usenet/checkcached");
        let mut body = serde_json::json!({ "hashes": hashes });
        
        if let Some(fmt) = format {
            body.as_object_mut().unwrap().insert("format".to_string(), serde_json::json!(fmt));
        }
        if let Some(list) = list_files {
            body.as_object_mut().unwrap().insert("list_files".to_string(), serde_json::json!(list));
        }
        
        let response = self.request(reqwest::Method::POST, url, Some(&body)).await?;
        Self::handle_response(response).await
    }

    // Torrent info endpoints
    pub async fn get_torrent_info(&self, hash: String, timeout: Option<i32>, use_cache_lookup: Option<bool>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
        let mut url = format!("{}?hash={}", self.build_api_url("/v1/api/torrents/torrentinfo"), hash);
        
        if let Some(to) = timeout {
            url.push_str(&format!("&timeout={}", to));
        }
        if let Some(cache) = use_cache_lookup {
            url.push_str(&format!("&use_cache_lookup={}", cache));
        }
        
        let response = self.request(reqwest::Method::GET, url, None::<&()>).await?;
        Self::handle_response(response).await
    }

    pub async fn get_torrent_info_post(&self, hash: Option<String>, magnet: Option<String>, file: Option<String>, timeout: Option<i32>, use_cache_lookup: Option<bool>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
        let url = self.build_api_url("/v1/api/torrents/torrentinfo");
        let mut body = serde_json::json!({});
        
        if let Some(h) = hash {
            body.as_object_mut().unwrap().insert("hash".to_string(), serde_json::json!(h));
        }
        if let Some(m) = magnet {
            body.as_object_mut().unwrap().insert("magnet".to_string(), serde_json::json!(m));
        }
        if let Some(f) = file {
            body.as_object_mut().unwrap().insert("file".to_string(), serde_json::json!(f));
        }
        if let Some(to) = timeout {
            body.as_object_mut().unwrap().insert("timeout".to_string(), serde_json::json!(to));
        }
        if let Some(cache) = use_cache_lookup {
            body.as_object_mut().unwrap().insert("use_cache_lookup".to_string(), serde_json::json!(cache));
        }
        
        let response = self.request(reqwest::Method::POST, url, Some(&body)).await?;
        Self::handle_response(response).await
    }

    // Export torrent data
    pub async fn export_torrent_data(&self, torrent_id: i32, export_type: String) -> Result<ApiResponse<String>, ApiError> {
        let url = format!("{}?torrent_id={}&type={}", self.build_api_url("/v1/api/torrents/exportdata"), torrent_id, export_type);
        
        let response = self.request(reqwest::Method::GET, url, None::<&()>).await?;
        Self::handle_response(response).await
    }

    // Convert magnet to file
    pub async fn magnet_to_file(&self, magnet: String) -> Result<ApiResponse<String>, ApiError> {
        let url = self.build_api_url("/v1/api/torrents/magnettofile");
        let body = serde_json::json!({ "magnet": magnet });
        
        let response = self.request(reqwest::Method::POST, url, Some(&body)).await?;
        Self::handle_response(response).await
    }
}
