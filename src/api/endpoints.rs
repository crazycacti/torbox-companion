pub mod main_api {
    pub const USER_ME: &str = "/v1/api/user/me";
    pub const USER_REFRESH_TOKEN: &str = "/v1/api/user/refreshtoken";
    pub const USER_GET_CONFIRMATION: &str = "/v1/api/user/getconfirmation";
    pub const USER_ADD_REFERRAL: &str = "/v1/api/user/addreferral";
    pub const USER_REFERRAL_DATA: &str = "/v1/api/user/referraldata";
    pub const USER_SUBSCRIPTIONS: &str = "/v1/api/user/subscriptions";
    pub const USER_TRANSACTIONS: &str = "/v1/api/user/transactions";
    pub const USER_DELETE_ME: &str = "/v1/api/user/deleteme";
    
    pub const DEVICE_AUTH_START: &str = "/v1/api/user/auth/device/start";
    pub const DEVICE_AUTH_TOKEN: &str = "/v1/api/user/auth/device/token";
    
    pub const SEARCH_ENGINES_ADD: &str = "/v1/api/user/settings/addsearchengines";
    pub const SEARCH_ENGINES_GET: &str = "/v1/api/user/settings/searchengines";
    pub const SEARCH_ENGINES_MODIFY: &str = "/v1/api/user/settings/modifysearchengines";
    pub const SEARCH_ENGINES_CONTROL: &str = "/v1/api/user/settings/controlsearchengines";
    
    pub const TORRENTS_CREATE: &str = "/v1/api/torrents/createtorrent";
    pub const TORRENTS_ASYNC_CREATE: &str = "/v1/api/torrents/asynccreatetorrent";
    pub const TORRENTS_CONTROL: &str = "/v1/api/torrents/controltorrent";
    pub const TORRENTS_LIST: &str = "/v1/api/torrents/mylist";
    pub const TORRENTS_QUEUED: &str = "/v1/api/torrents/getqueued";
    pub const TORRENTS_CONTROL_QUEUED: &str = "/v1/api/torrents/controlqueued";
    pub const TORRENTS_REQUEST_DL: &str = "/v1/api/torrents/requestdl";
    pub const TORRENTS_CHECK_CACHED: &str = "/v1/api/torrents/checkcached";
    pub const TORRENTS_EXPORT_DATA: &str = "/v1/api/torrents/exportdata";
    pub const TORRENTS_MAGNET_TO_FILE: &str = "/v1/api/torrents/magnettofile";
    pub const TORRENTS_INFO: &str = "/v1/api/torrents/torrentinfo";
    
    pub const WEB_DL_CREATE: &str = "/v1/api/webdl/createwebdownload";
    pub const WEB_DL_ASYNC_CREATE: &str = "/v1/api/webdl/asynccreatewebdownload";
    pub const WEB_DL_CONTROL: &str = "/v1/api/webdl/controlwebdownload";
    pub const WEB_DL_LIST: &str = "/v1/api/webdl/mylist";
    pub const WEB_DL_REQUEST_DL: &str = "/v1/api/webdl/requestdl";
    pub const WEB_DL_CHECK_CACHED: &str = "/v1/api/webdl/checkcached";
    pub const WEB_DL_HOSTERS: &str = "/v1/api/webdl/hosters";
    
    pub const USENET_CREATE: &str = "/v1/api/usenet/createusenetdownload";
    pub const USENET_ASYNC_CREATE: &str = "/v1/api/usenet/asynccreateusenetdownload";
    pub const USENET_CONTROL: &str = "/v1/api/usenet/controlusenetdownload";
    pub const USENET_LIST: &str = "/v1/api/usenet/mylist";
    pub const USENET_REQUEST_DL: &str = "/v1/api/usenet/requestdl";
    pub const USENET_CHECK_CACHED: &str = "/v1/api/usenet/checkcached";
    
    pub const RSS_ADD: &str = "/v1/api/rss/addrss";
    pub const RSS_CONTROL: &str = "/v1/api/rss/controlrss";
    pub const RSS_MODIFY: &str = "/v1/api/rss/modifyrss";
    pub const RSS_GET_FEEDS: &str = "/v1/api/rss/getfeeds";
    pub const RSS_GET_FEED_ITEMS: &str = "/v1/api/rss/getfeeditems";
    
    pub const STREAM_CREATE: &str = "/v1/api/stream/createstream";
    pub const STREAM_GET_DATA: &str = "/v1/api/stream/getstreamdata";
    
    pub const NOTIFICATIONS_RSS: &str = "/v1/api/notifications/rss";
    pub const NOTIFICATIONS_MY: &str = "/v1/api/notifications/mynotifications";
    pub const NOTIFICATIONS_CLEAR: &str = "/v1/api/notifications/clear";
    pub const NOTIFICATIONS_CLEAR_BY_ID: &str = "/v1/api/notifications/clear/{id}";
    pub const NOTIFICATIONS_TEST: &str = "/v1/api/notifications/test";
    
    pub const INTEGRATION_OAUTH_REDIRECT: &str = "/v1/api/integration/oauth/{provider}";
    pub const INTEGRATION_OAUTH_CALLBACK: &str = "/v1/api/integration/oauth/{provider}/callback";
    pub const INTEGRATION_OAUTH_SUCCESS: &str = "/v1/api/integration/oauth/{provider}/success";
    pub const INTEGRATION_GOOGLE_DRIVE: &str = "/v1/api/integration/googledrive";
    pub const INTEGRATION_DROPBOX: &str = "/v1/api/integration/dropbox";
    pub const INTEGRATION_ONEDRIVE: &str = "/v1/api/integration/onedrive";
    pub const INTEGRATION_GOFILE: &str = "/v1/api/integration/gofile";
    pub const INTEGRATION_1FICHIER: &str = "/v1/api/integration/1fichier";
    pub const INTEGRATION_PIXELDRAIN: &str = "/v1/api/integration/pixeldrain";
    pub const INTEGRATION_JOBS: &str = "/v1/api/integration/jobs";
    pub const INTEGRATION_JOB_BY_HASH: &str = "/v1/api/integration/jobs/{hash}";
    pub const INTEGRATION_CANCEL_JOB: &str = "/v1/api/integration/job/{job_id}";
    
    pub const VENDORS_REGISTER: &str = "/v1/api/vendors/register";
    pub const VENDORS_ACCOUNT: &str = "/v1/api/vendors/account";
    pub const VENDORS_UPDATE_ACCOUNT: &str = "/v1/api/vendors/updateaccount";
    pub const VENDORS_GET_ACCOUNTS: &str = "/v1/api/vendors/getaccounts";
    pub const VENDORS_GET_ACCOUNT: &str = "/v1/api/vendors/getaccount";
    pub const VENDORS_REGISTER_USER: &str = "/v1/api/vendors/registeruser";
    pub const VENDORS_REMOVE_USER: &str = "/v1/api/vendors/removeuser";
    
    pub const QUEUED_GET: &str = "/v1/api/queued/getqueued";
    pub const QUEUED_CONTROL: &str = "/v1/api/queued/controlqueued";
    
    pub const STATUS: &str = "/";
    pub const STATS: &str = "/v1/api/stats";
    pub const STATS_30_DAYS: &str = "/v1/api/stats/30days";
    pub const INTERCOM_HASH: &str = "/v1/api/intercom/hash";
    pub const CHANGELOGS_RSS: &str = "/v1/api/changelogs/rss";
    pub const CHANGELOGS_JSON: &str = "/v1/api/changelogs/json";
    pub const SPEEDTEST: &str = "/v1/api/speedtest";
    pub const TRANSACTION_PDF: &str = "/v1/api/user/transaction/pdf";
}

