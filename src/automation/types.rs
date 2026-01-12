use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationRule {
    pub id: Option<i64>,
    pub api_key_hash: String,
    pub name: String,
    pub enabled: bool,
    pub trigger_config: TriggerConfig,
    pub conditions: Vec<Condition>,
    pub action_config: ActionConfig,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TriggerConfig {
    Cron { expression: String },
    Interval { minutes: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub r#type: ConditionType,
    pub operator: Operator,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionType {
    SeedingTime,
    SeedingRatio,
    StalledTime,
    Age,
    DownloadSpeed,
    UploadSpeed,
    FileSize,
    Progress,
    Seeds,
    Peers,
    TotalUploaded,
    TotalDownloaded,
    DownloadState,
    Inactive,
    DownloadFinished,
    Cached,
    Private,
    LongTermSeeding,
    SeedTorrent,
    ETA,
    Availability,
    ExpiresAt,
    DownloadPresent,
    TorrentFile,
    AllowZipped,
    HasMagnet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operator {
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Equal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionConfig {
    pub action_type: ActionType,
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    StopSeeding,
    Delete,
    Stop,
    Resume,
    Restart,
    Reannounce,
    ForceStart,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedItem {
    pub id: i32,
    pub name: String,
    pub action: String,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionLog {
    pub id: Option<i64>,
    pub rule_id: i64,
    pub rule_name: String,
    pub api_key_hash: String,
    pub execution_type: String,
    pub items_processed: i32,
    #[serde(default)]
    pub total_items: Option<i32>,
    pub success: bool,
    pub error_message: Option<String>,
    pub processed_items: Option<Vec<ProcessedItem>>,
    pub executed_at: Option<String>,
    #[serde(default)]
    pub partial: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct ApiKeyRecord {
    pub id: i64,
    pub api_key_hash: String,
    pub encrypted_api_key: Vec<u8>,
    pub nonce: Vec<u8>,
    pub created_at: String,
    pub last_used_at: Option<String>,
}
