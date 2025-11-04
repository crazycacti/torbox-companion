use crate::api::{TorboxClient, ApiConfig, ApiError};
use crate::api::types::*;
use leptos::prelude::*;
use std::collections::HashMap;

/// Central request handler for all TorBox API operations
/// This demonstrates how to utilize every single API endpoint
pub struct RequestHandler {
    client: TorboxClient,
}

impl RequestHandler {
    pub fn new(api_key: String) -> Self {
        let client = TorboxClient::new(api_key);
        Self { client }
    }

    pub fn with_config(config: ApiConfig) -> Self {
        let client = TorboxClient::with_config(config);
        Self { client }
    }

    pub fn with_user_ip(api_key: String, user_ip: Option<String>) -> Self {
        let client = TorboxClient::with_user_ip(api_key, user_ip);
        Self { client }
    }
    
    /// Get current user information with optional settings
    pub async fn get_user_info(&self, include_settings: bool) -> Result<User, ApiError> {
        let response = self.client.get_user(Some(include_settings)).await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Refresh API token using session token
    pub async fn refresh_api_token(&self, session_token: String) -> Result<String, ApiError> {
        let response = self.client.refresh_token(session_token).await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Get confirmation code for account verification
    pub async fn get_confirmation_code(&self) -> Result<String, ApiError> {
        let response = self.client.get_confirmation_code().await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Add referral code to user account
    pub async fn add_referral(&self, referral_code: String) -> Result<String, ApiError> {
        let response = self.client.add_referral(referral_code).await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Get user's referral data
    pub async fn get_referral_data(&self) -> Result<serde_json::Value, ApiError> {
        let response = self.client.get_referral_data().await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Get user's subscription information
    pub async fn get_subscriptions(&self) -> Result<serde_json::Value, ApiError> {
        let response = self.client.get_subscriptions().await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Get user's transaction history
    pub async fn get_transactions(&self) -> Result<serde_json::Value, ApiError> {
        let response = self.client.get_transactions().await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Create a new torrent from file or magnet link
    pub async fn create_torrent(&self, request: CreateTorrentRequest) -> Result<serde_json::Value, ApiError> {
        let response = self.client.create_torrent(request).await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Create torrent asynchronously (returns immediately)
    pub async fn create_torrent_async(&self, request: CreateTorrentRequest) -> Result<serde_json::Value, ApiError> {
        let response = self.client.async_create_torrent(request).await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Control torrent operations (start, stop, pause, resume, delete)
    pub async fn control_torrent(&self, operation: String, torrent_id: i32, all: bool) -> Result<String, ApiError> {
        let response = self.client.control_torrent(operation, torrent_id, all).await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Get list of user's torrents with pagination
    pub async fn get_torrent_list(&self, id: Option<i32>, bypass_cache: Option<bool>, offset: Option<i32>, limit: Option<i32>) -> Result<Vec<Torrent>, ApiError> {
        let response = self.client.get_torrent_list(id, bypass_cache, offset, limit).await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Get queued torrents
    pub async fn get_queued_torrents(&self) -> Result<Vec<Torrent>, ApiError> {
        let response = self.client.get_queued_torrents().await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Request download link for torrent
    pub async fn request_torrent_download(&self, token: String, torrent_id: i32, file_id: Option<i32>, zip_link: Option<bool>, user_ip: Option<String>, redirect: Option<bool>) -> Result<String, ApiError> {
        let response = self.client.request_download(token, torrent_id, file_id, zip_link, user_ip, redirect).await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Request download link for web download
    pub async fn request_web_download(&self, token: String, web_id: i32, file_id: Option<i32>, zip_link: Option<bool>, user_ip: Option<String>, redirect: Option<bool>) -> Result<String, ApiError> {
        let response = self.client.request_web_download(token, web_id, file_id, zip_link, user_ip, redirect).await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Request download link for usenet download
    pub async fn request_usenet_download(&self, token: String, usenet_id: i32, file_id: Option<i32>, zip_link: Option<bool>, user_ip: Option<String>, redirect: Option<bool>) -> Result<String, ApiError> {
        let response = self.client.request_usenet_download(token, usenet_id, file_id, zip_link, user_ip, redirect).await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Get speedtest files with user IP for CDN optimization
    pub async fn get_speedtest_files(&self, user_ip: Option<String>, region: Option<String>) -> Result<String, ApiError> {
        let response = self.client.get_speedtest_files(user_ip, region).await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Create web download from URL
    pub async fn create_web_download(&self, request: CreateWebDownloadRequest) -> Result<serde_json::Value, ApiError> {
        let response = self.client.create_web_download(request).await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Create web download asynchronously
    pub async fn create_web_download_async(&self, request: CreateWebDownloadRequest) -> Result<serde_json::Value, ApiError> {
        let response = self.client.async_create_web_download(request).await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Control web download operations
    pub async fn control_web_download(&self, operation: String, webdl_id: i32, all: bool) -> Result<String, ApiError> {
        let response = self.client.control_web_download(operation, webdl_id, all).await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Get list of web downloads
    pub async fn get_web_download_list(&self, id: Option<i32>, bypass_cache: Option<bool>, offset: Option<i32>, limit: Option<i32>) -> Result<Vec<WebDownload>, ApiError> {
        let response = self.client.get_web_download_list(id, bypass_cache, offset, limit).await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Create usenet download from link or NZB file
    pub async fn create_usenet_download(&self, request: CreateUsenetDownloadRequest) -> Result<serde_json::Value, ApiError> {
        let response = self.client.create_usenet_download(request).await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Create usenet download asynchronously
    pub async fn create_usenet_download_async(&self, request: CreateUsenetDownloadRequest) -> Result<serde_json::Value, ApiError> {
        let response = self.client.async_create_usenet_download(request).await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Control usenet download operations
    pub async fn control_usenet_download(&self, operation: String, usenet_id: i32, all: bool) -> Result<String, ApiError> {
        let response = self.client.control_usenet_download(operation, usenet_id, all).await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Get list of usenet downloads
    pub async fn get_usenet_download_list(&self, id: Option<i32>, bypass_cache: Option<bool>, offset: Option<i32>, limit: Option<i32>) -> Result<Vec<UsenetDownload>, ApiError> {
        let response = self.client.get_usenet_download_list(id, bypass_cache, offset, limit).await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Add new RSS feed
    pub async fn add_rss_feed(&self, request: CreateRssFeedRequest) -> Result<serde_json::Value, ApiError> {
        let response = self.client.add_rss_feed(request).await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Get RSS feeds
    pub async fn get_rss_feeds(&self, id: Option<i32>) -> Result<Vec<RssFeed>, ApiError> {
        let response = self.client.get_rss_feeds(id).await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Control RSS feed operations
    pub async fn control_rss_feed(&self, operation: String, rss_feed_id: i32) -> Result<String, ApiError> {
        let response = self.client.control_rss_feed(operation, rss_feed_id).await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Create stream from TorBox item
    pub async fn create_stream(&self, request: CreateStreamRequest) -> Result<Stream, ApiError> {
        let response = self.client.create_stream(request).await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Get stream data with subtitles and audio tracks
    pub async fn get_stream_data(&self, token: String, presigned_token: String, chosen_subtitle_index: Option<i32>, chosen_audio_index: Option<i32>) -> Result<Stream, ApiError> {
        let response = self.client.get_stream_data(token, presigned_token, chosen_subtitle_index, chosen_audio_index).await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Get metadata by ID (IMDB, TMDB, etc.)
    pub async fn get_metadata(&self, id_type: String, id: String) -> Result<SearchMetadata, ApiError> {
        let response = self.client.get_metadata(id_type, id).await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Search for torrents
    pub async fn search_torrents(&self, query: String) -> Result<Vec<SearchTorrent>, ApiError> {
        let response = self.client.search_torrents(
            query,
            Some(false),
            Some(false),
            Some(false),
            Some(false),
        ).await?;
        Ok(response.data.ok_or(ApiError::ServerError)?.torrents)
    }

    /// Search for usenet posts
    pub async fn search_usenet(&self, query: String) -> Result<Vec<SearchUsenet>, ApiError> {
        let response = self.client.search_usenet(
            query,
            Some(false),
            None,
            None,
            Some(false),
            Some(false),
            Some(false),
        ).await?;
        Ok(response.data.ok_or(ApiError::ServerError)?.nzbs)
    }

    /// Search for metadata by title
    pub async fn search_metadata(&self, query: String) -> Result<Vec<SearchMetadata>, ApiError> {
        let response = self.client.search_metadata(query).await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Get relay status and user statistics
    pub async fn get_relay_status(&self) -> Result<RelayStatus, ApiError> {
        self.client.get_relay_status().await
    }

    /// Request torrent update from relay
    pub async fn request_torrent_update(&self, user_id: String, torrent_id: i32) -> Result<serde_json::Value, ApiError> {
        let response = self.client.request_torrent_update(user_id, torrent_id).await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Get user notifications
    pub async fn get_notifications(&self) -> Result<Vec<Notification>, ApiError> {
        let response = self.client.get_notifications().await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Clear all notifications
    pub async fn clear_all_notifications(&self) -> Result<String, ApiError> {
        let response = self.client.clear_all_notifications().await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Clear specific notification
    pub async fn clear_notification(&self, notification_id: String) -> Result<String, ApiError> {
        let response = self.client.clear_notification(notification_id).await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Send test notification
    pub async fn test_notification(&self) -> Result<String, ApiError> {
        let response = self.client.test_notification().await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Upload to Google Drive
    pub async fn upload_to_google_drive(&self, request: CloudUpload) -> Result<serde_json::Value, ApiError> {
        let response = self.client.upload_to_google_drive(request).await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Upload to Dropbox
    pub async fn upload_to_dropbox(&self, request: CloudUpload) -> Result<serde_json::Value, ApiError> {
        let response = self.client.upload_to_dropbox(request).await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Upload to OneDrive
    pub async fn upload_to_onedrive(&self, request: CloudUpload) -> Result<serde_json::Value, ApiError> {
        let response = self.client.upload_to_onedrive(request).await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Get transfer jobs
    pub async fn get_transfer_jobs(&self) -> Result<Vec<TransferJob>, ApiError> {
        let response = self.client.get_transfer_jobs().await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Cancel transfer job
    pub async fn cancel_transfer_job(&self, job_id: i32) -> Result<String, ApiError> {
        let response = self.client.cancel_transfer_job(job_id).await?;
        response.data.ok_or(ApiError::ServerError)
    }

    /// Get client configuration
    pub fn get_config(&self) -> &ApiConfig {
        self.client.get_config()
    }

    /// Update API key
    pub fn update_api_key(&mut self, api_key: String) {
        self.client.update_api_key(api_key);
    }

    /// Test API connection
    pub async fn test_connection(&self) -> Result<bool, ApiError> {
        match self.get_user_info(false).await {
            Ok(_) => Ok(true),
            Err(ApiError::AuthenticationError) => Ok(false),
            Err(e) => Err(e),
        }
    }
}

pub fn create_handler(api_key: String) -> RequestHandler {
    RequestHandler::new(api_key)
}

pub fn create_handler_with_config(config: ApiConfig) -> RequestHandler {
    RequestHandler::with_config(config)
}

pub fn create_handler_with_user_ip(api_key: String, user_ip: Option<String>) -> RequestHandler {
    RequestHandler::with_user_ip(api_key, user_ip)
}

pub async fn demonstrate_all_apis(api_key: String) -> Result<(), ApiError> {
    let mut handler = create_handler(api_key);

    // Test connection
    if !handler.test_connection().await? {
        return Err(ApiError::AuthenticationError);
    }

    let user = handler.get_user_info(true).await?;
    println!("User: {:?}", user);

    let subscriptions = handler.get_subscriptions().await?;
    println!("Subscriptions: {:?}", subscriptions);

    let search_results = handler.search_metadata("Star Wars".to_string()).await?;
    println!("Search results: {:?}", search_results);

    let torrents = handler.get_torrent_list(None, None, Some(0), Some(10)).await?;
    println!("Torrents: {:?}", torrents);

    let web_downloads = handler.get_web_download_list(None, None, Some(0), Some(10)).await?;
    println!("Web downloads: {:?}", web_downloads);

    let usenet_downloads = handler.get_usenet_download_list(None, None, Some(0), Some(10)).await?;
    println!("Usenet downloads: {:?}", usenet_downloads);

    let rss_feeds = handler.get_rss_feeds(None).await?;
    println!("RSS feeds: {:?}", rss_feeds);

    let notifications = handler.get_notifications().await?;
    println!("Notifications: {:?}", notifications);

    let relay_status = handler.get_relay_status().await?;
    println!("Relay status: {:?}", relay_status);

    let transfer_jobs = handler.get_transfer_jobs().await?;
    println!("Transfer jobs: {:?}", transfer_jobs);

    Ok(())
}
