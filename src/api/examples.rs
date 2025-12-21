use crate::api::*;
use crate::api::types::*;

pub async fn complete_api_workflow_example(api_key: String) -> Result<(), ApiError> {
 println!("Starting complete TorBox API workflow demonstration...");
 
 let mut handler = create_handler(api_key);
 
 println!("Testing API connection...");
 match handler.test_connection().await {
 Ok(true) => println!("API connection successful!"),
 Ok(false) => {
 println!("API key is invalid!");
 return Err(ApiError::AuthenticationError);
 },
 Err(e) => {
 println!("Connection failed: {}", e);
 return Err(e);
 }
 }

 println!("\n USER MANAGEMENT EXAMPLES");
 
 let user = handler.get_user_info(true).await?;
 println!("User info retrieved: {}", user.email);
 
 let subscriptions = handler.get_subscriptions().await?;
 println!(" Subscriptions retrieved");
 
 let transactions = handler.get_transactions().await?;
 println!(" Transactions retrieved");
 
 let referral_data = handler.get_referral_data().await?;
 println!(" Referral data retrieved");

 println!("\n SEARCH API EXAMPLES");
 
 let search_results = handler.search_metadata("Star Wars".to_string()).await?;
 println!(" Found {} search results", search_results.len());
 
 if let Some(result) = search_results.first() {
 let metadata = handler.get_metadata("imdb".to_string(), result.id.clone()).await?;
 println!(" Metadata retrieved: {}", metadata.title);
 }
 
 let torrent_search = handler.search_torrents("linux".to_string()).await?;
 println!(" Found {} torrents", torrent_search.len());
 
 let usenet_search = handler.search_usenet("software".to_string()).await?;
 println!(" Found {} usenet posts", usenet_search.len());

 println!("\n TORRENT MANAGEMENT EXAMPLES");
 
 let torrents = handler.get_torrent_list(None, None, Some(0), Some(10)).await?;
 println!(" Retrieved {} torrents", torrents.len());
 
 let queued_torrents = handler.get_queued_torrents().await?;
 println!(" Retrieved {} queued torrents", queued_torrents.len());
 
 /*
 let create_request = CreateTorrentRequest {
 file: None,
 magnet: Some("magnet:?xt=urn:btih:example".to_string()),
 seed: Some(1),
 allow_zip: Some(false),
 name: Some("Example Torrent".to_string()),
 as_queued: Some(false),
 add_only_if_cached: Some(false),
 };
 let new_torrent = handler.create_torrent(create_request).await?;
 println!(" Torrent created: {:?}", new_torrent);
 */

 println!("\n WEB DOWNLOAD EXAMPLES");
 
 let web_downloads = handler.get_web_download_list(None, None, Some(0), Some(10)).await?;
 println!(" Retrieved {} web downloads", web_downloads.len());
 
 /*
 let web_request = CreateWebDownloadRequest {
 link: "https://example.com/file.zip".to_string(),
 password: None,
 name: Some("Example Download".to_string()),
 as_queued: Some(false),
 add_only_if_cached: Some(false),
 };
 let new_web_download = handler.create_web_download(web_request).await?;
 println!(" Web download created: {:?}", new_web_download);
 */

 println!("\n USENET DOWNLOAD EXAMPLES");
 
 let usenet_downloads = handler.get_usenet_download_list(None, None, Some(0), Some(10)).await?;
 println!(" Retrieved {} usenet downloads", usenet_downloads.len());

 println!("\n RSS FEED EXAMPLES");
 
 let rss_feeds = handler.get_rss_feeds(None).await?;
 println!(" Retrieved {} RSS feeds", rss_feeds.len());
 
 /*
 let rss_request = CreateRssFeedRequest {
 url: "https://example.com/rss".to_string(),
 name: "Example RSS Feed".to_string(),
 do_regex: ".*".to_string(),
 dont_regex: "".to_string(),
 dont_older_than: 7,
 pass_check: false,
 scan_interval: vec![60],
 rss_type: vec!["torrent".to_string()],
 torrent_seeding: 1,
 };
 let new_rss_feed = handler.add_rss_feed(rss_request).await?;
 println!(" RSS feed added: {:?}", new_rss_feed);
 */

 println!("\n STREAMING EXAMPLES");
 
 /*
 let stream_request = CreateStreamRequest {
 id: 123,
 file_id: Some(1),
 r#type: Some("torrent".to_string()),
 chosen_subtitle_index: Some(0),
 chosen_audio_index: Some(0),
 };
 let stream = handler.create_stream(stream_request).await?;
 println!(" Stream created: {}", stream.stream_url);
 */

 println!("\n NOTIFICATION EXAMPLES");
 
 let notifications = handler.get_notifications().await?;
 println!(" Retrieved {} notifications", notifications.len());
 
 /*
 let test_result = handler.test_notification().await?;
 println!(" Test notification sent: {}", test_result);
 */

 println!("\n RELAY API EXAMPLES");
 
 let relay_status = handler.get_relay_status().await?;
 println!(" Relay status: {} users online", relay_status.data.current_online);

 println!("\n INTEGRATION EXAMPLES");
 
 let transfer_jobs = handler.get_transfer_jobs().await?;
 println!(" Retrieved {} transfer jobs", transfer_jobs.len());

 println!("\n Complete API workflow demonstration finished successfully!");
 Ok(())
}

