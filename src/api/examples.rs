use crate::api::*;
use crate::api::types::*;

pub async fn complete_api_workflow_example(api_key: String) -> Result<(), ApiError> {
 let handler = create_handler(api_key);
 
 match handler.test_connection().await {
 Ok(true) => {},
 Ok(false) => {
 return Err(ApiError::AuthenticationError);
 },
 Err(e) => {
 return Err(e);
 }
 }

 let _user = handler.get_user_info(true).await?;
 let _subscriptions = handler.get_subscriptions().await?;
 let _transactions = handler.get_transactions().await?;
 let _referral_data = handler.get_referral_data().await?;

 let search_results = handler.search_metadata("Star Wars".to_string()).await?;
 
 if let Some(result) = search_results.first() {
 let _metadata = handler.get_metadata("imdb".to_string(), result.id.clone()).await?;
 }
 
 let _torrent_search = handler.search_torrents("linux".to_string()).await?;
 let _usenet_search = handler.search_usenet("software".to_string()).await?;

 let _torrents = handler.get_torrent_list(None, None, Some(0), Some(10)).await?;
 let _queued_torrents = handler.get_queued_torrents().await?;
 
 let _web_downloads = handler.get_web_download_list(None, None, Some(0), Some(10)).await?;
 
 let _usenet_downloads = handler.get_usenet_download_list(None, None, Some(0), Some(10)).await?;

 let _rss_feeds = handler.get_rss_feeds(None).await?;
 
 let _notifications = handler.get_notifications().await?;
 
 let _relay_status = handler.get_relay_status().await?;

 let _transfer_jobs = handler.get_transfer_jobs().await?;

 Ok(())
}

pub async fn torrent_management_example(api_key: String) -> Result<(), ApiError> {
 let handler = create_handler(api_key);
 
 let _torrents = handler.get_torrent_list(None, None, Some(0), Some(100)).await?;
 
 Ok(())
}

pub async fn search_and_download_example(api_key: String, search_query: String) -> Result<(), ApiError> {
 let handler = create_handler(api_key);
 
 let search_results = handler.search_metadata(search_query.clone()).await?;
 
 if let Some(result) = search_results.first() {
 let _torrent_results = handler.search_torrents(result.title.clone()).await?;
 }
 
 Ok(())
}

pub async fn rss_automation_example(api_key: String) -> Result<(), ApiError> {
 let handler = create_handler(api_key);
 
 let _rss_feeds = handler.get_rss_feeds(None).await?;
 
 Ok(())
}

pub async fn cloud_integration_example(api_key: String) -> Result<(), ApiError> {
 let handler = create_handler(api_key);
 
 let _transfer_jobs = handler.get_transfer_jobs().await?;
 
 Ok(())
}

pub async fn notification_management_example(api_key: String) -> Result<(), ApiError> {
 let handler = create_handler(api_key);
 
 let notifications = handler.get_notifications().await?;
 let _unread_count = notifications.iter().filter(|n| !n.read).count();
 
 Ok(())
}

pub async fn dashboard_data_example(api_key: String) -> Result<DashboardData, ApiError> {
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
