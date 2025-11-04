use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::logging::log;
#[cfg(target_arch = "wasm32")]
use web_sys;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use js_sys;
use crate::api::{TorboxClient, Torrent, WebDownload, UsenetDownload};
use crate::dashboard::DashboardContext;
use crate::dashboard::components::loading_spinner::{LoadingSpinner, SpinnerSize, SpinnerVariant};
use chrono::DateTime;
use serde::{Serialize, Deserialize};
use std::collections::{HashSet, HashMap};
use futures;
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DownloadItem {
    pub id: i32,
    pub name: String,
    pub size: i64,
    pub created_at: String,
    pub status: String,
    pub download_type: DownloadType,
    pub progress: f32,
    pub download_speed: i64,
    pub upload_speed: i64,
    pub active: bool,
    pub files: Vec<DownloadFile>,
    pub is_season: bool,
    pub season_info: Option<SeasonInfo>,
    pub eta: Option<i32>,
    pub total_downloaded: Option<i64>,
    pub private: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DownloadFile {
    pub id: i32,
    pub name: String,
    pub size: i64,
    pub md5: Option<String>,
    pub hash: Option<String>,
    pub zipped: Option<bool>,
    pub s3_path: Option<String>,
    pub infected: Option<bool>,
    pub mimetype: Option<String>,
    pub short_name: Option<String>,
    pub absolute_path: Option<String>,
    pub opensubtitles_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SeasonInfo {
    pub season_number: Option<i32>,
    pub episode_count: i32,
    pub episodes: Vec<EpisodeInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EpisodeInfo {
    pub episode_number: i32,
    pub name: String,
    pub size: i64,
    pub file_id: i32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DownloadType {
    Torrent,
    WebDownload,
    Usenet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionState {
    pub selected_items: HashSet<i32>,
    pub selected_files: HashMap<i32, HashSet<i32>>,
}

impl Default for SelectionState {
    fn default() -> Self {
        Self {
            selected_items: HashSet::new(),
            selected_files: HashMap::new(),
        }
    }
}

impl SelectionState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn has_selected_items(&self) -> bool {
        !self.selected_items.is_empty()
    }

    pub fn has_selected_files(&self) -> bool {
        self.selected_files.values().any(|files| !files.is_empty())
    }

    pub fn get_total_selected_count(&self) -> usize {
        self.selected_items.len() + self.selected_files.values().map(|files| files.len()).sum::<usize>()
    }

    pub fn clear(&mut self) {
        self.selected_items.clear();
        self.selected_files.clear();
    }

    pub fn select_item(&mut self, item_id: i32) {
        self.selected_items.insert(item_id);
    }

    pub fn deselect_item(&mut self, item_id: i32) {
        self.selected_items.remove(&item_id);
    }

    pub fn toggle_item(&mut self, item_id: i32) {
        if self.selected_items.contains(&item_id) {
            self.selected_items.remove(&item_id);
        } else {
            self.selected_items.insert(item_id);
        }
    }

    pub fn is_item_selected(&self, item_id: i32) -> bool {
        self.selected_items.contains(&item_id)
    }

    pub fn select_file(&mut self, item_id: i32, file_id: i32) {
        self.selected_files.entry(item_id).or_insert_with(HashSet::new).insert(file_id);
    }

    pub fn deselect_file(&mut self, item_id: i32, file_id: i32) {
        if let Some(files) = self.selected_files.get_mut(&item_id) {
            files.remove(&file_id);
            if files.is_empty() {
                self.selected_files.remove(&item_id);
            }
        }
    }

    pub fn toggle_file(&mut self, item_id: i32, file_id: i32) {
        if let Some(files) = self.selected_files.get_mut(&item_id) {
            if files.contains(&file_id) {
                files.remove(&file_id);
                if files.is_empty() {
                    self.selected_files.remove(&item_id);
                }
            } else {
                files.insert(file_id);
            }
        } else {
            let mut files = HashSet::new();
            files.insert(file_id);
            self.selected_files.insert(item_id, files);
        }
    }

    pub fn is_file_selected(&self, item_id: i32, file_id: i32) -> bool {
        self.selected_files.get(&item_id).map_or(false, |files| files.contains(&file_id))
    }

    pub fn select_all_items(&mut self, items: &[DownloadItem]) {
        self.selected_items.clear();
        for item in items {
            self.selected_items.insert(item.id);
        }
    }

    pub fn deselect_all_items(&mut self) {
        self.selected_items.clear();
    }

    pub fn toggle_all_items(&mut self, items: &[DownloadItem]) {
        if self.selected_items.len() == items.len() {
            self.selected_items.clear();
        } else {
            self.select_all_items(items);
        }
    }

    pub fn save_to_storage(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(serialized) = serde_json::to_string(self) {
                        let _ = storage.set_item("torbox_selection_state", &serialized);
                    }
                }
            }
        }
    }

    pub fn load_from_storage() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(serialized)) = storage.get_item("torbox_selection_state") {
                        if let Ok(state) = serde_json::from_str::<SelectionState>(&serialized) {
                            return state;
                        }
                    }
                }
            }
        }
        Self::default()
    }
}

impl std::fmt::Display for DownloadType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DownloadType::Torrent => write!(f, "Torrent"),
            DownloadType::WebDownload => write!(f, "Web Download"),
            DownloadType::Usenet => write!(f, "Usenet"),
        }
    }
}

// Convert API types to DownloadItem
impl From<Torrent> for DownloadItem {
    fn from(torrent: Torrent) -> Self {
        // Cache lowercase name once (much faster than calling to_lowercase() 10+ times)
        let name_lower = torrent.name.to_lowercase();
        let is_season = name_lower.contains("season") || 
                       name_lower.contains("s0") ||
                       name_lower.contains("s1") ||
                       name_lower.contains("s2") ||
                       name_lower.contains("s3") ||
                       name_lower.contains("s4") ||
                       name_lower.contains("s5") ||
                       name_lower.contains("s6") ||
                       name_lower.contains("s7") ||
                       name_lower.contains("s8") ||
                       name_lower.contains("s9");

        let season_info = if is_season {
            if let Some(files) = &torrent.files {
                Some(SeasonInfo {
                    season_number: extract_season_number(&torrent.name),
                    episode_count: files.len() as i32,
                    episodes: Vec::new(), // Load lazily when row expands or when season info is displayed
                })
            } else {
                None
            }
        } else {
            None
        };

        // Detect stalled status for torrents
        let mut final_status = detect_stalled_status(
            &torrent.download_state,
            torrent.download_speed,
            torrent.upload_speed,
            Some(torrent.active),
            Some(torrent.seeds),
            Some(torrent.peers),
            &torrent.created_at,
            Some(&torrent.updated_at),
        );
        
        // Optimize: Cache lowercase status for multiple contains() checks
        let status_lower_final = final_status.to_lowercase();
        
        // If status is "uploading" but not active and download is finished, treat as cached/completed
        // Only active uploading items should be considered "Seeding"
        if (status_lower_final.contains("uploading") || status_lower_final.contains("seeding")) 
            && !torrent.active 
            && torrent.download_finished {
            // Change inactive finished uploading items to cached status
            final_status = if torrent.cached {
                "cached".to_string()
            } else {
                "completed".to_string()
            };
        }
        
        if status_lower_final.contains("stalled") && !torrent.active && !torrent.download_finished {
            final_status = "inactive".to_string();
        }
        
        if !torrent.active && !torrent.download_finished 
            && final_status != "expired" 
            && !status_lower_final.contains("cached") 
            && !status_lower_final.contains("completed") 
            && !status_lower_final.contains("uploading") 
            && !status_lower_final.contains("seeding")
            && !status_lower_final.contains("stalled") {  // Already handled above
            final_status = "inactive".to_string();
        }

        Self {
            id: torrent.id,
            name: torrent.name,
            size: torrent.size,
            created_at: torrent.created_at,
            status: final_status,
            download_type: DownloadType::Torrent,
            progress: torrent.progress,
            download_speed: torrent.download_speed,
            upload_speed: torrent.upload_speed,
            active: torrent.active,
            files: torrent.files.unwrap_or_default().into_iter().map(|f| DownloadFile {
                id: f.id,
                name: f.name,
                size: f.size,
                md5: f.md5,
                hash: f.hash,
                zipped: Some(f.zipped),
                s3_path: Some(f.s3_path),
                infected: Some(f.infected),
                mimetype: Some(f.mimetype),
                short_name: Some(f.short_name),
                absolute_path: Some(f.absolute_path),
                opensubtitles_hash: f.opensubtitles_hash,
            }).collect(),
            is_season,
            season_info,
            eta: Some(torrent.eta),
            total_downloaded: Some(torrent.total_downloaded),
            private: torrent.private,
        }
    }
}

impl From<WebDownload> for DownloadItem {
    fn from(web_dl: WebDownload) -> Self {
        let name_lower = web_dl.name.to_lowercase();
        let is_season = name_lower.contains("season") || 
                       name_lower.contains("s0") ||
                       name_lower.contains("s1") ||
                       name_lower.contains("s2") ||
                       name_lower.contains("s3") ||
                       name_lower.contains("s4") ||
                       name_lower.contains("s5") ||
                       name_lower.contains("s6") ||
                       name_lower.contains("s7") ||
                       name_lower.contains("s8") ||
                       name_lower.contains("s9");

        // Optimize: For seasons, only calculate count initially (defer episode list)
        let season_info = if is_season {
            Some(SeasonInfo {
                season_number: extract_season_number(&web_dl.name),
                episode_count: web_dl.files.len() as i32,
                episodes: Vec::new(), // Load lazily when needed
            })
        } else {
            None
        };

        // Detect stalled status for web downloads (check if "checking" for > 6 hours)
        let final_status = detect_stalled_status(
            &web_dl.status,
            0, // Web downloads don't have speed info
            0, // No upload
            None, // No active flag
            None, // No seeds
            None, // No peers
            &web_dl.created_at,
            None, // Web downloads don't have updated_at
        );

        Self {
            id: web_dl.id,
            name: web_dl.name,
            size: web_dl.size,
            created_at: web_dl.created_at,
            status: final_status,
            download_type: DownloadType::WebDownload,
            progress: web_dl.progress,
            download_speed: 0, // Web downloads don't have speed info
            upload_speed: 0,
            active: web_dl.status.to_lowercase() == "downloading" || web_dl.status.to_lowercase() == "active",
            files: web_dl.files.into_iter().map(|f| DownloadFile {
                id: f.id,
                name: f.name,
                size: f.size,
                md5: None,
                hash: None,
                zipped: None,
                s3_path: None,
                infected: None,
                mimetype: None,
                short_name: None,
                absolute_path: None,
                opensubtitles_hash: None,
            }).collect(),
            is_season,
            season_info,
            eta: None, // Web downloads don't have ETA
            total_downloaded: None, // Web downloads don't have total_downloaded, calculate from progress
            private: false, // Web downloads are not private trackers
        }
    }
}

impl From<UsenetDownload> for DownloadItem {
    fn from(usenet: UsenetDownload) -> Self {
        let name_lower = usenet.name.to_lowercase();
        let is_season = name_lower.contains("season") || 
                       name_lower.contains("s0") ||
                       name_lower.contains("s1") ||
                       name_lower.contains("s2") ||
                       name_lower.contains("s3") ||
                       name_lower.contains("s4") ||
                       name_lower.contains("s5") ||
                       name_lower.contains("s6") ||
                       name_lower.contains("s7") ||
                       name_lower.contains("s8") ||
                       name_lower.contains("s9");

        // Optimize: For seasons, only calculate count initially (defer episode list)
        let season_info = if is_season {
            Some(SeasonInfo {
                season_number: extract_season_number(&usenet.name),
                episode_count: usenet.files.len() as i32,
                episodes: Vec::new(), // Load lazily when needed
            })
        } else {
            None
        };

        // Detect stalled status for usenet downloads
        let mut final_status = detect_stalled_status(
            &usenet.download_state,
            usenet.download_speed,
            0, // Usenet has no upload
            Some(usenet.active),
            None, // No seeds for usenet
            None, // No peers for usenet
            &usenet.created_at,
            Some(&usenet.updated_at),
        );
        
        // Optimize: Cache lowercase status for multiple contains() checks
        let status_lower_final = final_status.to_lowercase();
        
        // If status is "uploading" but not active and download is finished, treat as cached/completed
        // Only active uploading items should be considered "Seeding"
        if (status_lower_final.contains("uploading") || status_lower_final.contains("seeding")) 
            && !usenet.active 
            && usenet.download_finished {
            // Change inactive finished uploading items to cached status
            final_status = if usenet.cached {
                "cached".to_string()
            } else {
                "completed".to_string()
            };
        }
        
        if !usenet.active && !usenet.download_finished 
            && final_status != "expired" 
            && !status_lower_final.contains("cached") 
            && !status_lower_final.contains("completed") 
            && !status_lower_final.contains("uploading") 
            && !status_lower_final.contains("seeding") {
            final_status = "inactive".to_string();
        }

        Self {
            id: usenet.id,
            name: usenet.name,
            size: usenet.size,
            created_at: usenet.created_at,
            status: final_status,
            download_type: DownloadType::Usenet,
            progress: usenet.progress,
            download_speed: usenet.download_speed,
            upload_speed: 0,
            active: usenet.active,
            files: usenet.files.into_iter().map(|f| DownloadFile {
                id: f.id,
                name: f.name,
                size: f.size,
                md5: f.md5,
                hash: f.hash,
                zipped: Some(f.zipped),
                s3_path: Some(f.s3_path),
                infected: Some(f.infected),
                mimetype: Some(f.mimetype),
                short_name: Some(f.short_name),
                absolute_path: Some(f.absolute_path),
                opensubtitles_hash: f.opensubtitles_hash,
            }).collect(),
            is_season,
            season_info,
            eta: Some(usenet.eta),
            total_downloaded: None, // Usenet doesn't have total_downloaded, calculate from progress
            private: false, // Usenet downloads are not private trackers
        }
    }
}

static SEASON_PATTERNS: Lazy<[Regex; 4]> = Lazy::new(|| [
    Regex::new(r"S(\d+)").unwrap(),
    Regex::new(r"Season\s*(\d+)").unwrap(),
    Regex::new(r"s(\d+)").unwrap(),
    Regex::new(r"season\s*(\d+)").unwrap(),
]);

fn extract_season_number(name: &str) -> Option<i32> {
    for re in SEASON_PATTERNS.iter() {
        if let Some(captures) = re.captures(name) {
            if let Some(season_str) = captures.get(1) {
                if let Ok(season_num) = season_str.as_str().parse::<i32>() {
                    return Some(season_num);
                }
            }
        }
    }
    
    None
}

