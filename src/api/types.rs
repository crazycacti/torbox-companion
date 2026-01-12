use serde::{Deserialize, Serialize, Deserializer};

fn deserialize_categories<'de, D>(deserializer: D) -> Result<Option<Vec<serde_json::Value>>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
    match value {
        serde_json::Value::Null => Ok(None),
        serde_json::Value::Array(arr) => Ok(Some(arr)),
        _ => Ok(Some(vec![value])),
    }
}

fn deserialize_files<'de, D>(deserializer: D) -> Result<Option<serde_json::Value>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
    match value {
        serde_json::Value::Null => Ok(None),
        _ => Ok(Some(value)),
    }
}

fn deserialize_optional_int<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
    match value {
        serde_json::Value::Null => Ok(None),
        serde_json::Value::Number(n) => {
            n.as_i64()
                .and_then(|v| i32::try_from(v).ok())
                .map(Some)
                .ok_or_else(|| D::Error::custom("Invalid integer"))
        }
        serde_json::Value::Array(arr) => {
            if let Some(first) = arr.first() {
                if let Some(n) = first.as_i64() {
                    if let Ok(season_num) = i32::try_from(n) {
                        return Ok(Some(season_num));
                    }
                }
            }
            Ok(None)
        }
        _ => Ok(None),
    }
}