pub async fn torrent_management_example(api_key: String) -> Result<(), ApiError> {
 println!(" Torrent Management Example");
 
 let handler = create_handler(api_key);
 
 let torrents = handler.get_torrent_list(None, None, Some(0), Some(100)).await?;
 println!("Found {} torrents", torrents.len());
 
 for torrent in &torrents {
 println!("Torrent: {} - Status: {} - Progress: {:.1}%", 
  torrent.name, torrent.download_state, torrent.progress);
 
 match torrent.download_state.as_str() {
 "completed" => {
 println!(" → Torrent completed, ready for download");
 },
 "downloading" => {
 println!(" → Downloading at {} KB/s", torrent.download_speed / 1024);
 },
 "seeding" => {
 println!(" → Seeding at {} KB/s", torrent.upload_speed / 1024);
 },
 "paused" => {
 println!(" → Torrent is paused");
 },
 _ => {
 println!(" → Status: {}", torrent.download_state);
 }
 }
 }
 
 Ok(())
}

pub async fn search_and_download_example(api_key: String, search_query: String) -> Result<(), ApiError> {
 println!(" Search and Download Example");
 
 let handler = create_handler(api_key);
 
 let search_results = handler.search_metadata(search_query.clone()).await?;
 println!("Found {} results for '{}'", search_results.len(), search_query);
 
 if let Some(result) = search_results.first() {
 println!("Selected: {}", result.title);
 
 let torrent_results = handler.search_torrents(result.title.clone()).await?;
 println!("Found {} torrents for {}", torrent_results.len(), result.title);
 
 if let Some(torrent) = torrent_results.first() {
     println!("Selected torrent: {} ({} MB)", torrent.title, torrent.size / 1024 / 1024);
 
 /*
 let create_request = CreateTorrentRequest {
     file: None,
     magnet: torrent.magnet.clone(),
     seed: Some(1),
     allow_zip: Some(false),
     name: Some(torrent.title.clone()),
     as_queued: Some(false),
     add_only_if_cached: Some(false),
 };
 
 let new_torrent = handler.create_torrent(create_request).await?;
 println!(" Torrent added to queue: {:?}", new_torrent);
 */
 }
 }
 
 Ok(())
}