fn format_size(bytes: i64) -> String {
    if bytes < 0 {
        return "Unknown".to_string();
    }
    if bytes >= 1_099_511_627_776 { // 1 TB
        format!("{:.2} TB", bytes as f64 / 1_099_511_627_776.0)
    } else if bytes >= 1_073_741_824 { // 1 GB
        format!("{:.2} GB", bytes as f64 / 1_073_741_824.0)
    } else if bytes >= 1_048_576 { // 1 MB
        format!("{:.2} MB", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1024 { // 1 KB
        format!("{:.2} KB", bytes as f64 / 1024.0)
    } else {
        format!("{} bytes", bytes)
    }
}

fn format_eta(eta_seconds: i32) -> String {
    if eta_seconds <= 0 {
        return "-".to_string();
    }
    
    let days = eta_seconds / 86400;
    let hours = (eta_seconds % 86400) / 3600;
    let minutes = (eta_seconds % 3600) / 60;
    let seconds = eta_seconds % 60;
    
    if days > 0 {
        format!("{}d {}h", days, hours)
    } else if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}

fn format_date(date_str: &str) -> String {
    if date_str.is_empty() {
        return "N/A".to_string();
    }
    
    if let Ok(parsed) = DateTime::parse_from_rfc3339(date_str) {
        parsed.format("%B %d, %Y at %I:%M %p").to_string()
    } else {
        date_str.to_string()
    }
}

fn is_media_file(file: &DownloadFile) -> bool {
    if let Some(mimetype) = &file.mimetype {
        mimetype.starts_with("video/") || mimetype.starts_with("audio/")
    } else {
        false
    }
}

fn is_video_file(file: &DownloadFile) -> bool {
    if let Some(mimetype) = &file.mimetype {
        mimetype.starts_with("video/")
    } else {
        false
    }
}

fn get_media_files(files: &[DownloadFile]) -> Vec<&DownloadFile> {
    files.iter()
        .filter(|f| {
            is_media_file(f) 
                && !f.zipped.unwrap_or(false)
                && !f.infected.unwrap_or(false)
        })
        .collect()
}

fn get_video_files(files: &[DownloadFile]) -> Vec<&DownloadFile> {
    files.iter()
        .filter(|f| {
            is_video_file(f)
                && !f.zipped.unwrap_or(false)
                && !f.infected.unwrap_or(false)
        })
        .collect()
}

fn get_largest_video_file(files: &[DownloadFile]) -> Option<&DownloadFile> {
    get_video_files(files)
        .into_iter()
        .max_by_key(|f| f.size)
}

fn get_first_video_file(files: &[DownloadFile]) -> Option<&DownloadFile> {
    get_video_files(files)
        .into_iter()
        .next()
}

fn detect_stalled_status(original_status: &str, download_speed: i64, upload_speed: i64, is_active: Option<bool>, seeds: Option<i32>, peers: Option<i32>, created_at: &str, updated_at: Option<&str>) -> String {
    let status_lower = original_status.to_lowercase();
    
    if status_lower.contains("uploading") {
        return original_status.replace("uploading", "seeding").replace("Uploading", "seeding");
    }
    
    if status_lower == "checking" {
        let check_time = updated_at.unwrap_or(created_at);
        if let Ok(parsed_time) = DateTime::parse_from_rfc3339(check_time) {
            let now = chrono::Utc::now();
            let parsed_utc = parsed_time.with_timezone(&chrono::Utc);
            let duration = now.signed_duration_since(parsed_utc);
            if duration.num_seconds() > 21600 {
                return format!("{} (stalled)", original_status);
            }
        }
    }
    
    let is_downloading_status = status_lower == "downloading" 
        || status_lower == "active" 
        || status_lower.contains("downloading");
    
    if is_downloading_status {
        let has_no_speed = download_speed < 1024;
        let has_no_seeds = seeds.map_or(false, |s| s == 0);
        let has_no_peers = peers.map_or(false, |p| p == 0);
        let is_inactive = is_active.map_or(false, |a| !a);
        
        if seeds.is_some() || peers.is_some() {
            if has_no_speed && upload_speed == 0 && ((has_no_seeds && has_no_peers) || is_inactive) {
                return format!("{} (stalled)", original_status);
            }
        } else if is_active.is_some() {
            if has_no_speed && is_inactive {
                return format!("{} (stalled)", original_status);
            }
        }
    }
    
    original_status.to_string()
}

fn normalize_status(status: &str) -> String {
    let status_lower = status.to_lowercase();
    
    if status_lower == "expired" {
        return "Expired".to_string();
    }
    
    if status_lower.contains("stalled") {
        return "Stalled".to_string();
    }
    
    if status_lower == "inactive" {
        return "Inactive".to_string();
    }
    
    if status_lower == "stopped seeding" {
        return "Paused".to_string();
    }
    if status_lower.contains("seeding") || status_lower.contains("uploading") || status_lower == "queuedup" {
        return "Seeding".to_string();
    }
    
    if status_lower.contains("queued") && !status_lower.contains("queuedup") {
        return "Queued".to_string();
    }
    
    if status_lower == "completed" || status_lower == "cached" {
        if status_lower == "cached" {
            return "Cached".to_string();
        }
        return "Completed".to_string();
    }
    
    if status_lower == "downloading" 
        || status_lower == "active" 
        || status_lower.contains("downloading")
        || status_lower == "metadl"
        || status_lower == "checkingresumedata"
        || status_lower == "checking"
        || status_lower.contains("checking")
        || status_lower == "allocating" {
        return "Downloading".to_string();
    }
    
    if status_lower == "paused" || status_lower == "stopped" || status_lower.contains("stopped") {
        return "Paused".to_string();
    }
    
    if status_lower.contains("error") 
        || status_lower == "failed" 
        || status_lower == "missingfiles"
        || status_lower.starts_with("failed") {
        return "Failed".to_string();
    }
    
    if status.is_empty() {
        return "Unknown".to_string();
    }
    let mut chars = status.chars();
    match chars.next() {
        None => "Unknown".to_string(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn get_status_priority(status: &str) -> u8 {
    let normalized = normalize_status(status);
    match normalized.as_str() {
        "Queued" => 1,
        "Downloading" => 2,
        "Seeding" => 3,
        "Completed" => 4,
        "Cached" => 5,
        "Paused" => 6,
        "Stalled" => 7,
        "Inactive" => 8,
        "Failed" => 9,
        "Expired" => 10,
        _ => 99,
    }
}

fn get_status_color_style(status: &str) -> String {
    let normalized = normalize_status(status);
    match normalized.as_str() {
        "Completed" | "Cached" => "color: #4ade80;".to_string(), // green-400
        "Downloading" | "Seeding" | "Queued" => "color: #60a5fa;".to_string(), // blue-400
        "Paused" => "color: #facc15;".to_string(), // yellow-400
        "Stalled" | "Failed" => "color: #f87171;".to_string(), // red-400
        "Inactive" | "Expired" => "color: #9ca3af;".to_string(), // gray-400
        _ => "color: #94a3b8;".to_string(), // slate-400
    }
}

fn get_status_badge_style(status: &str) -> String {
    let normalized = normalize_status(status);
    match normalized.as_str() {
        "Completed" => "background-color: rgba(34, 197, 94, 0.15); color: #4ade80; border: 1px solid rgba(34, 197, 94, 0.3);".to_string(),
        "Cached" => "background-color: rgba(34, 197, 94, 0.15); color: #4ade80; border: 1px solid rgba(34, 197, 94, 0.3);".to_string(),
        "Downloading" => "background-color: rgba(96, 165, 250, 0.15); color: #60a5fa; border: 1px solid rgba(96, 165, 250, 0.3);".to_string(),
        "Seeding" => "background-color: rgba(96, 165, 250, 0.15); color: #60a5fa; border: 1px solid rgba(96, 165, 250, 0.3);".to_string(),
        "Queued" => "background-color: rgba(96, 165, 250, 0.15); color: #60a5fa; border: 1px solid rgba(96, 165, 250, 0.3);".to_string(),
        "Paused" => "background-color: rgba(250, 204, 21, 0.15); color: #facc15; border: 1px solid rgba(250, 204, 21, 0.3);".to_string(),
        "Stalled" => "background-color: rgba(248, 113, 113, 0.15); color: #f87171; border: 1px solid rgba(248, 113, 113, 0.3);".to_string(),
        "Inactive" | "Expired" => "background-color: rgba(156, 163, 175, 0.15); color: #9ca3af; border: 1px solid rgba(156, 163, 175, 0.3);".to_string(),
        "Failed" => "background-color: rgba(248, 113, 113, 0.15); color: #f87171; border: 1px solid rgba(248, 113, 113, 0.3);".to_string(),
        _ => "background-color: rgba(148, 163, 184, 0.15); color: #94a3b8; border: 1px solid rgba(148, 163, 184, 0.3);".to_string(),
    }
}

fn get_progress_bg_color(_status: &str) -> &'static str {
    ""
}

fn get_progress_bar_style(_status: &str, _progress: f32) -> String {
    "background-color: var(--progress-fill);".to_string()
}

#[component]
pub fn DownloadsTable(
    downloads_signal: RwSignal<Vec<DownloadItem>>,
) -> impl IntoView {
    let context = use_context::<DashboardContext>()
        .expect("DashboardContext should be provided by MainDashboard");
    let user_data = context.user_data;
    let user_loading = context.user_loading;
    
    let downloads = downloads_signal;
    let loading = RwSignal::new(false);
    let error = RwSignal::new(None::<String>);
    let warnings = RwSignal::new(Vec::<String>::new());
    let expanded_rows = RwSignal::new(std::collections::HashSet::<i32>::new());
    let expanded_file_rows = RwSignal::new(std::collections::HashSet::<i32>::new());
    let is_blurred = RwSignal::new(false);
    
    let selection_state = RwSignal::new(SelectionState::default());
    #[cfg(target_arch = "wasm32")]
    {
        let selection_state_clone = selection_state.clone();
        spawn_local(async move {
            let loaded_state = SelectionState::load_from_storage();
            selection_state_clone.set(loaded_state);
        });
    }
    let show_bulk_actions = RwSignal::new(false);
    
    let open_dropdown = RwSignal::new(Option::<i32>::None);
    
    let status_filter = RwSignal::new("all".to_string());
    let type_filter = RwSignal::new("all".to_string());
    let sort_by = RwSignal::new("date".to_string());
    let sort_order = RwSignal::new("desc".to_string());
    
    let fetch_user_data_if_needed = {
        let user_data = user_data.clone();
        let user_loading = user_loading.clone();
        move || {
            #[cfg(target_arch = "wasm32")]
            {
                if user_data.get().is_none() && !user_loading.get() {
                    user_loading.set(true);
                    spawn_local(async move {
                        if let Some(window) = web_sys::window() {
                            if let Ok(Some(storage)) = window.local_storage() {
                                if let Ok(Some(api_key)) = storage.get_item("api_key") {
                                    if !api_key.is_empty() {
                                        let client = TorboxClient::new(api_key);
                                        match client.get_user(Some(false)).await {
                                            Ok(response) => {
                                                if let Some(user) = response.data {
                                                    user_data.set(Some(user));
                                                }
                                            }
                                            Err(_) => {
                                                // Keep None if API call fails
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        user_loading.set(false);
                    });
                }
            }
        }
    };
    
    fetch_user_data_if_needed();
    
    let has_streaming_plan = move || {
        user_data.get()
            .map(|u| u.plan == 2)
            .unwrap_or(false)
    };

    let fetch_downloads = move || {
        #[cfg(target_arch = "wasm32")]
        {
            let loading_clone = loading.clone();
            let error_clone = error.clone();
            let warnings_clone = warnings.clone();
            let downloads_clone = downloads.clone();
            
            spawn_local(async move {
                use wasm_bindgen_futures::JsFuture;
                use web_sys::js_sys::Promise;
                
                // Use setTimeout for yielding - allows browser to render between batches
                async fn yield_to_browser() {
                    let window = web_sys::window().unwrap();
                    let (tx, rx) = futures::channel::oneshot::channel();
                    let closure = wasm_bindgen::closure::Closure::once(move || {
                        let _ = tx.send(());
                    });
                    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                        closure.as_ref().unchecked_ref(),
                        0
                    );
                    closure.forget();
                    let _ = rx.await;
                }
                
                // Simple microtask yield for less critical cases
                async fn yield_microtask() {
                    let promise = Promise::resolve(&wasm_bindgen::JsValue::UNDEFINED);
                    let _ = JsFuture::from(promise).await;
                }
                
                yield_microtask().await;
                
                #[cfg(target_arch = "wasm32")]
                let load_start_time = js_sys::Date::now();
                #[cfg(target_arch = "wasm32")]
                web_sys::console::log_1(&"[DownloadsTable] Starting to fetch and load all mylist items".into());
                #[cfg(not(target_arch = "wasm32"))]
                let load_start_time = 0.0;
                
                loading_clone.set(true);
                error_clone.set(None);
                warnings_clone.set(Vec::new());
                
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(storage)) = window.local_storage() {
                        if let Ok(Some(api_key)) = storage.get_item("api_key") {
                            if !api_key.is_empty() {
                                let client = TorboxClient::new(api_key);
                                let mut all_downloads = Vec::new();
                                let mut api_errors = Vec::new();
                                
                                let torrents_future = client.get_torrent_list(None, Some(false), None, None);
                                let web_downloads_future = client.get_web_download_list(None, Some(false), None, None);
                                let usenet_future = client.get_usenet_download_list(None, Some(false), None, None);

                                let queued_torrents_future = client.get_queued_downloads(Some("torrent".to_string()), None, Some(false), None, None);
                                let queued_usenet_future = client.get_queued_downloads(Some("usenet".to_string()), None, Some(false), None, None);
                                let queued_webdl_future = client.get_queued_downloads(Some("webdl".to_string()), None, Some(false), None, None);
                                
                                let (torrents_result, web_result, usenet_result, queued_torrents_result, queued_usenet_result, queued_webdl_result) = futures::join!(
                                    torrents_future,
                                    web_downloads_future,
                                    usenet_future,
                                    queued_torrents_future,
                                    queued_usenet_future,
                                    queued_webdl_future
                                );
                                
                                const UPDATE_THRESHOLD: usize = 20;
                                const MICROTASK_YIELD: usize = 3;
                                let mut total_processed = 0;
                                
                                match torrents_result {
                                    Ok(response) => {
                                        if let Some(data) = response.data {
                                            for torrent in data {
                                                all_downloads.push(DownloadItem::from(torrent));
                                                total_processed += 1;
                                                
                                                if total_processed % UPDATE_THRESHOLD == 0 {
                                                    downloads_clone.set(all_downloads.clone());
                                                    yield_to_browser().await;
                                                } else if total_processed % MICROTASK_YIELD == 0 {
                                                    yield_microtask().await;
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        log!("Torrent API error: {:?}", e);
                                        api_errors.push(format!("Failed to fetch torrents: {}", e));
                                    }
                                }
                                
                                yield_to_browser().await;
                                
                                match web_result {
                                    Ok(response) => {
                                        if let Some(data) = response.data {
                                            for web_dl in data {
                                                all_downloads.push(DownloadItem::from(web_dl));
                                                total_processed += 1;
                                                
                                                if total_processed % UPDATE_THRESHOLD == 0 {
                                                    downloads_clone.set(all_downloads.clone());
                                                    yield_to_browser().await;
                                                } else if total_processed % MICROTASK_YIELD == 0 {
                                                    yield_microtask().await;
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        log!("Web download API error: {:?}", e);
                                        api_errors.push(format!("Failed to fetch web downloads: {}", e));
                                    }
                                }
                                
                                yield_to_browser().await;
                                
                                match usenet_result {
                                    Ok(response) => {
                                        if let Some(data) = response.data {
                                            for usenet in data {
                                                all_downloads.push(DownloadItem::from(usenet));
                                                total_processed += 1;
                                                
                                                if total_processed % UPDATE_THRESHOLD == 0 {
                                                    downloads_clone.set(all_downloads.clone());
                                                    yield_to_browser().await;
                                                } else if total_processed % MICROTASK_YIELD == 0 {
                                                    yield_microtask().await;
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        log!("Usenet API error: {:?}", e);
                                        api_errors.push(format!("Failed to fetch usenet downloads: {}", e));
                                    }
                                }
                                
                                yield_to_browser().await;
                                

                                match queued_torrents_result {
                                    Ok(response) => {
                                        if let Some(data) = response.data {
                                            // Handle both array response (when type is specified) and nested object (when no type)
                                            let torrents_array = if let Some(arr) = data.as_array() {
                                                Some(arr.clone())
                                            } else if let Ok(queued_data) = serde_json::from_value::<serde_json::Value>(data) {
                                                queued_data.get("torrents").and_then(|v| v.as_array()).cloned()
                                            } else {
                                                None
                                            };
                                            
                                            if let Some(torrents_array) = torrents_array {
                                                for item in torrents_array {
                                                    if let Ok(mut torrent) = serde_json::from_value::<Torrent>(item.clone()) {
                                                        torrent.download_state = "queued".to_string();
                                                        all_downloads.push(DownloadItem::from(torrent));
                                                        total_processed += 1;
                                                        if total_processed % UPDATE_THRESHOLD == 0 {
                                                            downloads_clone.set(all_downloads.clone());
                                                            yield_to_browser().await;
                                                        } else if total_processed % MICROTASK_YIELD == 0 {
                                                            yield_microtask().await;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        log!("Queued torrents API error: {:?}", e);
                                        api_errors.push(format!("Failed to fetch queued torrents: {}", e));
                                    }
                                }
                                
                                yield_to_browser().await;
                                
                                match queued_usenet_result {
                                    Ok(response) => {
                                        if let Some(data) = response.data {
                                            // Handle both array response (when type is specified) and nested object (when no type)
                                            let usenet_array = if let Some(arr) = data.as_array() {
                                                Some(arr.clone())
                                            } else if let Ok(queued_data) = serde_json::from_value::<serde_json::Value>(data) {
                                                queued_data.get("usenet").and_then(|v| v.as_array()).cloned()
                                            } else {
                                                None
                                            };
                                            
                                            if let Some(usenet_array) = usenet_array {
                                                for item in usenet_array {
                                                    if let Ok(mut usenet) = serde_json::from_value::<UsenetDownload>(item.clone()) {
                                                        usenet.download_state = "queued".to_string();
                                                        all_downloads.push(DownloadItem::from(usenet));
                                                        total_processed += 1;
                                                        if total_processed % UPDATE_THRESHOLD == 0 {
                                                            downloads_clone.set(all_downloads.clone());
                                                            yield_to_browser().await;
                                                        } else if total_processed % MICROTASK_YIELD == 0 {
                                                            yield_microtask().await;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        log!("Queued usenet API error: {:?}", e);
                                        api_errors.push(format!("Failed to fetch queued usenet downloads: {}", e));
                                    }
                                }
                                
                                yield_to_browser().await;
                                
                                // Process queued web downloads
                                match queued_webdl_result {
                                    Ok(response) => {
                                        if let Some(data) = response.data {
                                            // Handle both array response (when type is specified) and nested object (when no type)
                                            let webdl_array = if let Some(arr) = data.as_array() {
                                                Some(arr.clone())
                                            } else if let Ok(queued_data) = serde_json::from_value::<serde_json::Value>(data) {
                                                queued_data.get("webdl").and_then(|v| v.as_array()).cloned()
                                            } else {
                                                None
                                            };
                                            
                                            if let Some(webdl_array) = webdl_array {
                                                for item in webdl_array {
                                                    if let Ok(mut web_dl) = serde_json::from_value::<WebDownload>(item.clone()) {
                                                        web_dl.status = "queued".to_string();
                                                        all_downloads.push(DownloadItem::from(web_dl));
                                                        total_processed += 1;
                                                        if total_processed % UPDATE_THRESHOLD == 0 {
                                                            downloads_clone.set(all_downloads.clone());
                                                            yield_to_browser().await;
                                                        } else if total_processed % MICROTASK_YIELD == 0 {
                                                            yield_microtask().await;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        log!("Queued web downloads API error: {:?}", e);
                                        api_errors.push(format!("Failed to fetch queued web downloads: {}", e));
                                    }
                                }
                                
                                if !api_errors.is_empty() && all_downloads.is_empty() {
                                    error_clone.set(Some(api_errors.join("; ")));
                                } else if !api_errors.is_empty() {
                                    warnings_clone.set(api_errors);
                                    log!("Partial API failures: {}", warnings_clone.get().join("; "));
                                }
                                
                                #[cfg(target_arch = "wasm32")]
                                {
                                    let load_end_time = js_sys::Date::now();
                                    let load_duration_ms = load_end_time - load_start_time;
                                    let item_count = all_downloads.len();
                                    web_sys::console::log_1(
                                        &format!(
                                            "[DownloadsTable] Finished loading all mylist items: {} items in {:.2}s",
                                            item_count,
                                            load_duration_ms / 1000.0
                                        ).into()
                                    );
                                    
                                    let render_start_time = js_sys::Date::now();
                                    
                                    downloads_clone.set(all_downloads);
                                    loading_clone.set(false);
                                    
                                    yield_to_browser().await;
                                    yield_to_browser().await;
                                    yield_microtask().await;
                                    
                                    let render_end_time = js_sys::Date::now();
                                    let render_duration_ms = render_end_time - render_start_time;
                                    let total_duration_ms = render_end_time - load_start_time;
                                    
                                    web_sys::console::log_1(
                                        &format!(
                                            "[DownloadsTable] Table fully rendered: {} items rendered in {:.2}s (total: {:.2}s)",
                                            item_count,
                                            render_duration_ms / 1000.0,
                                            total_duration_ms / 1000.0
                                        ).into()
                                    );
                                }
                                #[cfg(not(target_arch = "wasm32"))]
                                {
                                    downloads_clone.set(all_downloads);
                                    loading_clone.set(false);
                                }
                            } else {
                                error_clone.set(Some("No API key found".to_string()));
                                loading_clone.set(false);
                            }
                        } else {
                            error_clone.set(Some("Failed to access localStorage".to_string()));
                            loading_clone.set(false);
                        }
                    } else {
                        error_clone.set(Some("Failed to access localStorage".to_string()));
                        loading_clone.set(false);
                    }
                } else {
                    error_clone.set(Some("Failed to access window".to_string()));
                    loading_clone.set(false);
                }
            });
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            loading.set(true);
            error.set(None);
            warnings.set(Vec::new());
            loading.set(false);
        }
    };

    let fetch_downloads_clone = fetch_downloads.clone();
    #[cfg(target_arch = "wasm32")]
    {
        spawn_local(async move {
            use wasm_bindgen_futures::JsFuture;
            use web_sys::js_sys::Promise;
            
            let promise = Promise::resolve(&wasm_bindgen::JsValue::UNDEFINED);
            let _ = JsFuture::from(promise).await;
            
            let promise2 = Promise::resolve(&wasm_bindgen::JsValue::UNDEFINED);
            let _ = JsFuture::from(promise2).await;
            
            fetch_downloads_clone();
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        fetch_downloads();
    }
    
    #[cfg(target_arch = "wasm32")]
    {
        let downloads_poll = downloads.clone();
        spawn_local(async move {
            use wasm_bindgen_futures::JsFuture;
            use web_sys::js_sys::Promise;
            
            let promise = Promise::new(&mut |resolve, _| {
                let window = web_sys::window().unwrap();
                let closure = wasm_bindgen::closure::Closure::once(move || {
                    resolve.call0(&wasm_bindgen::JsValue::UNDEFINED).ok();
                });
                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    2000,
                );
                closure.forget();
            });
            let _ = JsFuture::from(promise).await;
            
            loop {
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(storage)) = window.local_storage() {
                        if let Ok(Some(api_key)) = storage.get_item("api_key") {
                            if !api_key.is_empty() {
                                // Use the same API key for the entire polling cycle
                                let api_key_clone = api_key.clone();
                                let client = TorboxClient::new(api_key_clone);
                                let current_downloads = downloads_poll.get();
                                
                                let mut current_map: std::collections::HashMap<(i32, DownloadType), usize> = std::collections::HashMap::new();
                                for (idx, item) in current_downloads.iter().enumerate() {
                                    current_map.insert((item.id, item.download_type.clone()), idx);
                                }
                                
                                // Use bypass_cache: false for consistency with initial fetch and to avoid wrong cached data
                                let torrents_future = client.get_torrent_list(None, Some(false), None, None);
                                let web_downloads_future = client.get_web_download_list(None, Some(false), None, None);
                                let usenet_future = client.get_usenet_download_list(None, Some(false), None, None);

                                let queued_torrents_future = client.get_queued_downloads(Some("torrent".to_string()), None, Some(false), None, None);
                                let queued_usenet_future = client.get_queued_downloads(Some("usenet".to_string()), None, Some(false), None, None);
                                let queued_webdl_future = client.get_queued_downloads(Some("webdl".to_string()), None, Some(false), None, None);
                                
                                let (torrents_result, web_result, usenet_result, queued_torrents_result, queued_usenet_result, queued_webdl_result) = futures::join!(
                                    torrents_future,
                                    web_downloads_future,
                                    usenet_future,
                                    queued_torrents_future,
                                    queued_usenet_future,
                                    queued_webdl_future
                                );
                                
                                let mut updated_downloads = current_downloads.clone();
                                let mut seen_ids: std::collections::HashSet<(i32, DownloadType)> = std::collections::HashSet::new();
                                
                                if let Ok(response) = torrents_result {
                                    if let Some(data) = response.data {
                                        for torrent in data {
                                            let item = DownloadItem::from(torrent);
                                            let key = (item.id, item.download_type.clone());
                                            seen_ids.insert(key.clone());
                                            
                                            if let Some(&idx) = current_map.get(&key) {
                                                updated_downloads[idx] = item;
                                            } else {
                                                updated_downloads.push(item);
                                            }
                                        }
                                    }
                                }
                                
                                if let Ok(response) = web_result {
                                    if let Some(data) = response.data {
                                        for web_dl in data {
                                            let item = DownloadItem::from(web_dl);
                                            let key = (item.id, item.download_type.clone());
                                            seen_ids.insert(key.clone());
                                            
                                            if let Some(&idx) = current_map.get(&key) {
                                                updated_downloads[idx] = item;
                                            } else {
                                                updated_downloads.push(item);
                                            }
                                        }
                                    }
                                }
                                
                                if let Ok(response) = usenet_result {
                                    if let Some(data) = response.data {
                                        for usenet in data {
                                            let item = DownloadItem::from(usenet);
                                            let key = (item.id, item.download_type.clone());
                                            seen_ids.insert(key.clone());
                                            
                                            if let Some(&idx) = current_map.get(&key) {
                                                updated_downloads[idx] = item;
                                            } else {
                                                updated_downloads.push(item);
                                            }
                                        }
                                    }
                                }
                                
                                // Process queued torrents
                                if let Ok(response) = queued_torrents_result {
                                    if let Some(data) = response.data {
                                        // Handle both array response (when type is specified) and nested object (when no type)
                                        let torrents_array = if let Some(arr) = data.as_array() {
                                            Some(arr.clone())
                                        } else if let Ok(queued_data) = serde_json::from_value::<serde_json::Value>(data) {
                                            queued_data.get("torrents").and_then(|v| v.as_array()).cloned()
                                        } else {
                                            None
                                        };
                                        
                                        if let Some(torrents_array) = torrents_array {
                                            for item in torrents_array {
                                                if let Ok(mut torrent) = serde_json::from_value::<Torrent>(item.clone()) {
                                                    torrent.download_state = "queued".to_string();
                                                    let item = DownloadItem::from(torrent);
                                                    let key = (item.id, item.download_type.clone());
                                                    seen_ids.insert(key.clone());
                                                    
                                                    if let Some(&idx) = current_map.get(&key) {
                                                        updated_downloads[idx] = item;
                                                    } else {
                                                        updated_downloads.push(item);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                
                                // Process queued usenet
                                if let Ok(response) = queued_usenet_result {
                                    if let Some(data) = response.data {
                                        // Handle both array response (when type is specified) and nested object (when no type)
                                        let usenet_array = if let Some(arr) = data.as_array() {
                                            Some(arr.clone())
                                        } else if let Ok(queued_data) = serde_json::from_value::<serde_json::Value>(data) {
                                            queued_data.get("usenet").and_then(|v| v.as_array()).cloned()
                                        } else {
                                            None
                                        };
                                        
                                        if let Some(usenet_array) = usenet_array {
                                            for item in usenet_array {
                                                if let Ok(mut usenet) = serde_json::from_value::<UsenetDownload>(item.clone()) {
                                                    usenet.download_state = "queued".to_string();
                                                    let item = DownloadItem::from(usenet);
                                                    let key = (item.id, item.download_type.clone());
                                                    seen_ids.insert(key.clone());
                                                    
                                                    if let Some(&idx) = current_map.get(&key) {
                                                        updated_downloads[idx] = item;
                                                    } else {
                                                        updated_downloads.push(item);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                
                                // Process queued web downloads
                                if let Ok(response) = queued_webdl_result {
                                    if let Some(data) = response.data {
                                        // Handle both array response (when type is specified) and nested object (when no type)
                                        let webdl_array = if let Some(arr) = data.as_array() {
                                            Some(arr.clone())
                                        } else if let Ok(queued_data) = serde_json::from_value::<serde_json::Value>(data) {
                                            queued_data.get("webdl").and_then(|v| v.as_array()).cloned()
                                        } else {
                                            None
                                        };
                                        
                                        if let Some(webdl_array) = webdl_array {
                                            for item in webdl_array {
                                                if let Ok(mut web_dl) = serde_json::from_value::<WebDownload>(item.clone()) {
                                                    web_dl.status = "queued".to_string();
                                                    let item = DownloadItem::from(web_dl);
                                                    let key = (item.id, item.download_type.clone());
                                                    seen_ids.insert(key.clone());
                                                    
                                                    if let Some(&idx) = current_map.get(&key) {
                                                        updated_downloads[idx] = item;
                                                    } else {
                                                        updated_downloads.push(item);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                
                                updated_downloads.retain(|item| {
                                    seen_ids.contains(&(item.id, item.download_type.clone()))
                                });
                                
                                downloads_poll.set(updated_downloads);
                            }
                        }
                    }
                }
                
                let promise = Promise::new(&mut |resolve, _| {
                    let window = web_sys::window().unwrap();
                    let closure = wasm_bindgen::closure::Closure::once(move || {
                        resolve.call0(&wasm_bindgen::JsValue::UNDEFINED).ok();
                    });
                    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                        closure.as_ref().unchecked_ref(),
                        10000,
                    );
                    closure.forget();
                });
                let _ = JsFuture::from(promise).await;
            }
        });
    }

    let toggle_expansion = move |id: i32| {
        let mut expanded = expanded_rows.get();
        if expanded.contains(&id) {
            expanded.remove(&id);
        } else {
            expanded.insert(id);
        }
        expanded_rows.set(expanded);
    };

    let toggle_file_expansion = {
        let expanded_file_rows = expanded_file_rows.clone();
        move |id: i32| {
            let mut expanded = expanded_file_rows.get();
            if expanded.contains(&id) {
                expanded.remove(&id);
            } else {
                expanded.insert(id);
            }
            expanded_file_rows.set(expanded);
        }
    };

    let toggle_expand_all_files = {
        let expanded_file_rows = expanded_file_rows.clone();
        let downloads = downloads.clone();
        move || {
            let current_downloads = downloads.get();
            let mut expanded = expanded_file_rows.get();
            let downloads_with_files: Vec<i32> = current_downloads.iter()
                .filter(|d| !d.files.is_empty())
                .map(|d| d.id)
                .collect();
            
            let all_expanded = downloads_with_files.iter().all(|id| expanded.contains(id));
            
            if all_expanded {
                for id in downloads_with_files {
                    expanded.remove(&id);
                }
            } else {
                for id in downloads_with_files {
                    expanded.insert(id);
                }
            }
            expanded_file_rows.set(expanded);
        }
    };

    let filtered_downloads = create_memo(move |_| {
        let mut filtered = downloads.get();
        let status_filter_val = status_filter.get();
        let type_filter_val = type_filter.get();
        let sort_by_val = sort_by.get();
        let sort_order_val = sort_order.get();
        
        if status_filter_val != "all" {
            let filter_status = status_filter_val.to_lowercase();
            filtered.retain(|download| {
                let normalized = normalize_status(&download.status);
                let normalized_lower = normalized.to_lowercase();
                
                if normalized_lower == filter_status {
                    return true;
                }
                
                if (filter_status == "error" || filter_status == "failed") && normalized_lower == "failed" {
                    return true;
                }
                
                if filter_status == "cached" && (normalized_lower == "cached" || normalized_lower == "completed") {
                    return true;
                }
                
                false
            });
        }
        
        if type_filter_val != "all" {
            filtered.retain(|download| {
                match type_filter_val.as_str() {
                    "torrent" => download.download_type == DownloadType::Torrent,
                    "web" => download.download_type == DownloadType::WebDownload,
                    "usenet" => download.download_type == DownloadType::Usenet,
                    _ => true,
                }
            });
        }
        
        let is_desc = sort_order_val == "desc";
        filtered.sort_by(|a, b| {
            match sort_by_val.as_str() {
                "name" => {
                    let result = a.name.cmp(&b.name);
                    if is_desc { result.reverse() } else { result }
                }
                "size" => {
                    let result = a.size.cmp(&b.size);
                    if is_desc { result.reverse() } else { result }
                }
                "progress" => {
                    let result = a.progress.partial_cmp(&b.progress).unwrap_or(std::cmp::Ordering::Equal);
                    if is_desc { result.reverse() } else { result }
                }
                "status" => {
                    let a_priority = get_status_priority(&a.status);
                    let b_priority = get_status_priority(&b.status);
                    let result = a_priority.cmp(&b_priority);
                    if result == std::cmp::Ordering::Equal {
                        let a_normalized = normalize_status(&a.status);
                        let b_normalized = normalize_status(&b.status);
                        let name_result = a_normalized.cmp(&b_normalized);
                        if is_desc { name_result.reverse() } else { name_result }
                    } else {
                        if is_desc { result.reverse() } else { result }
                    }
                }
                "queued" => {
                    let a_is_queued = a.status.to_lowercase() == "queued";
                    let b_is_queued = b.status.to_lowercase() == "queued";
                    match (a_is_queued, b_is_queued) {
                        (true, false) => if is_desc { std::cmp::Ordering::Greater } else { std::cmp::Ordering::Less },
                        (false, true) => if is_desc { std::cmp::Ordering::Less } else { std::cmp::Ordering::Greater },
                        _ => {
                            let result = a.created_at.cmp(&b.created_at);
                            if is_desc { result.reverse() } else { result }
                        }
                    }
                }
                "type" => {
                    let result = a.download_type.to_string().cmp(&b.download_type.to_string());
                    if is_desc { result.reverse() } else { result }
                }
                _ => {
                    let result = a.created_at.cmp(&b.created_at);
                    if is_desc { result.reverse() } else { result }
                }
            }
        });
        
        filtered
    });

    let toggle_item_selection = move |item_id: i32| {
        let mut state = selection_state.get();
        state.toggle_item(item_id);
        state.save_to_storage();
        selection_state.set(state);
        
        let new_state = selection_state.get();
        show_bulk_actions.set(new_state.has_selected_items() || new_state.has_selected_files());
    };

    let toggle_select_all = move || {
        let filtered = filtered_downloads.get();
        let mut state = selection_state.get();
        state.toggle_all_items(&filtered);
        state.save_to_storage();
        selection_state.set(state);
        
        let new_state = selection_state.get();
        show_bulk_actions.set(new_state.has_selected_items() || new_state.has_selected_files());
    };

    let clear_selection = move || {
        let mut state = selection_state.get();
        state.clear();
        state.save_to_storage();
        selection_state.set(state);
        show_bulk_actions.set(false);
    };

    let toggle_dropdown = move |download_id: i32| {
        let current = open_dropdown.get();
        if current == Some(download_id) {
            open_dropdown.set(None);
        } else {
            open_dropdown.set(Some(download_id));
        }
    };

    let close_dropdown = move || {
        open_dropdown.set(None);
    };

    let is_all_selected = move || {
        let filtered = filtered_downloads.get();
        let state = selection_state.get();
        !filtered.is_empty() && state.selected_items.len() == filtered.len()
    };

    let is_partially_selected = move || {
        let filtered = filtered_downloads.get();
        let state = selection_state.get();
        !filtered.is_empty() && !state.selected_items.is_empty() && state.selected_items.len() < filtered.len()
    };

    let handle_download = {
        let downloads_clone = downloads.clone();
        move |id: i32, download_type: DownloadType, file_id: Option<i32>| {
            #[cfg(target_arch = "wasm32")]
            {
                let downloads_ref = downloads_clone.clone();
                spawn_local(async move {
                    if let Some(window) = web_sys::window() {
                        if let Ok(Some(storage)) = window.local_storage() {
                            if let Ok(Some(api_key)) = storage.get_item("api_key") {
                                if !api_key.is_empty() {
                                    let origin = match window.location().origin() {
                                        Ok(orig) => orig,
                                        Err(_) => {
                                            log!("Failed to get window origin");
                                            return;
                                        }
                                    };
                                    
                                    let base_path = match download_type {
                                        DownloadType::Torrent => "/api/torrents/download",
                                        DownloadType::WebDownload => "/api/webdl/download",
                                        DownloadType::Usenet => "/api/usenet/download",
                                    };
                                    
                                    let file_ids_to_download: Vec<i32> = if file_id.is_none() {
                                        let downloads_list = downloads_ref.get();
                                        if let Some(download) = downloads_list.iter().find(|d| d.id == id) {
                                            let media_files = get_media_files(&download.files);
                                            if !media_files.is_empty() {
                                                media_files.iter().map(|f| f.id).collect()
                                            } else {
                                                Vec::new()
                                            }
                                        } else {
                                            Vec::new()
                                        }
                                    } else {
                                        vec![file_id.unwrap()]
                                    };
                                    
                                    let client = reqwest::Client::new();
                                    
                                    if !file_ids_to_download.is_empty() {
                                        for fid in file_ids_to_download {
                                            let mut url = format!("{}{}?", origin, base_path);
                                            match download_type {
                                                DownloadType::Torrent => {
                                                    url.push_str(&format!("torrent_id={}", id));
                                                }
                                                DownloadType::WebDownload => {
                                                    url.push_str(&format!("web_id={}", id));
                                                }
                                                DownloadType::Usenet => {
                                                    url.push_str(&format!("usenet_id={}", id));
                                                }
                                            }
                                            url.push_str(&format!("&file_id={}", fid));
                                            
                                            match client
                                                .get(&url)
                                                .header("Authorization", format!("Bearer {}", api_key.clone()))
                                                .send()
                                                .await
                                            {
                                                Ok(response) => {
                                                    if response.status().is_success() {
                                                        match response.json::<serde_json::Value>().await {
                                                            Ok(json) => {
                                                                if let Some(download_url) = json.get("data").and_then(|d| d.as_str()) {
                                                                    if let Some(document) = window.document() {
                                                                        if let Ok(anchor) = document.create_element("a") {
                                                                            if let Ok(anchor) = anchor.dyn_into::<web_sys::HtmlAnchorElement>() {
                                                                                anchor.set_href(download_url);
                                                                                anchor.set_attribute("download", "").ok();
                                                                                anchor.set_attribute("style", "display: none;").ok();
                                                                                if let Some(body) = document.body() {
                                                                                    body.append_child(&anchor).ok();
                                                                                    anchor.click();
                                                                                    body.remove_child(&anchor).ok();
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    log!("Download response missing data field for file_id {}", fid);
                                                                }
                                                            }
                                                            Err(e) => {
                                                                log!("Failed to parse download response for file_id {}: {:?}", fid, e);
                                                            }
                                                        }
                                                    } else {
                                                        log!("Download request failed for file_id {} with status: {}", fid, response.status());
                                                    }
                                                }
                                                Err(e) => {
                                                    log!("Failed to request download for file_id {}: {:?}", fid, e);
                                                }
                                            }
                                            
                                            let (tx, rx) = futures::channel::oneshot::channel();
                                            if let Some(window_for_delay) = web_sys::window() {
                                                let closure = wasm_bindgen::closure::Closure::once(move || {
                                                    let _ = tx.send(());
                                                });
                                                let _ = window_for_delay.set_timeout_with_callback_and_timeout_and_arguments_0(
                                                    closure.as_ref().unchecked_ref(),
                                                    100,
                                                );
                                                closure.forget();
                                            }
                                            let _ = rx.await;
                                        }
                                    } else {
                                        let mut url = format!("{}{}?", origin, base_path);
                                        match download_type {
                                            DownloadType::Torrent => {
                                                url.push_str(&format!("torrent_id={}", id));
                                            }
                                            DownloadType::WebDownload => {
                                                url.push_str(&format!("web_id={}", id));
                                            }
                                            DownloadType::Usenet => {
                                                url.push_str(&format!("usenet_id={}", id));
                                            }
                                        }
                                        
                                        match client
                                            .get(&url)
                                            .header("Authorization", format!("Bearer {}", api_key))
                                            .send()
                                            .await
                                        {
                                            Ok(response) => {
                                                if response.status().is_success() {
                                                    match response.json::<serde_json::Value>().await {
                                                        Ok(json) => {
                                                            if let Some(download_url) = json.get("data").and_then(|d| d.as_str()) {
                                                                // Create a temporary anchor element to trigger download
                                                                if let Some(document) = window.document() {
                                                                    if let Ok(anchor) = document.create_element("a") {
                                                                        if let Ok(anchor) = anchor.dyn_into::<web_sys::HtmlAnchorElement>() {
                                                                            anchor.set_href(download_url);
                                                                            anchor.set_attribute("download", "").ok();
                                                                            anchor.set_attribute("style", "display: none;").ok();
                                                                            if let Some(body) = document.body() {
                                                                                body.append_child(&anchor).ok();
                                                                                anchor.click();
                                                                                body.remove_child(&anchor).ok();
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                log!("Download response missing data field");
                                                            }
                                                        }
                                                        Err(e) => {
                                                            log!("Failed to parse download response: {:?}", e);
                                                        }
                                                    }
                                                } else {
                                                    log!("Download request failed with status: {}", response.status());
                                                }
                                            }
                                            Err(e) => {
                                                log!("Failed to request download: {:?}", e);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                });
            }
        }
    };

    let handle_delete = move |id: i32, download_type: DownloadType| {
        #[cfg(target_arch = "wasm32")]
        {
            spawn_local(async move {
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(storage)) = window.local_storage() {
                        if let Ok(Some(api_key)) = storage.get_item("api_key") {
                            if !api_key.is_empty() {
                                let client = TorboxClient::new(api_key);
                                
                                match download_type {
                                    DownloadType::Torrent => {
                                        match client.control_torrent("delete".to_string(), id, false).await {
                                            Ok(_) => {
                                                log!("Torrent deleted successfully: {}", id);
                                                fetch_downloads();
                                            }
                                            Err(e) => {
                                                log!("Failed to delete torrent: {:?}", e);
                                            }
                                        }
                                    }
                                    DownloadType::WebDownload => {
                                        match client.control_web_download("delete".to_string(), id, false).await {
                                            Ok(_) => {
                                                log!("Web download deleted successfully: {}", id);
                                                fetch_downloads();
                                            }
                                            Err(e) => {
                                                log!("Failed to delete web download: {:?}", e);
                                            }
                                        }
                                    }
                                    DownloadType::Usenet => {
                                        match client.control_usenet_download("delete".to_string(), id, false).await {
                                            Ok(_) => {
                                                log!("Usenet download deleted successfully: {}", id);
                                                fetch_downloads();
                                            }
                                            Err(e) => {
                                                log!("Failed to delete usenet download: {:?}", e);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            });
        }
    };

    let handle_bulk_download = move || {
        let state = selection_state.get();
        let selected_items = state.selected_items;
        let downloads_clone = downloads.clone();
        
        #[cfg(target_arch = "wasm32")]
        {
            spawn_local(async move {
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(storage)) = window.local_storage() {
                            if let Ok(Some(api_key)) = storage.get_item("api_key") {
                                if !api_key.is_empty() {
                                    let downloads_list = downloads_clone.get();
                                    
                                    const CONCURRENT_DOWNLOADS: usize = 3;
                                    const DELAY_MS: u64 = 200;
                                    
                                    let items: Vec<_> = selected_items.into_iter()
                                    .filter_map(|item_id| {
                                        downloads_list.iter()
                                            .find(|d| d.id == item_id)
                                            .map(|d| (item_id, d.download_type.clone()))
                                    })
                                    .collect();
                                
                                for chunk in items.chunks(CONCURRENT_DOWNLOADS) {
                                    let futures: Vec<_> = chunk.iter().map(|(item_id, download_type)| {
                                        let api_key = api_key.clone();
                                        let window_clone = web_sys::window();
                                        
                                        async move {
                                            let origin = match window_clone.as_ref()
                                                .and_then(|w| w.location().origin().ok()) {
                                                Some(orig) => orig,
                                                None => {
                                                    log!("Failed to get window origin for bulk download");
                                                    return;
                                                }
                                            };
                                            
                                            let base_path = match download_type {
                                                DownloadType::Torrent => "/api/torrents/download",
                                                DownloadType::WebDownload => "/api/webdl/download",
                                                DownloadType::Usenet => "/api/usenet/download",
                                            };
                                            
                                            let mut url = format!("{}{}?", origin, base_path);
                                            match download_type {
                                                DownloadType::Torrent => {
                                                    url.push_str(&format!("torrent_id={}", item_id));
                                                }
                                                DownloadType::WebDownload => {
                                                    url.push_str(&format!("web_id={}", item_id));
                                                }
                                                DownloadType::Usenet => {
                                                    url.push_str(&format!("usenet_id={}", item_id));
                                                }
                                            }
                                            
                                            let client = reqwest::Client::new();
                                            match client
                                                .get(&url)
                                                .header("Authorization", format!("Bearer {}", api_key))
                                                .send()
                                                .await
                                            {
                                                Ok(response) => {
                                                    if response.status().is_success() {
                                                        match response.json::<serde_json::Value>().await {
                                                            Ok(json) => {
                                                                if let Some(download_url) = json.get("data").and_then(|d| d.as_str()) {
                                                                    // Create a temporary anchor element to trigger download
                                                                    if let Some(window) = window_clone {
                                                                        if let Some(document) = window.document() {
                                                                            if let Ok(anchor) = document.create_element("a") {
                                                                                if let Ok(anchor) = anchor.dyn_into::<web_sys::HtmlAnchorElement>() {
                                                                                    anchor.set_href(download_url);
                                                                                    anchor.set_attribute("download", "").ok();
                                                                                    anchor.set_attribute("style", "display: none;").ok();
                                                                                    if let Some(body) = document.body() {
                                                                                        body.append_child(&anchor).ok();
                                                                                        anchor.click();
                                                                                        body.remove_child(&anchor).ok();
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    log!("Download response missing data field for ID: {}", item_id);
                                                                }
                                                            }
                                                            Err(e) => {
                                                                log!("Failed to parse download response for ID {}: {:?}", item_id, e);
                                                            }
                                                        }
                                                    } else {
                                                        log!("Download request failed for ID {} with status: {}", item_id, response.status());
                                                    }
                                                }
                                                Err(e) => {
                                                    log!("Failed to request download for ID {}: {:?}", item_id, e);
                                                }
                                            }
                                        }
                                    }).collect();
                                    
                                    futures::future::join_all(futures).await;
                                    
                                    if chunk.len() == CONCURRENT_DOWNLOADS {
                                        let delay_ms = DELAY_MS;
                                        let (tx, rx) = futures::channel::oneshot::channel();
                                        if let Some(window) = web_sys::window() {
                                            let closure = wasm_bindgen::closure::Closure::once(move || {
                                                let _ = tx.send(());
                                            });
                                            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                                                closure.as_ref().unchecked_ref(),
                                                delay_ms as i32,
                                            );
                                            closure.forget();
                                            let _ = rx.await;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            });
        }
    };

    let handle_bulk_delete = {
        let fetch_downloads_clone = fetch_downloads.clone();
        let clear_selection_clone = clear_selection.clone();
        move || {
            let state = selection_state.get();
            let selected_items = state.selected_items.clone();
            let downloads_clone = downloads.clone();
        
        #[cfg(target_arch = "wasm32")]
        {
            spawn_local(async move {
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(storage)) = window.local_storage() {
                        if let Ok(Some(api_key)) = storage.get_item("api_key") {
                            if !api_key.is_empty() {
                                let client = TorboxClient::new(api_key.clone());
                                let downloads_list = downloads_clone.get();
                                
                                const CONCURRENT_DELETES: usize = 3;
                                const DELAY_MS: u64 = 200;
                                
                                let items: Vec<_> = selected_items.into_iter()
                                    .filter_map(|item_id| {
                                        downloads_list.iter()
                                            .find(|d| d.id == item_id)
                                            .map(|d| (item_id, d.download_type.clone()))
                                    })
                                    .collect();
                                
                                let mut success_count = 0;
                                let mut error_count = 0;
                                
                                for chunk in items.chunks(CONCURRENT_DELETES) {
                                    let futures: Vec<_> = chunk.iter().map(|(item_id, download_type)| {
                                        let api_key = api_key.clone();
                                        let client = client.clone();
                                        
                                        async move {
                                            let result = match download_type {
                                                DownloadType::Torrent => {
                                                    client.control_torrent("delete".to_string(), *item_id, false).await
                                                }
                                                DownloadType::WebDownload => {
                                                    client.control_web_download("delete".to_string(), *item_id, false).await
                                                }
                                                DownloadType::Usenet => {
                                                    client.control_usenet_download("delete".to_string(), *item_id, false).await
                                                }
                                            };
                                            
                                            match result {
                                                Ok(_) => {
                                                    log!("{} deleted successfully: {}", 
                                                        match download_type {
                                                            DownloadType::Torrent => "Torrent",
                                                            DownloadType::WebDownload => "Web download",
                                                            DownloadType::Usenet => "Usenet",
                                                        },
                                                        item_id
                                                    );
                                                    Ok(*item_id)
                                                }
                                                Err(e) => {
                                                    log!("Failed to delete {} {}: {:?}", 
                                                        match download_type {
                                                            DownloadType::Torrent => "torrent",
                                                            DownloadType::WebDownload => "web download",
                                                            DownloadType::Usenet => "usenet",
                                                        },
                                                        item_id,
                                                        e
                                                    );
                                                    Err(*item_id)
                                                }
                                            }
                                        }
                                    }).collect();
                                    
                                    // Wait for all concurrent deletes in this chunk
                                    let results = futures::future::join_all(futures).await;
                                    for result in results {
                                        match result {
                                            Ok(_) => success_count += 1,
                                            Err(_) => error_count += 1,
                                        }
                                    }
                                    
                                    // Delay before next chunk (rate limiting)
                                    if chunk.len() == CONCURRENT_DELETES {
                                        // Simple delay using setTimeout via wasm-bindgen
                                        let delay_ms = DELAY_MS;
                                        let (tx, rx) = futures::channel::oneshot::channel();
                                        if let Some(window) = web_sys::window() {
                                            let closure = wasm_bindgen::closure::Closure::once(move || {
                                                let _ = tx.send(());
                                            });
                                            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                                                closure.as_ref().unchecked_ref(),
                                                delay_ms as i32,
                                            );
                                            closure.forget();
                                            let _ = rx.await;
                                        }
                                    }
                                }
                                
                                log!("Bulk delete completed: {} succeeded, {} failed", success_count, error_count);
                                
                                fetch_downloads_clone();
                                clear_selection_clone();
                            }
                        }
                    }
                }
            });
        }
        }
    };

    // Function to handle pause/resume action
    let handle_pause_resume = move |id: i32, download_type: DownloadType, current_status: String| {
        #[cfg(target_arch = "wasm32")]
        {
            spawn_local(async move {
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(storage)) = window.local_storage() {
                        if let Ok(Some(api_key)) = storage.get_item("api_key") {
                            if !api_key.is_empty() {
                                let client = TorboxClient::new(api_key);
                                
                                // Determine operation based on current status
                                let operation = match current_status.to_lowercase().as_str() {
                                    "downloading" | "active" | "seeding" => "stop_seeding",
                                    "paused" | "stopped" | "stalled" => "resume",
                                    _ => "stop_seeding", // Default to stop_seeding for completed/cached items
                                };
                                
                                match download_type {
                                    DownloadType::Torrent => {
                                        match client.control_torrent(operation.to_string(), id, false).await {
                                            Ok(_) => {
                                                log!("Torrent {} successfully: {}", operation, id);
                                                fetch_downloads();
                                            }
                                            Err(e) => {
                                                log!("Failed to {} torrent: {:?}", operation, e);
                                            }
                                        }
                                    }
                                    DownloadType::WebDownload => {
                                        match client.control_web_download(operation.to_string(), id, false).await {
                                            Ok(_) => {
                                                log!("Web download {} successfully: {}", operation, id);
                                                fetch_downloads();
                                            }
                                            Err(e) => {
                                                log!("Failed to {} web download: {:?}", operation, e);
                                            }
                                        }
                                    }
                                    DownloadType::Usenet => {
                                        match client.control_usenet_download(operation.to_string(), id, false).await {
                                            Ok(_) => {
                                                log!("Usenet download {} successfully: {}", operation, id);
                                                fetch_downloads();
                                            }
                                            Err(e) => {
                                                log!("Failed to {} usenet download: {:?}", operation, e);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            });
        }
    };

    // Function to handle streaming action
    let handle_stream = {
        let downloads_clone = downloads.clone();
        move |id: i32, download_type: DownloadType, file_id: Option<i32>| {
            #[cfg(target_arch = "wasm32")]
            {
                let downloads_ref = downloads_clone.clone();
                spawn_local(async move {
                    if let Some(window) = web_sys::window() {
                        if let Ok(Some(storage)) = window.local_storage() {
                            if let Ok(Some(api_key)) = storage.get_item("api_key") {
                                if !api_key.is_empty() {
                                    let client = TorboxClient::new(api_key);
                                    
                                    let stream_type = match download_type {
                                        DownloadType::Torrent => "torrent",
                                        DownloadType::WebDownload => "webdownload",
                                        DownloadType::Usenet => "usenet",
                                    };
                                    
                                    // Determine which file_id to use for streaming
                                    let final_file_id = if let Some(fid) = file_id {
                                        // Specific file requested (from sub-table)
                                        Some(fid)
                                    } else {
                                        // Main stream button - find largest video file
                                        let downloads_list = downloads_ref.get();
                                        if let Some(download) = downloads_list.iter().find(|d| d.id == id) {
                                            get_largest_video_file(&download.files).map(|f| f.id)
                                        } else {
                                            None
                                        }
                                    };
                                    
                                    if final_file_id.is_none() {
                                        log!("No video file found for streaming download ID: {}", id);
                                        return;
                                    }
                                    
                                    let request = crate::api::types::CreateStreamRequest {
                                        id,
                                        file_id: final_file_id,
                                        r#type: Some(stream_type.to_string()),
                                        chosen_subtitle_index: None,
                                        chosen_audio_index: Some(0),
                                    };
                                
                                match client.create_stream(request).await {
                                    Ok(response) => {
                                        if let Some(stream_data) = response.data {
                                            log!("Stream created for ID: {}", id);
                                            if let Some(window) = web_sys::window() {
                                                let encoded_url = js_sys::encode_uri_component(&stream_data.stream_url);
                                                let mut player_url = format!("/stream?url={}", encoded_url.as_string().unwrap_or_default());
                                                
                                                // Add presigned token and user token for subtitle/audio switching
                                                let encoded_token = js_sys::encode_uri_component(&stream_data.presigned_token);
                                                player_url.push_str(&format!("&presigned_token={}", encoded_token.as_string().unwrap_or_default()));
                                                if let Some(user_token) = &stream_data.user_token {
                                                    let encoded_user_token = js_sys::encode_uri_component(user_token);
                                                    player_url.push_str(&format!("&user_token={}", encoded_user_token.as_string().unwrap_or_default()));
                                                }
                                                // Add metadata if available
                                                if let Some(metadata) = &stream_data.metadata {
                                                    if let Ok(metadata_json) = serde_json::to_string(metadata) {
                                                        let encoded_metadata = js_sys::encode_uri_component(&metadata_json);
                                                        player_url.push_str(&format!("&metadata={}", encoded_metadata.as_string().unwrap_or_default()));
                                                    }
                                                }
                                                // Add subtitle URLs if available (from stream response, not metadata)
                                                if let Some(subtitles) = &stream_data.subtitles {
                                                    if !subtitles.is_empty() {
                                                        if let Ok(subtitles_json) = serde_json::to_string(subtitles) {
                                                            let encoded_subtitles = js_sys::encode_uri_component(&subtitles_json);
                                                            player_url.push_str(&format!("&subtitle_urls={}", encoded_subtitles.as_string().unwrap_or_default()));
                                                        }
                                                    }
                                                }
                                                
                                                if let Ok(_) = window.open_with_url_and_target(&player_url, "_blank") {
                                                    log!("Stream opened for ID: {}", id);
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        log!("Failed to create stream: {:?}", e);
                                    }
                                }
                            }
                        }
                    }
                }
            });
        }
        }
    };

    // Function to handle reannounce action (for stalled torrents)
    let handle_reannounce = move |id: i32, download_type: DownloadType| {
        #[cfg(target_arch = "wasm32")]
        {
            spawn_local(async move {
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(storage)) = window.local_storage() {
                        if let Ok(Some(api_key)) = storage.get_item("api_key") {
                            if !api_key.is_empty() {
                                let client = TorboxClient::new(api_key);
                                
                                match download_type {
                                    DownloadType::Torrent => {
                                        match client.control_torrent("reannounce".to_string(), id, false).await {
                                            Ok(_) => {
                                                log!("Torrent reannounced successfully: {}", id);
                                                fetch_downloads();
                                            }
                                            Err(e) => {
                                                log!("Failed to reannounce torrent: {:?}", e);
                                            }
                                        }
                                    }
                                    _ => {
                                        log!("Reannounce only available for torrents");
                                    }
                                }
                            }
                        }
                    }
                }
            });
        }
    };

    // Function to handle cloud upload action
    let handle_cloud_upload = move |id: i32, download_type: DownloadType, provider: String| {
        #[cfg(target_arch = "wasm32")]
        {
            spawn_local(async move {
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(storage)) = window.local_storage() {
                        if let Ok(Some(api_key)) = storage.get_item("api_key") {
                            if !api_key.is_empty() {
                                let client = TorboxClient::new(api_key);
                                
                                let request = crate::api::types::CloudUpload {
                                    id,
                                    file_id: 0,
                                    zip: false,
                                    r#type: match download_type {
                                        DownloadType::Torrent => "torrent".to_string(),
                                        DownloadType::WebDownload => "webdl".to_string(),
                                        DownloadType::Usenet => "usenet".to_string(),
                                    },
                                    token: String::new(), // Empty token for now - would need to be provided by user
                                };
                                
                                match provider.as_str() {
                                    "google" => {
                                        match client.upload_to_google_drive(request).await {
                                            Ok(_) => {
                                                log!("Google Drive upload started for ID: {}", id);
                                                fetch_downloads();
                                            }
                                            Err(e) => {
                                                log!("Failed to upload to Google Drive: {:?}", e);
                                            }
                                        }
                                    }
                                    "dropbox" => {
                                        match client.upload_to_dropbox(request).await {
                                            Ok(_) => {
                                                log!("Dropbox upload started for ID: {}", id);
                                                fetch_downloads();
                                            }
                                            Err(e) => {
                                                log!("Failed to upload to Dropbox: {:?}", e);
                                            }
                                        }
                                    }
                                    "onedrive" => {
                                        match client.upload_to_onedrive(request).await {
                                            Ok(_) => {
                                                log!("OneDrive upload started for ID: {}", id);
                                                fetch_downloads();
                                            }
                                            Err(e) => {
                                                log!("Failed to upload to OneDrive: {:?}", e);
                                            }
                                        }
                                    }
                                    _ => {
                                        log!("Unknown cloud provider: {}", provider);
                                    }
                                }
                            }
                        }
                    }
                }
            });
        }
    };

    // Helper functions to determine if actions should be enabled
    let is_download_enabled = move |status: &str| -> bool {
        let status_lower = status.to_lowercase();
        status_lower == "completed" || status_lower == "cached" || status_lower.contains("seeding")
    };

    let is_pause_resume_enabled = move |status: &str| -> bool {
        let status_lower = status.to_lowercase();
        status_lower == "downloading" || status_lower == "active" || status_lower.contains("seeding") 
            || status_lower == "paused" || status_lower == "stopped" || status_lower.contains("stalled")
    };

    let is_stream_enabled = move |status: &str| -> bool {
        // Check if user has plan 2 first
        if !has_streaming_plan() {
            return false;
        }
        // Then check if download status allows streaming
        // Seeding (including variants like "seeding (no peers)") means the file is completed and ready
        let status_lower = status.to_lowercase();
        status_lower == "completed" || status_lower == "cached" || status_lower.contains("seeding")
    };

    let is_reannounce_enabled = move |status: &str, download_type: DownloadType| -> bool {
        download_type == DownloadType::Torrent && 
        match status.to_lowercase().as_str() {
            "stalled" | "stalled (no seeds)" | "checking" => true,
            _ => false,
        }
    };

    let is_cloud_upload_enabled = move |status: &str| -> bool {
        let status_lower = status.to_lowercase();
        status_lower == "completed" || status_lower == "cached" || status_lower.contains("seeding")
    };

    let is_delete_enabled = move |_status: &str| -> bool {
        true // Delete is always available
    };

    // Function to get download counts by type and status
    let get_download_counts = move || {
        let all_downloads = downloads.get();
        let mut counts = std::collections::HashMap::new();
        
        // Initialize counts
        counts.insert("total", all_downloads.len());
        counts.insert("torrents", 0);
        counts.insert("usenet", 0);
        counts.insert("webdl", 0);
        counts.insert("downloading", 0);
        counts.insert("seeding", 0);
        counts.insert("paused", 0);
        counts.insert("error", 0);
        counts.insert("cached", 0);
        counts.insert("queued", 0);
        counts.insert("stalled", 0);
        counts.insert("inactive", 0);
        counts.insert("expired", 0);
        
        for download in &all_downloads {
            // Count by type
            match download.download_type {
                DownloadType::Torrent => {
                    *counts.get_mut("torrents").unwrap() += 1;
                }
                DownloadType::Usenet => {
                    *counts.get_mut("usenet").unwrap() += 1;
                }
                DownloadType::WebDownload => {
                    *counts.get_mut("webdl").unwrap() += 1;
                }
            }
            
            // Count by normalized status
            let normalized = normalize_status(&download.status);
            match normalized.as_str() {
                "Completed" | "Cached" => {
                    // Merge completed and cached into single "cached" count for filtering
                    *counts.get_mut("cached").unwrap() += 1;
                }
                "Downloading" => {
                    *counts.get_mut("downloading").unwrap() += 1;
                }
                "Seeding" => {
                    *counts.get_mut("seeding").unwrap() += 1;
                }
                "Paused" => {
                    *counts.get_mut("paused").unwrap() += 1;
                }
                "Failed" => {
                    *counts.get_mut("error").unwrap() += 1;
                }
                "Queued" => {
                    *counts.get_mut("queued").unwrap() += 1;
                }
                "Stalled" => {
                    *counts.get_mut("stalled").unwrap() += 1;
                }
                "Inactive" => {
                    *counts.get_mut("inactive").unwrap() += 1;
                }
                "Expired" => {
                    *counts.get_mut("expired").unwrap() += 1;
                }
                _ => {
                    // Unknown status - could increment a counter if needed
                }
            }
        }
        
        counts
    };

    view! {
        <div class="w-full">
            // Download Counts and Filter/Sort Controls
            <div class="flex flex-col lg:flex-row gap-4 lg:gap-6 mb-6">
                // Download Counts Section
                <div class="flex-shrink-0 lg:w-auto lg:min-w-0">
                    <div class="p-4">
                        // Type Counts
                        <div class="space-y-3">
                            // Total row
                            <div class="flex justify-center items-center text-sm" style="gap: 8px;">
                                <span style="color: var(--text-secondary);">"Total:"</span>
                                <span class="font-medium" style="color: var(--text-primary);">{move || *get_download_counts().get("total").unwrap_or(&0)}</span>
                            </div>
                            
                            // Individual types in horizontal row (clickable)
                            <div class="flex flex-wrap gap-x-4 gap-y-1 items-center text-sm">
                                {
                                    let type_filter_torrent = type_filter.clone();
                                    let get_download_counts_torrent = get_download_counts.clone();
                                    let torrent_count = move || *get_download_counts_torrent().get("torrents").unwrap_or(&0);
                                    view! {
                                        <button
                                            class={move || format!("flex items-center transition-all cursor-pointer {}", if type_filter_torrent.get() == "torrent" { "opacity-100" } else { "opacity-70 hover:opacity-100" })}
                                            style={move || {
                                                if type_filter_torrent.get() == "torrent" {
                                                    "gap: 4px; background-color: rgba(147, 197, 253, 0.2); color: #93c5fd; border: 1px solid rgba(147, 197, 253, 0.4); padding: 2px 6px; border-radius: 4px;".to_string()
                                                } else {
                                                    "gap: 4px;".to_string()
                                                }
                                            }}
                                            on:click=move |_| {
                                                if type_filter_torrent.get() == "torrent" {
                                                    type_filter_torrent.set("all".to_string());
                                                } else {
                                                    type_filter_torrent.set("torrent".to_string());
                                                }
                                            }
                                        >
                                            <span class="text-blue-300">"Torrents:"</span>
                                            <span class="text-blue-300 font-medium">{move || format!("{}", torrent_count())}</span>
                                        </button>
                                    }
                                }
                                {
                                    let type_filter_usenet = type_filter.clone();
                                    let get_download_counts_usenet = get_download_counts.clone();
                                    let usenet_count = move || *get_download_counts_usenet().get("usenet").unwrap_or(&0);
                                    view! {
                                        <button
                                            class={move || format!("flex items-center transition-all cursor-pointer {}", if type_filter_usenet.get() == "usenet" { "opacity-100" } else { "opacity-70 hover:opacity-100" })}
                                            style={move || {
                                                if type_filter_usenet.get() == "usenet" {
                                                    "gap: 4px; background-color: rgba(196, 181, 253, 0.2); color: #c4b5fd; border: 1px solid rgba(196, 181, 253, 0.4); padding: 2px 6px; border-radius: 4px;".to_string()
                                                } else {
                                                    "gap: 4px;".to_string()
                                                }
                                            }}
                                            on:click=move |_| {
                                                if type_filter_usenet.get() == "usenet" {
                                                    type_filter_usenet.set("all".to_string());
                                                } else {
                                                    type_filter_usenet.set("usenet".to_string());
                                                }
                                            }
                                        >
                                            <span class="text-purple-300">"Usenet:"</span>
                                            <span class="text-purple-300 font-medium">{move || format!("{}", usenet_count())}</span>
                                        </button>
                                    }
                                }
                                {
                                    let type_filter_web = type_filter.clone();
                                    let get_download_counts_web = get_download_counts.clone();
                                    let web_count = move || *get_download_counts_web().get("webdl").unwrap_or(&0);
                                    view! {
                                        <button
                                            class={move || format!("flex items-center transition-all cursor-pointer {}", if type_filter_web.get() == "web" { "opacity-100" } else { "opacity-70 hover:opacity-100" })}
                                            style={move || {
                                                if type_filter_web.get() == "web" {
                                                    "gap: 4px; background-color: rgba(134, 239, 172, 0.2); color: #86efac; border: 1px solid rgba(134, 239, 172, 0.4); padding: 2px 6px; border-radius: 4px;".to_string()
                                                } else {
                                                    "gap: 4px;".to_string()
                                                }
                                            }}
                                            on:click=move |_| {
                                                if type_filter_web.get() == "web" {
                                                    type_filter_web.set("all".to_string());
                                                } else {
                                                    type_filter_web.set("web".to_string());
                                                }
                                            }
                                        >
                                            <span class="text-green-300">"Web:"</span>
                                            <span class="text-green-300 font-medium">{move || format!("{}", web_count())}</span>
                                        </button>
                                    }
                                }
                            </div>
                        </div>
                    </div>
                </div>
                
                // Filter and Sort Controls
                <div class="flex-1 min-w-0">
                    <div class="p-4">
                        <div class="flex flex-col gap-3">
                            // Status Filter - Badge-based clickable filters
                            <div class="flex flex-col gap-2">
                                <label class="block text-xs font-medium" style="color: var(--text-secondary);">"Status"</label>
                                <div class="flex flex-wrap gap-1.5 md:gap-3">
                                    {
                                        let status_filter_clone = status_filter.clone();
                                        let get_download_counts_clone = get_download_counts.clone();
                                        view! {
                                            // All button
                                            <button
                                                class={move || format!("px-2 py-0.5 md:px-2.5 md:py-1 rounded-full text-xs font-medium transition-all cursor-pointer {}", if status_filter_clone.get() == "all" { "opacity-100" } else { "opacity-70 hover:opacity-100" })}
                                                style={move || {
                                                    if status_filter_clone.get() == "all" {
                                                        "background-color: rgba(96, 165, 250, 0.2); color: #60a5fa; border: 1.5px solid rgba(96, 165, 250, 0.4);".to_string()
                                                    } else {
                                                        "background-color: rgba(148, 163, 184, 0.1); color: var(--text-secondary); border: 1.5px solid rgba(148, 163, 184, 0.2);".to_string()
                                                    }
                                                }}
                                                on:click=move |_| status_filter_clone.set("all".to_string())
                                            >
                                                {move || format!("All ({})", get_download_counts_clone().get("total").unwrap_or(&0))}
                                            </button>
                                            
                                            // Status badges
                                            {
                                                let status_filter_queued = status_filter.clone();
                                                let get_download_counts_queued = get_download_counts.clone();
                                                let queued_count = move || *get_download_counts_queued().get("queued").unwrap_or(&0);
                                                view! {
                                                    <Show when=move || queued_count() != 0>
                                                        <button
                                                            class={move || format!("px-2 py-0.5 md:px-2.5 md:py-1 rounded-full text-xs font-medium transition-all cursor-pointer {}", if status_filter_queued.get() == "queued" { "opacity-100" } else { "opacity-70 hover:opacity-100" })}
                                                            style={move || {
                                                                if status_filter_queued.get() == "queued" {
                                                                    "background-color: rgba(96, 165, 250, 0.2); color: #60a5fa; border: 1.5px solid rgba(96, 165, 250, 0.4);".to_string()
                                                                } else {
                                                                    "background-color: rgba(96, 165, 250, 0.1); color: #60a5fa; border: 1.5px solid rgba(96, 165, 250, 0.2);".to_string()
                                                                }
                                                            }}
                                                            on:click=move |_| {
                                                                if status_filter_queued.get() == "queued" {
                                                                    status_filter_queued.set("all".to_string());
                                                                } else {
                                                                    status_filter_queued.set("queued".to_string());
                                                                }
                                                            }
                                                        >
                                                            {move || format!("Queued ({})", queued_count())}
                                                        </button>
                                                    </Show>
                                                }
                                            }
                                            
                                            {
                                                let status_filter_downloading = status_filter.clone();
                                                let get_download_counts_downloading = get_download_counts.clone();
                                                let downloading_count = move || *get_download_counts_downloading().get("downloading").unwrap_or(&0);
                                                view! {
                                                    <Show when=move || downloading_count() != 0>
                                                        <button
                                                            class={move || format!("px-2 py-0.5 md:px-2.5 md:py-1 rounded-full text-xs font-medium transition-all cursor-pointer {}", if status_filter_downloading.get() == "downloading" { "opacity-100" } else { "opacity-70 hover:opacity-100" })}
                                                            style={move || {
                                                                if status_filter_downloading.get() == "downloading" {
                                                                    "background-color: rgba(96, 165, 250, 0.2); color: #60a5fa; border: 1.5px solid rgba(96, 165, 250, 0.4);".to_string()
                                                                } else {
                                                                    "background-color: rgba(96, 165, 250, 0.1); color: #60a5fa; border: 1.5px solid rgba(96, 165, 250, 0.2);".to_string()
                                                                }
                                                            }}
                                                            on:click=move |_| {
                                                                if status_filter_downloading.get() == "downloading" {
                                                                    status_filter_downloading.set("all".to_string());
                                                                } else {
                                                                    status_filter_downloading.set("downloading".to_string());
                                                                }
                                                            }
                                                        >
                                                            {move || format!("Downloading ({})", downloading_count())}
                                                        </button>
                                                    </Show>
                                                }
                                            }
                                            
                                            {
                                                let status_filter_seeding = status_filter.clone();
                                                let get_download_counts_seeding = get_download_counts.clone();
                                                let seeding_count = move || *get_download_counts_seeding().get("seeding").unwrap_or(&0);
                                                view! {
                                                    <Show when=move || seeding_count() != 0>
                                                        <button
                                                            class={move || format!("px-2 py-0.5 md:px-2.5 md:py-1 rounded-full text-xs font-medium transition-all cursor-pointer {}", if status_filter_seeding.get() == "seeding" { "opacity-100" } else { "opacity-70 hover:opacity-100" })}
                                                            style={move || {
                                                                if status_filter_seeding.get() == "seeding" {
                                                                    "background-color: rgba(96, 165, 250, 0.2); color: #60a5fa; border: 1.5px solid rgba(96, 165, 250, 0.4);".to_string()
                                                                } else {
                                                                    "background-color: rgba(96, 165, 250, 0.1); color: #60a5fa; border: 1.5px solid rgba(96, 165, 250, 0.2);".to_string()
                                                                }
                                                            }}
                                                            on:click=move |_| {
                                                                if status_filter_seeding.get() == "seeding" {
                                                                    status_filter_seeding.set("all".to_string());
                                                                } else {
                                                                    status_filter_seeding.set("seeding".to_string());
                                                                }
                                                            }
                                                        >
                                                            {move || format!("Seeding ({})", seeding_count())}
                                                        </button>
                                                    </Show>
                                                }
                                            }
                                            
                                            {
                                                let status_filter_cached = status_filter.clone();
                                                let get_download_counts_cached = get_download_counts.clone();
                                                let cached_count = move || *get_download_counts_cached().get("cached").unwrap_or(&0);
                                                view! {
                                                    <Show when=move || cached_count() != 0>
                                                        <button
                                                            class={move || format!("px-2 py-0.5 md:px-2.5 md:py-1 rounded-full text-xs font-medium transition-all cursor-pointer {}", if status_filter_cached.get() == "cached" { "opacity-100" } else { "opacity-70 hover:opacity-100" })}
                                                            style={move || {
                                                                if status_filter_cached.get() == "cached" {
                                                                    "background-color: rgba(34, 197, 94, 0.2); color: #4ade80; border: 1.5px solid rgba(34, 197, 94, 0.4);".to_string()
                                                                } else {
                                                                    "background-color: rgba(34, 197, 94, 0.1); color: #4ade80; border: 1.5px solid rgba(34, 197, 94, 0.2);".to_string()
                                                                }
                                                            }}
                                                            on:click=move |_| {
                                                                if status_filter_cached.get() == "cached" {
                                                                    status_filter_cached.set("all".to_string());
                                                                } else {
                                                                    status_filter_cached.set("cached".to_string());
                                                                }
                                                            }
                                                        >
                                                            {move || format!("Cached ({})", cached_count())}
                                                        </button>
                                                    </Show>
                                                }
                                            }
                                            
                                            {
                                                let status_filter_paused = status_filter.clone();
                                                let get_download_counts_paused = get_download_counts.clone();
                                                let paused_count = move || *get_download_counts_paused().get("paused").unwrap_or(&0);
                                                view! {
                                                    <Show when=move || paused_count() != 0>
                                                        <button
                                                            class={move || format!("px-2 py-0.5 md:px-2.5 md:py-1 rounded-full text-xs font-medium transition-all cursor-pointer {}", if status_filter_paused.get() == "paused" { "opacity-100" } else { "opacity-70 hover:opacity-100" })}
                                                            style={move || {
                                                                if status_filter_paused.get() == "paused" {
                                                                    "background-color: rgba(250, 204, 21, 0.2); color: #facc15; border: 1.5px solid rgba(250, 204, 21, 0.4);".to_string()
                                                                } else {
                                                                    "background-color: rgba(250, 204, 21, 0.1); color: #facc15; border: 1.5px solid rgba(250, 204, 21, 0.2);".to_string()
                                                                }
                                                            }}
                                                            on:click=move |_| {
                                                                if status_filter_paused.get() == "paused" {
                                                                    status_filter_paused.set("all".to_string());
                                                                } else {
                                                                    status_filter_paused.set("paused".to_string());
                                                                }
                                                            }
                                                        >
                                                            {move || format!("Paused ({})", paused_count())}
                                                        </button>
                                                    </Show>
                                                }
                                            }
                                            
                                            {
                                                let status_filter_stalled = status_filter.clone();
                                                let get_download_counts_stalled = get_download_counts.clone();
                                                let stalled_count = move || *get_download_counts_stalled().get("stalled").unwrap_or(&0);
                                                view! {
                                                    <Show when=move || stalled_count() != 0>
                                                        <button
                                                            class={move || format!("px-2 py-0.5 md:px-2.5 md:py-1 rounded-full text-xs font-medium transition-all cursor-pointer {}", if status_filter_stalled.get() == "stalled" { "opacity-100" } else { "opacity-70 hover:opacity-100" })}
                                                            style={move || {
                                                                if status_filter_stalled.get() == "stalled" {
                                                                    "background-color: rgba(248, 113, 113, 0.2); color: #f87171; border: 1.5px solid rgba(248, 113, 113, 0.4);".to_string()
                                                                } else {
                                                                    "background-color: rgba(248, 113, 113, 0.1); color: #f87171; border: 1.5px solid rgba(248, 113, 113, 0.2);".to_string()
                                                                }
                                                            }}
                                                            on:click=move |_| {
                                                                if status_filter_stalled.get() == "stalled" {
                                                                    status_filter_stalled.set("all".to_string());
                                                                } else {
                                                                    status_filter_stalled.set("stalled".to_string());
                                                                }
                                                            }
                                                        >
                                                            {move || format!("Stalled ({})", stalled_count())}
                                                        </button>
                                                    </Show>
                                                }
                                            }
                                            
                                            {
                                                let status_filter_error = status_filter.clone();
                                                let get_download_counts_error = get_download_counts.clone();
                                                let error_count = move || *get_download_counts_error().get("error").unwrap_or(&0);
                                                view! {
                                                    <Show when=move || error_count() != 0>
                                                        <button
                                                            class={move || format!("px-2 py-0.5 md:px-2.5 md:py-1 rounded-full text-xs font-medium transition-all cursor-pointer {}", if status_filter_error.get() == "error" || status_filter_error.get() == "failed" { "opacity-100" } else { "opacity-70 hover:opacity-100" })}
                                                            style={move || {
                                                                if status_filter_error.get() == "error" || status_filter_error.get() == "failed" {
                                                                    "background-color: rgba(248, 113, 113, 0.2); color: #f87171; border: 1.5px solid rgba(248, 113, 113, 0.4);".to_string()
                                                                } else {
                                                                    "background-color: rgba(248, 113, 113, 0.1); color: #f87171; border: 1.5px solid rgba(248, 113, 113, 0.2);".to_string()
                                                                }
                                                            }}
                                                            on:click=move |_| {
                                                                if status_filter_error.get() == "error" || status_filter_error.get() == "failed" {
                                                                    status_filter_error.set("all".to_string());
                                                                } else {
                                                                    status_filter_error.set("error".to_string());
                                                                }
                                                            }
                                                        >
                                                            {move || format!("Failed ({})", error_count())}
                                                        </button>
                                                    </Show>
                                                }
                                            }
                                            
                                            {
                                                let status_filter_expired = status_filter.clone();
                                                let get_download_counts_expired = get_download_counts.clone();
                                                let expired_count = move || *get_download_counts_expired().get("expired").unwrap_or(&0);
                                                view! {
                                                    <Show when=move || expired_count() != 0>
                                                        <button
                                                            class={move || format!("px-2 py-0.5 md:px-2.5 md:py-1 rounded-full text-xs font-medium transition-all cursor-pointer {}", if status_filter_expired.get() == "expired" { "opacity-100" } else { "opacity-70 hover:opacity-100" })}
                                                            style={move || {
                                                                if status_filter_expired.get() == "expired" {
                                                                    "background-color: rgba(156, 163, 175, 0.2); color: #9ca3af; border: 1.5px solid rgba(156, 163, 175, 0.4);".to_string()
                                                                } else {
                                                                    "background-color: rgba(156, 163, 175, 0.1); color: #9ca3af; border: 1.5px solid rgba(156, 163, 175, 0.2);".to_string()
                                                                }
                                                            }}
                                                            on:click=move |_| {
                                                                if status_filter_expired.get() == "expired" {
                                                                    status_filter_expired.set("all".to_string());
                                                                } else {
                                                                    status_filter_expired.set("expired".to_string());
                                                                }
                                                            }
                                                        >
                                                            {move || format!("Expired ({})", expired_count())}
                                                        </button>
                                                    </Show>
                                                }
                                            }
                                        }
                                    }
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                
                // Controls Card
                <div class="flex-shrink-0 lg:w-auto lg:min-w-0">
                    <div class="p-4">
                        <div class="flex items-center justify-center space-x-2">
                            // Blur Toggle Button
                            <button
                                class="px-4 py-2 rounded-lg transition-colors flex items-center space-x-2 text-sm font-medium whitespace-nowrap"
                                style="background-color: transparent; border: 1px solid var(--border-secondary);"
                                on:click=move |_| is_blurred.set(!is_blurred.get())
                                title="Toggle Blur Sensitive Data"
                            >
                                <Show when=move || is_blurred.get()>
                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--accent-secondary);">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"></path>
                                    </svg>
                                </Show>
                                <Show when=move || !is_blurred.get()>
                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--accent-warning);">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.878 9.878L3 3m6.878 6.878L21 21"></path>
                                    </svg>
                                </Show>
                                <span style="color: var(--text-primary);">{move || if is_blurred.get() { "Show" } else { "Blur" }}</span>
                            </button>
                            
                            // Expand/Collapse All Files Button
                            {
                                let has_files = move || {
                                    downloads.get().iter().any(|d| !d.files.is_empty())
                                };
                                let all_files_expanded = move || {
                                    let current_downloads = downloads.get();
                                    let expanded = expanded_file_rows.get();
                                    let downloads_with_files: Vec<i32> = current_downloads.iter()
                                        .filter(|d| !d.files.is_empty())
                                        .map(|d| d.id)
                                        .collect();
                                    downloads_with_files.iter().all(|id| expanded.contains(id))
                                };
                                view! {
                                    <button
                                        class="px-4 py-2 rounded-lg transition-colors flex items-center space-x-2 text-sm font-medium whitespace-nowrap"
                                        style="background-color: transparent; border: 1px solid var(--border-secondary);"
                                        disabled=move || !has_files()
                                        on:click=move |_| toggle_expand_all_files()
                                        title={move || if all_files_expanded() { "Collapse All Files" } else { "Expand All Files" }}
                                    >
                                        <Show when=move || all_files_expanded()>
                                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--accent-primary);">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 15l7-7 7 7"></path>
                                            </svg>
                                        </Show>
                                        <Show when=move || !all_files_expanded()>
                                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--accent-primary);">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
                                            </svg>
                                        </Show>
                                        <span style="color: var(--text-primary);">{move || if all_files_expanded() { "Collapse" } else { "Expand" }}</span>
                                    </button>
                                }
                            }
                            
                            // Configure Button
                            <button
                                class="px-4 py-2 rounded-lg transition-colors flex items-center space-x-2 text-sm font-medium whitespace-nowrap"
                                style="background-color: transparent; border: 1px solid var(--border-secondary);"
                                title="Configure Settings"
                            >
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--accent-primary);">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
                                </svg>
                                <span style="color: var(--text-primary);">"Config"</span>
                            </button>
                        </div>
                    </div>
                </div>
            </div>

            <Show when=move || loading.get()>
                <LoadingSpinner 
                    size=SpinnerSize::Medium 
                    variant=SpinnerVariant::Accent 
                    text="Loading table...".to_string()
                    centered=true
                />
            </Show>

            <Show when=move || error.get().is_some()>
                <div class="bg-red-900/20 border border-red-500/50 rounded-lg p-4 mb-6">
                    <div class="text-red-400">
                        <span>{move || error.get().unwrap_or_default()}</span>
                    </div>
                </div>
            </Show>


            // Bulk Actions Bar - Always visible but grayed out when nothing selected
            <div class={move || format!("rounded-lg p-4 mb-4 transition-all duration-200 {}", if show_bulk_actions.get() { "" } else { "opacity-50" })} style={move || format!("background: var(--bg-card); border: 1px solid var(--border-secondary); {}", if show_bulk_actions.get() { "" } else { "opacity: 0.5;" })}>
                <div class="flex items-center justify-between">
                    <div class="flex items-center space-x-4">
                        <span class={move || format!("text-sm transition-colors {}", if show_bulk_actions.get() { "text-white" } else { "" })} style={move || format!("color: var(--text-primary); {}", if show_bulk_actions.get() { "" } else { "opacity: 0.6;" })}>
                            {move || {
                                let state = selection_state.get();
                                let item_count = state.selected_items.len();
                                let file_count = state.selected_files.values().map(|files| files.len()).sum::<usize>();
                                if item_count > 0 && file_count > 0 {
                                    format!("{} items and {} files selected", item_count, file_count)
                                } else if item_count > 0 {
                                    format!("{} items selected", item_count)
                                } else if file_count > 0 {
                                    format!("{} files selected", file_count)
                                } else {
                                    "No items selected".to_string()
                                }
                            }}
                        </span>
                    </div>
                    <div class="flex items-center space-x-2">
                        <button
                            class={move || format!("px-4 py-2 text-sm rounded-lg transition-colors {}", if show_bulk_actions.get() { "" } else { "cursor-not-allowed" })}
                            style={move || if show_bulk_actions.get() { "background-color: var(--accent-secondary); color: var(--text-primary);" } else { "background-color: var(--bg-tertiary); color: var(--text-muted);" }}
                            disabled=move || !show_bulk_actions.get()
                            on:click=move |_| if show_bulk_actions.get() { handle_bulk_download() }
                        >
                            "Download Selected"
                        </button>
                        <button
                            class={move || format!("px-4 py-2 text-sm rounded-lg transition-colors {}", if show_bulk_actions.get() { "" } else { "cursor-not-allowed" })}
                            style={move || if show_bulk_actions.get() { "background-color: var(--accent-danger); color: var(--text-primary);" } else { "background-color: var(--bg-tertiary); color: var(--text-muted);" }}
                            disabled=move || !show_bulk_actions.get()
                            on:click=move |_| if show_bulk_actions.get() { handle_bulk_delete() }
                        >
                            "Delete Selected"
                        </button>
                        <button
                            class={move || format!("px-4 py-2 text-sm rounded-lg transition-colors {}", if show_bulk_actions.get() { "" } else { "cursor-not-allowed" })}
                            style={move || if show_bulk_actions.get() { "background-color: var(--bg-tertiary); color: var(--text-primary);" } else { "background-color: var(--bg-tertiary); color: var(--text-muted);" }}
                            disabled=move || !show_bulk_actions.get()
                            on:click=move |_| if show_bulk_actions.get() { clear_selection() }
                        >
                            "Clear Selection"
                        </button>
                    </div>
                </div>
            </div>

            <Show when=move || !loading.get() && error.get().is_none()>
                <div class="rounded-xl border" style="background-color: var(--bg-card); border-color: var(--border-secondary);">
                    <div style="height: 70vh; overflow-y: scroll; overflow-x: auto; display: block;">
                        <table class="table-fixed" style="width: 100%; min-width: 1200px;">
                            <thead class="border-b-2" style="position: sticky; top: 0; z-index: 10; background-color: var(--bg-card); border-color: var(--border-primary);">
                                <tr>
                                    <th class="px-4 py-4 text-center text-sm font-semibold uppercase tracking-wide border-r" style="width: 50px; color: var(--text-primary); border-color: var(--border-secondary);">
                                        <input
                                            type="checkbox"
                                            class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                                            checked=move || is_all_selected()
                                            class:opacity-50=move || is_partially_selected()
                                            on:change=move |_| toggle_select_all()
                                        />
                                    </th>
                                    <th class="px-4 py-4 text-center text-sm font-bold uppercase tracking-wide border-r" style="width: 350px; max-width: 350px; color: var(--text-primary); border-color: var(--border-secondary);">
                                        <button
                                            class="flex items-center justify-center gap-2 hover:opacity-80 transition-opacity cursor-pointer w-full font-bold"
                                            style="color: var(--text-primary);"
                                            on:click=move |_| {
                                                let current_sort = sort_by.get();
                                                if current_sort == "name" {
                                                    // Toggle sort order if clicking same column
                                                    sort_order.set(if sort_order.get() == "asc" { "desc".to_string() } else { "asc".to_string() });
                                                } else {
                                                    sort_by.set("name".to_string());
                                                    sort_order.set("asc".to_string());
                                                }
                                            }
                                        >
                                            <span class="font-bold">"Name"</span>
                                            <Show when=move || sort_by.get() == "name">
                                                <span class="font-bold" style="color: var(--accent-primary);">
                                                    {move || if sort_order.get() == "asc" { "↑" } else { "↓" }}
                                                </span>
                                            </Show>
                                        </button>
                                    </th>
                                    <th class="px-6 py-4 text-center text-sm font-bold uppercase tracking-wide border-r" style="width: 120px; color: var(--text-primary); border-color: var(--border-secondary);">
                                        <button
                                            class="flex items-center justify-center gap-2 hover:opacity-80 transition-opacity cursor-pointer w-full font-bold"
                                            style="color: var(--text-primary);"
                                            on:click=move |_| {
                                                let current_sort = sort_by.get();
                                                if current_sort == "size" {
                                                    sort_order.set(if sort_order.get() == "asc" { "desc".to_string() } else { "asc".to_string() });
                                                } else {
                                                    sort_by.set("size".to_string());
                                                    sort_order.set("asc".to_string());
                                                }
                                            }
                                        >
                                            <span class="font-bold">"Size"</span>
                                            <Show when=move || sort_by.get() == "size">
                                                <span class="font-bold" style="color: var(--accent-primary);">
                                                    {move || if sort_order.get() == "asc" { "↑" } else { "↓" }}
                                                </span>
                                            </Show>
                                        </button>
                                    </th>
                                    <th class="px-6 py-4 text-center text-sm font-bold uppercase tracking-wide border-r" style="width: 140px; color: var(--text-primary); border-color: var(--border-secondary);">
                                        <button
                                            class="flex items-center justify-center gap-2 hover:opacity-80 transition-opacity cursor-pointer w-full font-bold"
                                            style="color: var(--text-primary);"
                                            on:click=move |_| {
                                                let current_sort = sort_by.get();
                                                if current_sort == "date" {
                                                    sort_order.set(if sort_order.get() == "asc" { "desc".to_string() } else { "asc".to_string() });
                                                } else {
                                                    sort_by.set("date".to_string());
                                                    sort_order.set("asc".to_string());
                                                }
                                            }
                                        >
                                            <span class="font-bold">"Added"</span>
                                            <Show when=move || sort_by.get() == "date">
                                                <span class="font-bold" style="color: var(--accent-primary);">
                                                    {move || if sort_order.get() == "asc" { "↑" } else { "↓" }}
                                                </span>
                                            </Show>
                                        </button>
                                    </th>
                                    <th class="px-6 py-4 text-center text-sm font-bold uppercase tracking-wide border-r" style="width: 120px; color: var(--text-primary); border-color: var(--border-secondary);">
                                        <button
                                            class="flex items-center justify-center gap-2 hover:opacity-80 transition-opacity cursor-pointer w-full font-bold"
                                            style="color: var(--text-primary);"
                                            on:click=move |_| {
                                                let current_sort = sort_by.get();
                                                if current_sort == "status" {
                                                    sort_order.set(if sort_order.get() == "asc" { "desc".to_string() } else { "asc".to_string() });
                                                } else {
                                                    sort_by.set("status".to_string());
                                                    sort_order.set("asc".to_string());
                                                }
                                            }
                                        >
                                            <span class="font-bold">"Status"</span>
                                            <Show when=move || sort_by.get() == "status">
                                                <span class="font-bold" style="color: var(--accent-primary);">
                                                    {move || if sort_order.get() == "asc" { "↑" } else { "↓" }}
                                                </span>
                                            </Show>
                                        </button>
                                    </th>
                                    <th class="px-6 py-4 text-center text-sm font-bold uppercase tracking-wide border-r" style="width: 80px; color: var(--text-primary); border-color: var(--border-secondary);">
                                        <button
                                            class="flex items-center justify-center gap-2 hover:opacity-80 transition-opacity cursor-pointer w-full font-bold"
                                            style="color: var(--text-primary);"
                                            on:click=move |_| {
                                                let current_sort = sort_by.get();
                                                if current_sort == "type" {
                                                    sort_order.set(if sort_order.get() == "asc" { "desc".to_string() } else { "asc".to_string() });
                                                } else {
                                                    sort_by.set("type".to_string());
                                                    sort_order.set("asc".to_string());
                                                }
                                            }
                                        >
                                            <span class="font-bold">"Type"</span>
                                            <Show when=move || sort_by.get() == "type">
                                                <span class="font-bold" style="color: var(--accent-primary);">
                                                    {move || if sort_order.get() == "asc" { "↑" } else { "↓" }}
                                                </span>
                                            </Show>
                                        </button>
                                    </th>
                                    <th class="px-6 py-4 text-center text-sm font-bold uppercase tracking-wide border-r" style="width: 200px; color: var(--text-primary); border-color: var(--border-secondary);">
                                        <button
                                            class="flex items-center justify-center gap-2 hover:opacity-80 transition-opacity cursor-pointer w-full font-bold"
                                            style="color: var(--text-primary);"
                                            on:click=move |_| {
                                                let current_sort = sort_by.get();
                                                if current_sort == "progress" {
                                                    sort_order.set(if sort_order.get() == "asc" { "desc".to_string() } else { "asc".to_string() });
                                                } else {
                                                    sort_by.set("progress".to_string());
                                                    sort_order.set("asc".to_string());
                                                }
                                            }
                                        >
                                            <span class="font-bold">"Progress"</span>
                                            <Show when=move || sort_by.get() == "progress">
                                                <span class="font-bold" style="color: var(--accent-primary);">
                                                    {move || if sort_order.get() == "asc" { "↑" } else { "↓" }}
                                                </span>
                                            </Show>
                                        </button>
                                    </th>
                                    <th class="px-6 py-4 text-left text-sm font-semibold uppercase tracking-wide" style="width: 140px; color: var(--text-primary);">
                                        "Actions"
                                    </th>
                                </tr>
                            </thead>
                            <tbody class="divide-y divide-slate-700/30">
                                <For each=move || filtered_downloads.get() key=|download| download.id let:download>
                                    {
                                        let is_expanded = move || expanded_rows.get().contains(&download.id);
                                        let download_clone = download.clone();
                                        let pause_resume_clone = download.clone();
                                        let delete_clone = download.clone();
                                        let stream_clone = download.clone();
                                        let reannounce_clone = download.clone();
                                        let cloud_upload_clone = download.clone();
                                        let files_for_check = download.files.clone();
                                        let files_empty_check = !files_for_check.is_empty();
                                        let download_id_for_files = download.id;
                                        let files_for_display = files_for_check.clone();
                                        let download_for_files_display = download.clone();
                                        
                                        
                                        view! {
                                            <>
                                                <tr class={move || format!("hover:bg-slate-700/30 transition-colors {}", if selection_state.get().is_item_selected(download.id) { "bg-slate-700/20" } else { "" })}>
                                                    <td class="px-4 py-4 text-center" style="width: 50px;">
                                                        <input
                                                            type="checkbox"
                                                            class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                                                            checked=move || selection_state.get().is_item_selected(download.id)
                                                            on:change=move |_| toggle_item_selection(download.id)
                                                        />
                                                    </td>
                                                    <td class="px-4 py-4 overflow-hidden" style="width: 350px; max-width: 350px;">
                                                        <div class="flex items-center space-x-2">
                                                            <Show when=move || files_empty_check>
                                                                <button
                                                                    class="text-slate-400 hover:text-white transition-colors flex-shrink-0"
                                                                    on:click=move |_| toggle_file_expansion(download.id)
                                                                    title="Show/Hide Files"
                                                                >
                                                                    <Show when=move || expanded_file_rows.get().contains(&download.id)>
                                                                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
                                                                        </svg>
                                                                    </Show>
                                                                    <Show when=move || !expanded_file_rows.get().contains(&download.id)>
                                                                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                                                                        </svg>
                                                                    </Show>
                                                                </button>
                                                            </Show>
                                                            <Show when=move || !files_empty_check>
                                                                <div class="w-4 flex-shrink-0"></div>
                                                            </Show>
                                                            <div class="w-full">
                                                                        <div class="flex items-center gap-2">
                                                                            <p class={move || format!("text-sm font-medium text-white truncate {}", if is_blurred.get() { "blur-sm select-none" } else { "" })} style={move || format!("word-break: break-all; {}", if is_blurred.get() { "filter: blur(4px);" } else { "" })} title={move || if is_blurred.get() { "Hidden".to_string() } else { download.name.clone() }}>{download.name.clone()}</p>
                                                                            <Show when=move || download.private && download.download_type == DownloadType::Torrent>
                                                                                <svg class="w-4 h-4 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24" title="Private Tracker" style="color: #f97316;">
                                                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"></path>
                                                                                </svg>
                                                                            </Show>
                                                                        </div>
                                                                <p class={move || format!("text-xs text-slate-400 {}", if is_blurred.get() { "blur-sm select-none" } else { "" })} style={move || if is_blurred.get() { "filter: blur(4px);" } else { "" }}>
                                                                    {format!("ID: {}", download.id)}
                                                                </p>
                                                            </div>
                                                        </div>
                                                    </td>
                                                     <td class="px-6 py-4 text-sm text-slate-300" style="width: 120px;">
                                                         <span class={format!("{}", if is_blurred.get() { "select-none" } else { "" })} style={if is_blurred.get() { "opacity: 0.3; text-shadow: 0 0 8px rgba(255,255,255,0.5);" } else { "" }}>
                                                             {format_size(download.size)}
                                                         </span>
                                                     </td>
                                                     <td class="px-6 py-4 text-sm text-slate-300" style="width: 140px;">
                                                         <span class={format!("{}", if is_blurred.get() { "select-none" } else { "" })} style={if is_blurred.get() { "opacity: 0.3; text-shadow: 0 0 8px rgba(255,255,255,0.5);" } else { "" }}>
                                                             {format_date(&download.created_at)}
                                                         </span>
                                                     </td>
                                                    <td class="px-6 py-4" style="width: 120px;">
                                                        <span 
                                                            class={format!("inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {}", if is_blurred.get() { "select-none" } else { "" })} 
                                                            style={format!("{} {}", get_status_badge_style(&download.status), if is_blurred.get() { "opacity: 0.3; text-shadow: 0 0 8px rgba(255,255,255,0.5);" } else { "" })}
                                                        >
                                                            {normalize_status(&download.status)}
                                                        </span>
                                                    </td>
                                                    <td class="px-6 py-4" style="width: 80px;">
                                                        <span class={format!("inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {}", if is_blurred.get() { "select-none" } else { "" })} class:bg-blue-900={move || download.download_type == DownloadType::Torrent} class:text-blue-300={move || download.download_type == DownloadType::Torrent} class:bg-green-900={move || download.download_type == DownloadType::WebDownload} class:text-green-300={move || download.download_type == DownloadType::WebDownload} class:bg-purple-900={move || download.download_type == DownloadType::Usenet} class:text-purple-300={move || download.download_type == DownloadType::Usenet} style={if is_blurred.get() { "opacity: 0.3; text-shadow: 0 0 8px rgba(255,255,255,0.5);" } else { "" }}>
                                                            {download.download_type.to_string()}
                                                        </span>
                                                    </td>
                                                    <td class="px-6 py-4" style="width: 200px;">
                                                        {
                                                            // Pre-calculate values for progress display
                                                            let progress_for_bar = download.progress;
                                                            let download_speed_for_display = download.download_speed;
                                                            let upload_speed_for_display = download.upload_speed;
                                                            let total_downloaded_val = download.total_downloaded;
                                                            let size_val = download.size;
                                                            let progress_val = download.progress;
                                                            let eta_val = download.eta;
                                                            let status_for_bar = download.status.clone();
                                                            let status_lower = download.status.to_lowercase();
                                                            let is_downloading = status_lower == "downloading" || status_lower == "active" || status_lower.contains("downloading");
                                                            let is_seeding = status_lower.contains("seeding") || status_lower.contains("uploading");
                                                            
                                                            view! {
                                                                <div class="space-y-2">
                                                                    <Show when=move || { progress_for_bar > 0.0 || download_speed_for_display > 0 || upload_speed_for_display > 0 }>
                                                                        <div class="w-full h-4 rounded-full overflow-hidden border" style={format!("background-color: var(--progress-bg); border-color: var(--progress-border);")}>
                                                                            <div 
                                                                                class="h-full rounded-full transition-all duration-500 ease-out shadow-sm"
                                                                                style={format!("width: {:.1}%; min-width: 2px; {}", (progress_for_bar * 100.0).max(0.1), get_progress_bar_style(&status_for_bar, progress_for_bar))}
                                                                            ></div>
                                                                        </div>
                                                                    </Show>
                                                                    <Show when=move || { progress_for_bar == 0.0 && download_speed_for_display == 0 && upload_speed_for_display == 0 }>
                                                                        <div class="w-full h-4 flex items-center justify-center">
                                                                            <span class="text-xs font-mono" style="color: var(--text-muted);">"----"</span>
                                                                        </div>
                                                                    </Show>
                                                                    <div class="flex justify-between items-center text-xs">
                                                                        <span class={format!("font-medium {}", if is_blurred.get() { "select-none" } else { "" })} style={format!("color: var(--text-secondary); {}", if is_blurred.get() { "opacity: 0.3; text-shadow: 0 0 8px rgba(255,255,255,0.5);" } else { "" })}>{format!("{:.1}%", progress_for_bar * 100.0)}</span>
                                                                        <Show when=move || { download_speed_for_display > 0i64 && !is_seeding }>
                                                                            <span style="color: #34D399;">
                                                                                {
                                                                                    let speed_bytes = download_speed_for_display as f64;
                                                                                    let speed_tb = speed_bytes / 1_099_511_627_776.0; // 1024^4
                                                                                    let speed_gb = speed_bytes / 1_073_741_824.0; // 1024^3
                                                                                    let speed_mb = speed_bytes / 1_048_576.0; // 1024^2
                                                                                    let speed_kb = speed_bytes / 1024.0;
                                                                                    
                                                                                    if speed_tb >= 1.0 {
                                                                                        format!("↓ {:.2} TB/s", speed_tb)
                                                                                    } else if speed_gb >= 1.0 {
                                                                                        format!("↓ {:.2} GB/s", speed_gb)
                                                                                    } else if speed_mb >= 1.0 {
                                                                                        format!("↓ {:.2} MB/s", speed_mb)
                                                                                    } else {
                                                                                        format!("↓ {:.2} KB/s", speed_kb)
                                                                                    }
                                                                                }
                                                                            </span>
                                                                        </Show>
                                                                        <Show when=move || { is_seeding }>
                                                                            <span style="color: #F87171;">
                                                                                {
                                                                                    if upload_speed_for_display > 0i64 {
                                                                                        let speed_bytes = upload_speed_for_display as f64;
                                                                                        let speed_tb = speed_bytes / 1_099_511_627_776.0; // 1024^4
                                                                                        let speed_gb = speed_bytes / 1_073_741_824.0; // 1024^3
                                                                                        let speed_mb = speed_bytes / 1_048_576.0; // 1024^2
                                                                                        let speed_kb = speed_bytes / 1024.0;
                                                                                        
                                                                                        if speed_tb >= 1.0 {
                                                                                            format!("↑ {:.2} TB/s", speed_tb)
                                                                                        } else if speed_gb >= 1.0 {
                                                                                            format!("↑ {:.2} GB/s", speed_gb)
                                                                                        } else if speed_mb >= 1.0 {
                                                                                            format!("↑ {:.2} MB/s", speed_mb)
                                                                                        } else {
                                                                                            format!("↑ {:.2} KB/s", speed_kb)
                                                                                        }
                                                                                    } else {
                                                                                        "↑ 0.00 KB/s".to_string()
                                                                                    }
                                                                                }
                                                                            </span>
                                                                        </Show>
                                                                    </div>
                                                                    <Show when=move || is_downloading && (download_speed_for_display > 0 || progress_for_bar > 0.0)>
                                                                        <div class="flex justify-between items-center text-xs">
                                                                            <Show when=move || {
                                                                                let downloaded = total_downloaded_val.unwrap_or_else(|| {
                                                                                    (size_val as f64 * progress_val as f64) as i64
                                                                                });
                                                                                downloaded > 0 || size_val > 0
                                                                            }>
                                                                                <span style="color: var(--text-muted);">
                                                                                    {
                                                                                        let downloaded = total_downloaded_val.unwrap_or_else(|| {
                                                                                            (size_val as f64 * progress_val as f64) as i64
                                                                                        });
                                                                                        format!("{} / {}", format_size(downloaded), format_size(size_val))
                                                                                    }
                                                                                </span>
                                                                            </Show>
                                                                            <Show when=move || {
                                                                                if let Some(eta) = eta_val {
                                                                                    eta > 0
                                                                                } else {
                                                                                    false
                                                                                }
                                                                            }>
                                                                                <span style="color: var(--text-muted);">
                                                                                    {
                                                                                        if let Some(eta) = eta_val {
                                                                                            format!("ETA: {}", format_eta(eta))
                                                                                        } else {
                                                                                            String::new()
                                                                                        }
                                                                                    }
                                                                                </span>
                                                                            </Show>
                                                                        </div>
                                                                    </Show>
                                                                </div>
                                                            }
                                                        }
                                                    </td>
                                                    <td class="px-6 py-4" style="width: 140px;">
                                                        <div class="actions-container flex space-x-1">
                                                            // Download Button - clone status once per closure usage
                                                            {
                                                                let status_class = download.status.clone();
                                                                let status_style = download.status.clone();
                                                                let status_disabled = download.status.clone();
                                                                let status_click = download.status.clone();
                                                                let status_title = download.status.clone();
                                                                let status_svg = download.status.clone();
                                                                view! {
                                                                    <button
                                                                        class={move || format!("p-2 rounded transition-colors flex items-center justify-center {}", if is_download_enabled(&status_class) { "" } else { "cursor-not-allowed opacity-50" })}
                                                                        style={move || if is_download_enabled(&status_style) { "background-color: transparent;" } else { "background-color: transparent; opacity: 0.5;" }}
                                                                        disabled=move || !is_download_enabled(&status_disabled)
                                                                        on:click=move |_| if is_download_enabled(&status_click) { handle_download(download_clone.id, download_clone.download_type.clone(), None) }
                                                                        title={move || if is_download_enabled(&status_title) { "Download".to_string() } else { "Download not available for this status".to_string() }}
                                                                    >
                                                                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" style={move || if is_download_enabled(&status_svg) { "color: var(--accent-secondary);" } else { "color: var(--text-muted);" }}>
                                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"></path>
                                                                        </svg>
                                                                    </button>
                                                                }
                                                            }
                                                            
                                                            // Pause/Resume Button - clone status once per closure usage
                                                            {
                                                                let status_class = download.status.clone();
                                                                let status_style = download.status.clone();
                                                                let status_disabled = download.status.clone();
                                                                let status_click = download.status.clone();
                                                                let status_title = download.status.clone();
                                                                let status_svg = download.status.clone();
                                                                view! {
                                                                    <button
                                                                        class={move || format!("p-2 rounded transition-colors flex items-center justify-center {}", if is_pause_resume_enabled(&status_class) { "" } else { "cursor-not-allowed opacity-50" })}
                                                                        style={move || if is_pause_resume_enabled(&status_style) { "background-color: transparent;" } else { "background-color: transparent; opacity: 0.5;" }}
                                                                        disabled=move || !is_pause_resume_enabled(&status_disabled)
                                                                        on:click=move |_| if is_pause_resume_enabled(&status_click) { handle_pause_resume(pause_resume_clone.id, pause_resume_clone.download_type.clone(), pause_resume_clone.status.clone()) }
                                                                        title={move || if is_pause_resume_enabled(&status_title) { 
                                                                            match status_title.to_lowercase().as_str() {
                                                                                "downloading" | "active" | "seeding" => "Pause".to_string(),
                                                                                "paused" | "stopped" | "stalled" => "Resume".to_string(),
                                                                                _ => "Pause/Resume".to_string(),
                                                                            }
                                                                        } else { "Pause/Resume not available for this status".to_string() }}
                                                                    >
                                                                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" style={move || if is_pause_resume_enabled(&status_svg) { "color: var(--accent-warning);" } else { "color: var(--text-muted);" }}>
                                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 9v6m4-6v6m7-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                                                        </svg>
                                                                    </button>
                                                                }
                                                            }
                                                            
                                                            // Delete Button - clone status once per closure usage
                                                            {
                                                                let status_class = download.status.clone();
                                                                let status_style = download.status.clone();
                                                                let status_disabled = download.status.clone();
                                                                let status_click = download.status.clone();
                                                                let status_title = download.status.clone();
                                                                let status_svg = download.status.clone();
                                                                view! {
                                                                    <button
                                                                        class={move || format!("p-2 rounded transition-colors flex items-center justify-center {}", if is_delete_enabled(&status_class) { "" } else { "cursor-not-allowed opacity-50" })}
                                                                        style={move || if is_delete_enabled(&status_style) { "background-color: transparent;" } else { "background-color: transparent; opacity: 0.5;" }}
                                                                        disabled=move || !is_delete_enabled(&status_disabled)
                                                                        on:click=move |_| if is_delete_enabled(&status_click) { handle_delete(delete_clone.id, delete_clone.download_type.clone()) }
                                                                        title={move || if is_delete_enabled(&status_title) { "Delete".to_string() } else { "Delete not available".to_string() }}
                                                                    >
                                                                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" style={move || if is_delete_enabled(&status_svg) { "color: var(--accent-danger);" } else { "color: var(--text-muted);" }}>
                                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path>
                                                                        </svg>
                                                                    </button>
                                                                }
                                                            }
                                                            
                                                            // Reannounce Button (for stalled torrents)
                                                            {
                                                                let reannounce_status = download.status.clone();
                                                                let reannounce_type = download.download_type;
                                                                view! {
                                                                    <Show when=move || is_reannounce_enabled(&reannounce_status, reannounce_type)>
                                                                        <button
                                                                            class="p-2 rounded transition-colors flex items-center justify-center"
                                                                            style="background-color: transparent;"
                                                                            on:click=move |_| handle_reannounce(reannounce_clone.id, reannounce_clone.download_type.clone())
                                                                            title="Reannounce torrent"
                                                                        >
                                                                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--accent-warning);">
                                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"></path>
                                                                            </svg>
                                                                        </button>
                                                                    </Show>
                                                                }
                                                            }
                                                            
                                                            // Stream Button - clone status once per closure usage
                                                            {
                                                                let status_class = download.status.clone();
                                                                let status_style = download.status.clone();
                                                                let status_disabled = download.status.clone();
                                                                let status_click = download.status.clone();
                                                                let status_title = download.status.clone();
                                                                let status_svg = download.status.clone();
                                                                view! {
                                                                    <button
                                                                        class={move || format!("p-2 rounded transition-colors flex items-center justify-center {}", if is_stream_enabled(&status_class) { "" } else { "cursor-not-allowed opacity-50" })}
                                                                        style={move || if is_stream_enabled(&status_style) { "background-color: transparent;" } else { "background-color: transparent; opacity: 0.5;" }}
                                                                        disabled=move || !is_stream_enabled(&status_disabled)
                                                                        on:click=move |_| if is_stream_enabled(&status_click) { handle_stream(stream_clone.id, stream_clone.download_type.clone(), None) }
                                                                        title={move || {
                                                                            if !has_streaming_plan() {
                                                                                "Streaming requires Plan 2".to_string()
                                                                            } else if is_stream_enabled(&status_title) {
                                                                                "Stream".to_string()
                                                                            } else {
                                                                                "Stream not available for this status".to_string()
                                                                            }
                                                                        }}
                                                                    >
                                                                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" style={move || if is_stream_enabled(&status_svg) { "color: var(--accent-primary);" } else { "color: var(--text-muted);" }}>
                                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"></path>
                                                                        </svg>
                                                                    </button>
                                                                }
                                                            }
                                                            
                                                            // More Options Button (dots menu) - Always on the far right
                                                            <div class="relative inline-block text-left" on:click=move |ev| ev.stop_propagation()>
                                                                <button
                                                                    class="p-2 rounded transition-colors flex items-center justify-center"
                                                                    style="background-color: transparent;"
                                                                    title="More options"
                                                                    on:click=move |_| toggle_dropdown(download.id)
                                                                >
                                                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--text-secondary);">
                                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 5v.01M12 12v.01M12 19v.01M12 6a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2z"></path>
                                                                    </svg>
                                                                </button>
                                                                <Show when=move || open_dropdown.get() == Some(download.id)>
                                                                    <div class="absolute z-50 mt-2 w-48 rounded-md shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none" style="background: var(--bg-tertiary); border: 1px solid var(--border-secondary); left: 0; transform: translateX(-100%);" on:click=move |ev| ev.stop_propagation()>
                                                                    <div class="py-1">
                                                                        // Cloud Upload Options
                                                                        <button
                                                                            class="block w-full px-4 py-2 text-sm transition-colors hover:opacity-80"
                                                                            style="color: var(--text-secondary);"
                                                                            on:click=move |_| {
                                                                                handle_cloud_upload(cloud_upload_clone.id, cloud_upload_clone.download_type.clone(), "google".to_string());
                                                                                close_dropdown();
                                                                            }
                                                                        >
                                                                            "Google Drive"
                                                                        </button>
                                                                        <button
                                                                            class="block w-full px-4 py-2 text-sm transition-colors hover:opacity-80"
                                                                            style="color: var(--text-secondary);"
                                                                            on:click=move |_| {
                                                                                handle_cloud_upload(cloud_upload_clone.id, cloud_upload_clone.download_type.clone(), "dropbox".to_string());
                                                                                close_dropdown();
                                                                            }
                                                                        >
                                                                            "Dropbox"
                                                                        </button>
                                                                        <button
                                                                            class="block w-full px-4 py-2 text-sm transition-colors hover:opacity-80"
                                                                            style="color: var(--text-secondary);"
                                                                            on:click=move |_| {
                                                                                handle_cloud_upload(cloud_upload_clone.id, cloud_upload_clone.download_type.clone(), "onedrive".to_string());
                                                                                close_dropdown();
                                                                            }
                                                                        >
                                                                            "OneDrive"
                                                                        </button>
                                                                    </div>
                                                                </div>
                                                                </Show>
                                                            </div>
                                                        </div>
                                                    </td>
                                                </tr>
                                                
                                                // Files dropdown - Show files for all download types when expanded
                                                <Show when=move || expanded_file_rows.get().contains(&download_id_for_files) && files_empty_check>
                                                    {
                                                        let download_files_count = files_for_display.len();
                                                        let files_list_for_for = files_for_display.clone();
                                                        let download_for_for = download_for_files_display.clone();
                                                        let is_blurred_for_files = is_blurred.clone();
                                                        view! {
                                                            <tr>
                                                                <td colspan="8" class="px-6 py-4 bg-slate-700/20">
                                                                    <div class="space-y-2">
                                                                        <h4 class="text-sm font-medium text-slate-300 mb-3">
                                                                            {format!("Files ({})", download_files_count)}
                                                                        </h4>
                                                                        <div class="overflow-x-auto">
                                                                            <table class="w-full text-left border-collapse">
                                                                                <thead>
                                                                                    <tr class="border-b border-slate-600">
                                                                                        <th class="px-4 py-2 text-xs font-medium text-slate-400">"File Name"</th>
                                                                                        <th class="px-4 py-2 text-xs font-medium text-slate-400">"Size"</th>
                                                                                        <th class="px-4 py-2 text-xs font-medium text-slate-400">"MIME Type"</th>
                                                                                        <th class="px-4 py-2 text-xs font-medium text-slate-400">"Path"</th>
                                                                                        <th class="px-4 py-2 text-xs font-medium text-slate-400">"Actions"</th>
                                                                                    </tr>
                                                                                </thead>
                                                                                <tbody class="divide-y divide-slate-700/30">
                                                                                    <For each=move || files_list_for_for.clone() key=|file| file.id let:file>
                                                                                {
                                                                                    let file_download_clone = download_for_for.clone();
                                                                                    let file_clone = file.clone();
                                                                                    view! {
                                                                                        <tr class="hover:bg-slate-700/10">
                                                                                            <td class="px-4 py-2 text-sm text-slate-300">
                                                                                                <div class="flex items-center space-x-2">
                                                                                                    <Show when=move || file.zipped.unwrap_or(false)>
                                                                                                        <svg class="w-4 h-4 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" title="Zipped">
                                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4"></path>
                                                                        </svg>
                                                                                                    </Show>
                                                                                                    <Show when=move || file.infected.unwrap_or(false)>
                                                                                                        <svg class="w-4 h-4 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" title="Infected">
                                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path>
                                                                        </svg>
                                                                                                    </Show>
                                                                                                    <span class={move || format!("truncate {}", if is_blurred_for_files.get() { "blur-sm select-none" } else { "" })} style={move || format!("{}", if is_blurred_for_files.get() { "filter: blur(4px);" } else { "" })} title={move || if is_blurred_for_files.get() { "Hidden".to_string() } else { file.name.clone() }}>
                                                                                                        {file.short_name.clone().unwrap_or_else(|| file.name.clone())}
                                                                                                    </span>
                                                                                                </div>
                                                                                            </td>
                                                                                            <td class="px-4 py-2 text-sm text-slate-400">
                                                                                                {format_size(file.size)}
                                                                                            </td>
                                                                                            <td class="px-4 py-2 text-sm text-slate-400">
                                                                                                {file.mimetype.clone().unwrap_or_else(|| "N/A".to_string())}
                                                                                            </td>
                                                                                            <td class={move || format!("px-4 py-2 text-sm text-slate-500 truncate max-w-xs {}", if is_blurred_for_files.get() { "blur-sm select-none" } else { "" })} style={move || format!("{}", if is_blurred_for_files.get() { "filter: blur(4px); opacity: 0.3; text-shadow: 0 0 8px rgba(255,255,255,0.5);" } else { "" })} title={move || if is_blurred_for_files.get() { "Hidden".to_string() } else { file.absolute_path.clone().unwrap_or_else(|| "N/A".to_string()) }}>
                                                                                                {file.s3_path.clone().unwrap_or_else(|| "N/A".to_string())}
                                                                                            </td>
                                                                                            <td class="px-4 py-2">
                                                                                                <div class="flex items-center space-x-2">
                                                                                                    <button
                                                                                                        class="p-2 bg-green-600 hover:bg-green-700 text-white rounded transition-colors flex items-center justify-center"
                                                                                                        on:click=move |_| {
                                                                                                            let download_id = file_download_clone.id;
                                                                                                            let download_type = file_download_clone.download_type.clone();
                                                                                                            let file_id = file_clone.id;
                                                                                                            handle_download(download_id, download_type, Some(file_id));
                                                                                                        }
                                                                                                        title="Download File"
                                                                                                    >
                                                                                                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"></path>
                                                                                                        </svg>
                                                                                                    </button>
                                                                                                    {
                                                                                                        let file_check_clone = file_clone.clone();
                                                                                                        let file_stream_clone = file_download_clone.clone();
                                                                                                        let file_stream_file_clone = file_clone.clone();
                                                                                                        view! {
                                                                                                            <Show when=move || {
                                                                                                                // Only show stream button for video files
                                                                                                                is_video_file(&file_check_clone) && !file_check_clone.zipped.unwrap_or(false) && !file_check_clone.infected.unwrap_or(false)
                                                                                                            }>
                                                                                                                <button
                                                                                                                    class="p-2 rounded transition-colors flex items-center justify-center"
                                                                                                                    style="background-color: transparent;"
                                                                                                                    on:click=move |_| {
                                                                                                                        let download_id = file_stream_clone.id;
                                                                                                                        let download_type = file_stream_clone.download_type.clone();
                                                                                                                        let file_id = file_stream_file_clone.id;
                                                                                                                        handle_stream(download_id, download_type, Some(file_id));
                                                                                                                    }
                                                                                                                    title="Stream File"
                                                                                                                >
                                                                                                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--accent-primary);">
                                                                                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"></path>
                                                                                                                    </svg>
                                                                                                                </button>
                                                                                                            </Show>
                                                                                                        }
                                                                                                    }
                                                                                                </div>
                                                                                            </td>
                                                                                        </tr>
                                                                                    }
                                                                                }
                                                                            </For>
                                                                        </tbody>
                                                                            </table>
                                                                        </div>
                                                                    </div>
                                                                </td>
                                                            </tr>
                                                        }
                                                    }
                                                </Show>
                                            </>
                                        }
                                    }
                                </For>
                            </tbody>
                        </table>
                    </div>
                </div>
            </Show>
        </div>
    }
}