fn deserialize_size<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
    match value {
        serde_json::Value::Number(n) => {
            n.as_u64()
                .ok_or_else(|| D::Error::custom("Invalid u64 number"))
        }
        serde_json::Value::String(s) => {
            s.parse::<u64>()
                .map_err(|e| D::Error::custom(format!("Failed to parse size string '{}': {}", s, e)))
        }
        _ => Err(D::Error::custom(format!("Expected number or string for size, got: {:?}", value))),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub error: Option<String>,
    pub detail: String,
    pub data: Option<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchApiResponse<T> {
    pub success: bool,
    pub message: String,
    #[serde(default)]
    pub error: Option<String>,
    pub data: Option<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub auth_id: String,
    pub created_at: String,
    pub updated_at: String,
    pub plan: i32,
    pub total_downloaded: i32,
    pub customer: String,
    pub is_subscribed: bool,
    pub premium_expires_at: String,
    pub cooldown_until: String,
    pub email: String,
    pub user_referral: String,
    pub base_email: String,
    pub total_bytes_downloaded: i64,
    pub total_bytes_uploaded: i64,
    pub torrents_downloaded: i32,
    pub web_downloads_downloaded: i32,
    pub usenet_downloads_downloaded: i32,
    pub additional_concurrent_slots: i32,
    pub long_term_seeding: bool,
    pub long_term_storage: bool,
    pub is_vendor: bool,
    pub vendor_id: Option<String>,
    pub purchases_referred: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettings {
    pub search_engines: Vec<SearchEngine>,
    pub notifications: NotificationSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEngine {
    pub id: i32,
    pub r#type: String,
    pub url: String,
    pub apikey: String,
    pub download_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSettings {
    pub email: bool,
    pub push: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Torrent {
    pub id: i32,
    pub auth_id: String,
    pub server: i32,
    pub hash: String,
    pub name: String,
    pub magnet: Option<String>,
    pub size: i64,
    pub active: bool,
    pub created_at: String,
    pub updated_at: String,
    pub download_state: String,
    pub seeds: i32,
    pub peers: i32,
    pub ratio: f32,
    pub progress: f32,
    pub download_speed: i64,
    pub upload_speed: i64,
    pub eta: i32,
    pub torrent_file: bool,
    pub expires_at: Option<String>,
    pub download_present: bool,
    pub files: Option<Vec<TorrentFile>>,
    pub download_path: Option<String>,
    pub availability: f32,
    pub download_finished: bool,
    pub tracker: Option<String>,
    pub total_uploaded: i64,
    pub total_downloaded: i64,
    pub cached: bool,
    pub owner: String,
    pub seed_torrent: bool,
    pub allow_zipped: bool,
    pub long_term_seeding: bool,
    pub tracker_message: Option<String>,
    pub cached_at: Option<String>,
    pub private: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorrentFile {
    pub id: i32,
    pub md5: Option<String>,
    pub hash: Option<String>,
    pub name: String,
    pub size: i64,
    #[serde(default)]
    pub zipped: bool,
    pub s3_path: String,
    #[serde(default)]
    pub infected: bool,
    pub mimetype: String,
    pub short_name: String,
    pub absolute_path: String,
    pub opensubtitles_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTorrentRequest {
    pub file: Option<String>,
    pub magnet: Option<String>,
    pub seed: Option<i32>,
    pub allow_zip: Option<bool>,
    pub name: Option<String>,
    pub as_queued: Option<bool>,
    pub add_only_if_cached: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebDownload {
    pub id: i32,
    pub hash: String,
    pub name: String,
    pub auth_id: String,
    pub server: i32,
    pub size: i64,
    pub active: bool,
    pub created_at: String,
    pub updated_at: String,
    pub download_state: String,
    pub progress: f32,
    pub download_speed: i64,
    pub upload_speed: i64,
    pub eta: i32,
    pub torrent_file: bool,
    pub expires_at: Option<String>,
    pub download_present: bool,
    pub download_finished: bool,
    pub error: Option<String>,
    pub files: Vec<WebDownloadFile>,
    pub inactive_check: i32,
    pub availability: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebDownloadFile {
    pub id: i32,
    pub md5: Option<String>,
    pub s3_path: Option<String>,
    pub name: String,
    pub size: i64,
    pub mimetype: Option<String>,
    pub short_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWebDownloadRequest {
    pub link: String,
    pub password: Option<String>,
    pub name: Option<String>,
    pub as_queued: Option<bool>,
    pub add_only_if_cached: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsenetDownload {
    pub id: i32,
    pub auth_id: String,
    pub server: Option<i32>,
    pub hash: String,
    pub name: String,
    pub download_id: String,
    pub original_url: Option<String>,
    pub size: i64,
    pub active: bool,
    pub created_at: String,
    pub updated_at: String,
    pub download_state: String,
    pub download_speed: i64,
    pub eta: i32,
    pub progress: f32,
    pub expires_at: Option<String>,
    pub download_present: bool,
    pub download_finished: bool,
    pub cached: bool,
    pub cached_at: Option<String>,
    pub files: Vec<UsenetFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsenetFile {
    pub id: i32,
    pub md5: Option<String>,
    pub hash: Option<String>,
    pub name: String,
    pub size: i64,
    #[serde(default)]
    pub zipped: bool,
    pub s3_path: String,
    #[serde(default)]
    pub infected: bool,
    pub mimetype: String,
    pub short_name: String,
    pub absolute_path: String,
    pub opensubtitles_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUsenetDownloadRequest {
    pub link: Option<String>,
    pub file: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>,
    pub post_processing: Option<i32>,
    pub as_queued: Option<bool>,
    pub add_only_if_cached: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RssFeed {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub do_regex: Option<String>,
    pub dont_regex: Option<String>,
    pub dont_older_than: i32,
    pub pass_check: bool,
    pub scan_interval: Vec<i32>,
    pub rss_type: Vec<String>,
    pub torrent_seeding: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRssFeedRequest {
    pub url: String,
    pub name: String,
    pub do_regex: String,
    pub dont_regex: String,
    pub dont_older_than: i32,
    pub pass_check: bool,
    pub scan_interval: Vec<i32>,
    pub rss_type: Vec<String>,
    pub torrent_seeding: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stream {
    #[serde(default, alias = "token")]
    pub user_token: Option<String>,
    #[serde(alias = "file_token")]
    pub presigned_token: String,
    #[serde(alias = "hls_url")]
    pub stream_url: String,
    #[serde(default)]
    pub subtitles: Option<Vec<Subtitle>>,
    #[serde(default)]
    pub audio_tracks: Option<Vec<AudioTrack>>,
    #[serde(default)]
    pub domain: Option<String>,
    #[serde(default)]
    pub subtitle_index: Option<String>,
    #[serde(default)]
    pub audio_index: Option<i32>,
    #[serde(default)]
    pub is_transcoding: Option<bool>,
    #[serde(default)]
    pub needs_transcoding: Option<bool>,
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subtitle {
    pub index: i32,
    pub language: String,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioTrack {
    pub index: i32,
    pub language: String,
    pub name: String,
    pub channels: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStreamRequest {
    pub id: i32,
    pub file_id: Option<i32>,
    pub r#type: Option<String>,
    pub chosen_subtitle_index: Option<i32>,
    pub chosen_audio_index: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMetadata {
    #[serde(rename = "globalID")]
    pub global_id: String,
    pub id: String,
    pub title: String,
    pub titles: Vec<String>,
    pub description: Option<String>,
    #[serde(rename = "releasedDate")]
    pub release_date: Option<String>,
    pub genres: Vec<String>,
    pub rating: Option<f32>,
    pub image: Option<String>,
    pub backdrop: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TitleParsedData {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_int")]
    pub year: Option<i32>,
    #[serde(default)]
    pub resolution: Option<String>,
    #[serde(default)]
    pub quality: Option<String>,
    #[serde(default)]
    pub codec: Option<String>,
    #[serde(default)]
    pub encoder: Option<String>,
    #[serde(default)]
    pub audio: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_int")]
    pub season: Option<i32>,
    #[serde(default, deserialize_with = "deserialize_optional_int")]
    pub episode: Option<i32>,
    #[serde(flatten, default)]
    pub extra: serde_json::Value,
}

impl Default for TitleParsedData {
    fn default() -> Self {
        Self {
            title: None,
            year: None,
            resolution: None,
            quality: None,
            codec: None,
            encoder: None,
            audio: None,
            season: None,
            episode: None,
            extra: serde_json::Value::Object(serde_json::Map::new()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchTorrent {
    pub hash: String,
    #[serde(rename = "raw_title")]
    pub raw_title: String,
    pub title: String,
    #[serde(rename = "title_parsed_data", default)]
    pub title_parsed_data: Option<TitleParsedData>,
    pub magnet: Option<String>,
    #[serde(rename = "last_known_seeders", deserialize_with = "deserialize_optional_int")]
    pub last_known_seeders: Option<i32>,
    #[serde(rename = "last_known_peers", deserialize_with = "deserialize_optional_int")]
    pub last_known_peers: Option<i32>,
    #[serde(deserialize_with = "deserialize_size")]
    pub size: u64,
    pub tracker: Option<String>,
    #[serde(deserialize_with = "deserialize_categories")]
    pub categories: Option<Vec<serde_json::Value>>,
    #[serde(deserialize_with = "deserialize_files")]
    pub files: Option<serde_json::Value>,
    pub cached: Option<bool>,
    pub owned: Option<bool>,
    #[serde(rename = "upload_date")]
    pub upload_date: Option<String>,
    #[serde(default)]
    pub private: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchUsenet {
    pub hash: String,
    #[serde(rename = "raw_title")]
    pub raw_title: String,
    pub title: String,
    #[serde(rename = "title_parsed_data", default)]
    pub title_parsed_data: Option<TitleParsedData>,
    pub magnet: Option<String>,
    pub torrent: Option<String>,
    #[serde(rename = "last_known_seeders", deserialize_with = "deserialize_optional_int")]
    pub last_known_seeders: Option<i32>,
    #[serde(rename = "last_known_peers", deserialize_with = "deserialize_optional_int")]
    pub last_known_peers: Option<i32>,
    #[serde(deserialize_with = "deserialize_size")]
    pub size: u64,
    pub tracker: Option<String>,
    #[serde(deserialize_with = "deserialize_categories")]
    pub categories: Option<Vec<serde_json::Value>>,
    #[serde(deserialize_with = "deserialize_files")]
    pub files: Option<serde_json::Value>,
    pub nzb: Option<String>,
    pub age: Option<String>,
    pub r#type: Option<String>,
    #[serde(rename = "user_search")]
    pub user_search: Option<bool>,
    pub cached: Option<bool>,
    pub owned: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchTorrentsResponse {
    pub metadata: Option<SearchMetadata>,
    pub torrents: Vec<SearchTorrent>,
    #[serde(rename = "time_taken")]
    pub time_taken: Option<f64>,
    pub cached: Option<bool>,
    #[serde(rename = "total_torrents")]
    pub total_torrents: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchUsenetResponse {
    pub metadata: Option<SearchMetadata>,
    pub nzbs: Vec<SearchUsenet>,
    #[serde(rename = "time_taken")]
    pub time_taken: Option<f64>,
    pub cached: Option<bool>,
    #[serde(rename = "total_nzbs")]
    pub total_nzbs: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayStatus {
    pub status: String,
    pub data: RelayData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayData {
    pub current_online: i32,
    pub latency: Option<f32>,
    pub requests_per_second: Option<i32>,
    pub worker_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudUpload {
    pub id: i32,
    pub file_id: i32,
    pub zip: bool,
    pub r#type: String,
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferJob {
    pub job_id: i32,
    pub status: String,
    pub progress: f32,
    pub source: String,
    pub destination: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: String,
    pub title: String,
    pub message: String,
    pub r#type: String,
    pub created_at: String,
    pub read: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorAccount {
    pub vendor_name: String,
    pub vendor_url: String,
    pub accounts: Vec<VendorUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorUser {
    pub user_auth_id: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiError {
    AuthenticationError,
    RateLimitError,
    ValidationError,
    NotFoundError,
    ServerError,
    NetworkError,
    HttpError { status_code: u16, message: String },
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::AuthenticationError => write!(f, "Authentication failed"),
            ApiError::RateLimitError => write!(f, "Rate limit exceeded"),
            ApiError::ValidationError => write!(f, "Validation error"),
            ApiError::NotFoundError => write!(f, "Resource not found"),
            ApiError::ServerError => write!(f, "Server error"),
            ApiError::NetworkError => write!(f, "Network error"),
            ApiError::HttpError { status_code, message } => {
                write!(f, "HTTP {}: {}", status_code, message)
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: i32,
    pub offset: i32,
    pub limit: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlRequest {
    pub operation: String,
    pub torrent_id: i32,
    pub all: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlWebDownloadRequest {
    pub operation: String,
    pub webdl_id: i32,
    pub all: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlUsenetRequest {
    pub operation: String,
    pub usenet_id: i32,
    pub all: bool,
}

// API Configuration
#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub main_api_base: String,
    pub search_api_base: String,
    pub relay_api_base: String,
    pub stream_api_base: String,
    pub api_key: String,
    pub timeout: u64,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            main_api_base: "https://api.torbox.app".to_string(),
            search_api_base: "https://search-api.torbox.app".to_string(),
            relay_api_base: "https://relay.torbox.app".to_string(),
            stream_api_base: "/api/stream".to_string(),
            api_key: String::new(),
            timeout: 30,
        }
    }
}