pub async fn rss_automation_example(api_key: String) -> Result<(), ApiError> {
 println!(" RSS Automation Example");
 
 let handler = create_handler(api_key);
 
 let rss_feeds = handler.get_rss_feeds(None).await?;
 println!("Found {} RSS feeds", rss_feeds.len());
 
 for feed in &rss_feeds {
 println!("RSS Feed: {} - URL: {}", feed.name, feed.url);
 println!(" → Type: {:?}", feed.rss_type);
 println!(" → Scan interval: {:?} minutes", feed.scan_interval);
 println!(" → Regex filters: do='{}', dont='{}'", 
 feed.do_regex.as_deref().unwrap_or("none"), 
 feed.dont_regex.as_deref().unwrap_or("none"));
 }
 
 Ok(())
}

pub async fn cloud_integration_example(api_key: String) -> Result<(), ApiError> {
 println!(" Cloud Integration Example");
 
 let handler = create_handler(api_key);
 
 let transfer_jobs = handler.get_transfer_jobs().await?;
 println!("Found {} active transfer jobs", transfer_jobs.len());
 
 for job in &transfer_jobs {
 println!("Transfer Job #{}: {} → {} ({}%)", 
 job.job_id, job.source, job.destination, job.progress);
 println!(" → Status: {}", job.status);
 println!(" → Created: {}", job.created_at);
 }
 
 Ok(())
}

pub async fn notification_management_example(api_key: String) -> Result<(), ApiError> {
 println!(" Notification Management Example");
 
 let handler = create_handler(api_key);
 
 let notifications = handler.get_notifications().await?;
 println!("Found {} notifications", notifications.len());
 
 let unread_count = notifications.iter().filter(|n| !n.read).count();
 println!("Unread notifications: {}", unread_count);
 
 for notification in &notifications {
 let status = if notification.read { "read" } else { "unread" };
 println!("[{}] {}: {}", status, notification.r#type, notification.title);
 println!(" → {}", notification.message);
  println!(" → {}", notification.created_at);
 }
 
 /*
 if unread_count > 10 {
 let result = handler.clear_all_notifications().await?;
 println!(" Cleared all notifications: {}", result);
 }
 */
 
 Ok(())
}

pub async fn dashboard_data_example(api_key: String) -> Result<DashboardData, ApiError> {
 println!(" Dashboard Data Example");
 
 let handler = create_handler(api_key);
 
 let user = handler.get_user_info(true).await?;
 let torrents = handler.get_torrent_list(None, None, Some(0), Some(10)).await?;
 let web_downloads = handler.get_web_download_list(None, None, Some(0), Some(10)).await?;
 let usenet_downloads = handler.get_usenet_download_list(None, None, Some(0), Some(10)).await?;
 let rss_feeds = handler.get_rss_feeds(None).await?;
 let notifications = handler.get_notifications().await?;
 let relay_status = handler.get_relay_status().await?;
 let transfer_jobs = handler.get_transfer_jobs().await?;
 
 let total_torrents = torrents.len() as i32;
 let total_web_downloads = web_downloads.len() as i32;
 let total_usenet_downloads = usenet_downloads.len() as i32;
 
 let dashboard = DashboardData {
 user,
 torrents: torrents,
 web_downloads: web_downloads,
 usenet_downloads: usenet_downloads,
 rss_feeds,
 notifications,
 relay_status,
 transfer_jobs,
 total_torrents,
 total_web_downloads,
 total_usenet_downloads,
 };
 
 println!(" Dashboard data compiled successfully!");
 Ok(dashboard)
}

#[derive(Debug, Clone)]
pub struct DashboardData {
 pub user: User,
 pub torrents: Vec<Torrent>,
 pub web_downloads: Vec<WebDownload>,
 pub usenet_downloads: Vec<UsenetDownload>,
 pub rss_feeds: Vec<RssFeed>,
 pub notifications: Vec<Notification>,
 pub relay_status: RelayStatus,
 pub transfer_jobs: Vec<TransferJob>,
 pub total_torrents: i32,
 pub total_web_downloads: i32,
 pub total_usenet_downloads: i32,
}