pub mod search_api {
    pub const METADATA: &str = "/meta/{id_type}:{id}";
    pub const TORRENTS_BY_ID: &str = "/torrents/{id_type}:{id}";
    pub const TORRENTS_SEARCH: &str = "/torrents/search/{search_query}";
    pub const USENET_BY_ID: &str = "/usenet/{id_type}:{id}";
    pub const USENET_SEARCH: &str = "/usenet/search/{search_query}";
    pub const SEARCH_METADATA: &str = "/search/{search_query}";
}

pub mod relay_api {
    pub const STATUS: &str = "/";
    pub const TORRENT_UPDATE: &str = "/v1/inactivecheck/torrent/{user_id}/{torrent_id}";
}

pub mod stream_api {
    pub const CREATE_STREAM: &str = "/createstream";
    pub const GET_STREAM_DATA: &str = "/getstreamdata";
}

pub const MAIN_API_BASE: &str = "https://api.torbox.app";
pub const SEARCH_API_BASE: &str = "https://search-api.torbox.app";
pub const RELAY_API_BASE: &str = "https://relay.torbox.app";
pub const STREAM_API_BASE: &str = "/api/stream";

pub const RATE_LIMIT_GENERAL: u32 = 5;
pub const RATE_LIMIT_TORRENT_CREATE: u32 = 60;
pub const RATE_LIMIT_USENET_CREATE: u32 = 60;
pub const RATE_LIMIT_WEB_DL_CREATE: u32 = 60;
pub const RATE_LIMIT_SEARCH_API: u32 = 300;

pub const STATUS_OK: u16 = 200;
pub const STATUS_BAD_REQUEST: u16 = 400;
pub const STATUS_UNAUTHORIZED: u16 = 401;
pub const STATUS_NOT_FOUND: u16 = 404;
pub const STATUS_UNPROCESSABLE_ENTITY: u16 = 422;
pub const STATUS_TOO_MANY_REQUESTS: u16 = 429;
pub const STATUS_INTERNAL_SERVER_ERROR: u16 = 500;

pub const PARAM_SETTINGS: &str = "settings";
pub const PARAM_ID: &str = "id";
pub const PARAM_OFFSET: &str = "offset";
pub const PARAM_LIMIT: &str = "limit";
pub const PARAM_BYPASS_CACHE: &str = "bypass_cache";
pub const PARAM_TOKEN: &str = "token";
pub const PARAM_FILE_ID: &str = "file_id";
pub const PARAM_ZIP_LINK: &str = "zip_link";
pub const PARAM_USER_IP: &str = "user_ip";
pub const PARAM_REDIRECT: &str = "redirect";
pub const PARAM_TYPE: &str = "type";
pub const PARAM_CHOSEN_SUBTITLE_INDEX: &str = "chosen_subtitle_index";
pub const PARAM_CHOSEN_AUDIO_INDEX: &str = "chosen_audio_index";

pub const FIELD_OPERATION: &str = "operation";
pub const FIELD_ALL: &str = "all";
pub const FIELD_NAME: &str = "name";
pub const FIELD_URL: &str = "url";
pub const FIELD_LINK: &str = "link";
pub const FIELD_MAGNET: &str = "magnet";
pub const FIELD_FILE: &str = "file";
pub const FIELD_PASSWORD: &str = "password";
pub const FIELD_AS_QUEUED: &str = "as_queued";
pub const FIELD_ADD_ONLY_IF_CACHED: &str = "add_only_if_cached";
