use crate::automation::encryption::EncryptionService;
use crate::automation::types::*;
use rusqlite::{params, Connection, Result as SqliteResult};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Database {
    conn: Arc<Mutex<Connection>>,
    encryption: Arc<EncryptionService>,
}

impl Database {
    pub async fn new<P: AsRef<Path>>(db_path: P) -> Result<Self, String> {
        let db_dir = db_path.as_ref().parent().ok_or("Invalid database path")?;
        std::fs::create_dir_all(db_dir).map_err(|e| format!("Failed to create data directory: {}", e))?;

        let db_path_ref = db_path.as_ref();
        let conn = Connection::open(db_path_ref)
            .map_err(|e| format!("Failed to open database: {}", e))?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = std::fs::metadata(db_path_ref) {
                let mut perms = metadata.permissions();
                perms.set_mode(0o600);
                std::fs::set_permissions(db_path_ref, perms)
                    .map_err(|e| format!("Failed to set database permissions: {}", e))?;
            }
        }

        let _: String = conn
            .query_row("PRAGMA journal_mode = WAL", [], |row| row.get(0))
            .map_err(|e| format!("Failed to set WAL mode: {}", e))?;

        conn.execute("PRAGMA foreign_keys = ON", [])
            .map_err(|e| format!("Failed to enable foreign keys: {}", e))?;

        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
            encryption: Arc::new(EncryptionService::new()),
        };

        db.create_tables().await?;
        db.initialize_encryption_key().await?;

        Ok(db)
    }

    async fn create_tables(&self) -> Result<(), String> {
        let conn = self.conn.lock().await;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS server_key (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                encryption_key BLOB NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )
        .map_err(|e| format!("Failed to create server_key table: {}", e))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS api_keys (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                api_key_hash TEXT UNIQUE NOT NULL,
                encrypted_api_key BLOB NOT NULL,
                nonce BLOB NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                last_used_at DATETIME
            )",
            [],
        )
        .map_err(|e| format!("Failed to create api_keys table: {}", e))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS automation_rules (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                api_key_hash TEXT NOT NULL,
                name TEXT NOT NULL,
                enabled BOOLEAN DEFAULT true,
                trigger_config TEXT NOT NULL,
                conditions TEXT NOT NULL,
                action_config TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (api_key_hash) REFERENCES api_keys(api_key_hash)
            )",
            [],
        )
        .map_err(|e| format!("Failed to create automation_rules table: {}", e))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS rule_execution_log (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                rule_id INTEGER NOT NULL,
                rule_name TEXT NOT NULL,
                api_key_hash TEXT NOT NULL,
                execution_type TEXT NOT NULL,
                items_processed INTEGER DEFAULT 0,
                success BOOLEAN DEFAULT true,
                error_message TEXT,
                processed_items TEXT,
                executed_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (rule_id) REFERENCES automation_rules(id)
            )",
            [],
        )
        .map_err(|e| format!("Failed to create rule_execution_log table: {}", e))?;

        let processed_items_column_exists: bool = conn
            .query_row(
                "SELECT COUNT(*) FROM pragma_table_info('rule_execution_log') WHERE name = 'processed_items'",
                [],
                |row| Ok(row.get::<_, i64>(0)? > 0),
            )
            .unwrap_or(false);

        if !processed_items_column_exists {
            conn.execute(
                "ALTER TABLE rule_execution_log ADD COLUMN processed_items TEXT",
                [],
            )
            .map_err(|e| format!("Failed to add processed_items column: {}", e))?;
        }

        let total_items_column_exists: bool = conn
            .query_row(
                "SELECT COUNT(*) FROM pragma_table_info('rule_execution_log') WHERE name = 'total_items'",
                [],
                |row| Ok(row.get::<_, i64>(0)? > 0),
            )
            .unwrap_or(false);

        if !total_items_column_exists {
            let _ = conn.execute(
                "ALTER TABLE rule_execution_log ADD COLUMN total_items INTEGER",
                [],
            );
        }

        let partial_column_exists: bool = conn
            .query_row(
                "SELECT COUNT(*) FROM pragma_table_info('rule_execution_log') WHERE name = 'partial'",
                [],
                |row| Ok(row.get::<_, i64>(0)? > 0),
            )
            .unwrap_or(false);

        if !partial_column_exists {
            let _ = conn.execute(
                "ALTER TABLE rule_execution_log ADD COLUMN partial BOOLEAN",
                [],
            );
        }

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_rules_api_key_hash ON automation_rules(api_key_hash)",
            [],
        )
        .map_err(|e| format!("Failed to create index: {}", e))?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_rules_enabled ON automation_rules(enabled)",
            [],
        )
        .map_err(|e| format!("Failed to create index: {}", e))?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_logs_rule_id ON rule_execution_log(rule_id)",
            [],
        )
        .map_err(|e| format!("Failed to create index: {}", e))?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_logs_api_key_hash ON rule_execution_log(api_key_hash)",
            [],
        )
        .map_err(|e| format!("Failed to create index: {}", e))?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_logs_executed_at ON rule_execution_log(executed_at)",
            [],
        )
        .map_err(|e| format!("Failed to create index: {}", e))?;

        Ok(())
    }

    async fn initialize_encryption_key(&self) -> Result<(), String> {
        let conn = self.conn.lock().await;

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM server_key", [], |row| row.get(0))
            .map_err(|e| format!("Failed to check server_key: {}", e))?;

        if count == 0 {
            let key = EncryptionService::generate_key().await;
            conn.execute(
                "INSERT INTO server_key (id, encryption_key) VALUES (1, ?)",
                params![key.as_slice()],
            )
            .map_err(|e| format!("Failed to insert server key: {}", e))?;

            self.encryption.initialize(key).await;
        } else {
            let key: Vec<u8> = conn
                .query_row("SELECT encryption_key FROM server_key WHERE id = 1", [], |row| {
                    row.get(0)
                })
                .map_err(|e| format!("Failed to get server key: {}", e))?;

            if key.len() != 32 {
                return Err("Invalid server key length".to_string());
            }

            let mut key_array = [0u8; 32];
            key_array.copy_from_slice(&key);
            self.encryption.initialize(key_array).await;
        }

        Ok(())
    }

    pub async fn get_encryption_service(&self) -> Arc<EncryptionService> {
        self.encryption.clone()
    }

    pub async fn save_api_key(&self, api_key: &str) -> Result<String, String> {
        let hash = EncryptionService::hash_api_key(api_key);
        let (encrypted, nonce) = self.encryption.encrypt_api_key(api_key).await?;

        let conn = self.conn.lock().await;

        conn.execute(
            "INSERT OR REPLACE INTO api_keys (api_key_hash, encrypted_api_key, nonce, last_used_at)
             VALUES (?, ?, ?, CURRENT_TIMESTAMP)",
            params![hash, encrypted.as_slice(), nonce.as_slice()],
        )
        .map_err(|e| format!("Failed to save API key: {}", e))?;

        Ok(hash)
    }

    pub async fn get_api_key(&self, api_key_hash: &str) -> Result<String, String> {
        let conn = self.conn.lock().await;

        let (encrypted, nonce): (Vec<u8>, Vec<u8>) = conn
            .query_row(
                "SELECT encrypted_api_key, nonce FROM api_keys WHERE api_key_hash = ?",
                params![api_key_hash],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .map_err(|e| format!("Failed to get API key: {}", e))?;

        conn.execute(
            "UPDATE api_keys SET last_used_at = CURRENT_TIMESTAMP WHERE api_key_hash = ?",
            params![api_key_hash],
        )
        .map_err(|e| format!("Failed to update last_used_at: {}", e))?;

        self.encryption.decrypt_api_key(&encrypted, &nonce).await
    }

    pub async fn get_all_api_key_hashes(&self) -> Result<Vec<String>, String> {
        let conn = self.conn.lock().await;

        let mut stmt = conn
            .prepare("SELECT api_key_hash FROM api_keys")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let hashes = stmt
            .query_map([], |row| Ok(row.get::<_, String>(0)?))
            .map_err(|e| format!("Failed to query API keys: {}", e))?
            .collect::<Result<Vec<String>, _>>()
            .map_err(|e| format!("Failed to collect API keys: {}", e))?;

        Ok(hashes)
    }

    pub async fn save_rule(&self, rule: &AutomationRule) -> Result<i64, String> {
        let conn = self.conn.lock().await;

        let trigger_json = serde_json::to_string(&rule.trigger_config)
            .map_err(|e| format!("Failed to serialize trigger_config: {}", e))?;
        let conditions_json = serde_json::to_string(&rule.conditions)
            .map_err(|e| format!("Failed to serialize conditions: {}", e))?;
        let action_json = serde_json::to_string(&rule.action_config)
            .map_err(|e| format!("Failed to serialize action_config: {}", e))?;

        if let Some(id) = rule.id {
            conn.execute(
                "UPDATE automation_rules 
                 SET name = ?, enabled = ?, trigger_config = ?, conditions = ?, action_config = ?, updated_at = CURRENT_TIMESTAMP
                 WHERE id = ? AND api_key_hash = ?",
                params![rule.name, rule.enabled, trigger_json, conditions_json, action_json, id, rule.api_key_hash],
            )
            .map_err(|e| format!("Failed to update rule: {}", e))?;
            Ok(id)
        } else {
            conn.execute(
                "INSERT INTO automation_rules (api_key_hash, name, enabled, trigger_config, conditions, action_config)
                 VALUES (?, ?, ?, ?, ?, ?)",
                params![rule.api_key_hash, rule.name, rule.enabled, trigger_json, conditions_json, action_json],
            )
            .map_err(|e| format!("Failed to insert rule: {}", e))?;
            Ok(conn.last_insert_rowid())
        }
    }

    pub async fn count_rules_by_api_key(&self, api_key_hash: &str) -> Result<i64, String> {
        let conn = self.conn.lock().await;

        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM automation_rules WHERE api_key_hash = ?",
                params![api_key_hash],
                |row| row.get(0),
            )
            .map_err(|e| format!("Failed to count rules: {}", e))?;

        Ok(count)
    }

    pub async fn get_rules_by_api_key(&self, api_key_hash: &str) -> Result<Vec<AutomationRule>, String> {
        let conn = self.conn.lock().await;

        let mut stmt = conn
            .prepare(
                "SELECT id, api_key_hash, name, enabled, trigger_config, conditions, action_config, created_at, updated_at
                 FROM automation_rules WHERE api_key_hash = ? ORDER BY created_at DESC",
            )
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let rules = stmt
            .query_map(params![api_key_hash], |row| {
                let trigger_json: String = row.get(4)?;
                let conditions_json: String = row.get(5)?;
                let action_json: String = row.get(6)?;

                Ok(AutomationRule {
                    id: Some(row.get(0)?),
                    api_key_hash: row.get(1)?,
                    name: row.get(2)?,
                    enabled: row.get(3)?,
                    trigger_config: serde_json::from_str(&trigger_json)
                        .map_err(|_| rusqlite::Error::InvalidColumnType(4, "trigger_config".to_string(), rusqlite::types::Type::Text))?,
                    conditions: serde_json::from_str(&conditions_json)
                        .map_err(|_| rusqlite::Error::InvalidColumnType(5, "conditions".to_string(), rusqlite::types::Type::Text))?,
                    action_config: serde_json::from_str(&action_json)
                        .map_err(|_| rusqlite::Error::InvalidColumnType(6, "action_config".to_string(), rusqlite::types::Type::Text))?,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                })
            })
            .map_err(|e| format!("Failed to query rules: {}", e))?
            .collect::<Result<Vec<AutomationRule>, _>>()
            .map_err(|e| format!("Failed to collect rules: {}", e))?;

        Ok(rules)
    }

    pub async fn get_all_enabled_rules(&self) -> Result<Vec<AutomationRule>, String> {
        let conn = self.conn.lock().await;

        let mut stmt = conn
            .prepare(
                "SELECT id, api_key_hash, name, enabled, trigger_config, conditions, action_config, created_at, updated_at
                 FROM automation_rules WHERE enabled = true",
            )
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let rules = stmt
            .query_map([], |row| {
                let trigger_json: String = row.get(4)?;
                let conditions_json: String = row.get(5)?;
                let action_json: String = row.get(6)?;

                Ok(AutomationRule {
                    id: Some(row.get(0)?),
                    api_key_hash: row.get(1)?,
                    name: row.get(2)?,
                    enabled: row.get(3)?,
                    trigger_config: serde_json::from_str(&trigger_json)
                        .map_err(|_| rusqlite::Error::InvalidColumnType(4, "trigger_config".to_string(), rusqlite::types::Type::Text))?,
                    conditions: serde_json::from_str(&conditions_json)
                        .map_err(|_| rusqlite::Error::InvalidColumnType(5, "conditions".to_string(), rusqlite::types::Type::Text))?,
                    action_config: serde_json::from_str(&action_json)
                        .map_err(|_| rusqlite::Error::InvalidColumnType(6, "action_config".to_string(), rusqlite::types::Type::Text))?,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                })
            })
            .map_err(|e| format!("Failed to query rules: {}", e))?
            .collect::<Result<Vec<AutomationRule>, _>>()
            .map_err(|e| format!("Failed to collect rules: {}", e))?;

        Ok(rules)
    }

    pub async fn get_rule_by_id(&self, rule_id: i64, api_key_hash: &str) -> Result<Option<AutomationRule>, String> {
        let conn = self.conn.lock().await;

        let result = conn
            .query_row(
                "SELECT id, api_key_hash, name, enabled, trigger_config, conditions, action_config, created_at, updated_at
                 FROM automation_rules WHERE id = ? AND api_key_hash = ?",
                params![rule_id, api_key_hash],
                |row| {
                    let trigger_json: String = row.get(4)?;
                    let conditions_json: String = row.get(5)?;
                    let action_json: String = row.get(6)?;

                    Ok(AutomationRule {
                        id: Some(row.get(0)?),
                        api_key_hash: row.get(1)?,
                        name: row.get(2)?,
                        enabled: row.get(3)?,
                        trigger_config: serde_json::from_str(&trigger_json)
                            .map_err(|_| rusqlite::Error::InvalidColumnType(4, "trigger_config".to_string(), rusqlite::types::Type::Text))?,
                        conditions: serde_json::from_str(&conditions_json)
                            .map_err(|_| rusqlite::Error::InvalidColumnType(5, "conditions".to_string(), rusqlite::types::Type::Text))?,
                        action_config: serde_json::from_str(&action_json)
                            .map_err(|_| rusqlite::Error::InvalidColumnType(6, "action_config".to_string(), rusqlite::types::Type::Text))?,
                        created_at: row.get(7)?,
                        updated_at: row.get(8)?,
                    })
                },
            );

        match result {
            Ok(rule) => Ok(Some(rule)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("Failed to get rule: {}", e)),
        }
    }

    pub async fn delete_rule(&self, rule_id: i64, api_key_hash: &str) -> Result<bool, String> {
        let conn = self.conn.lock().await;

        let rule_exists = conn
            .query_row(
                "SELECT COUNT(*) FROM automation_rules WHERE id = ? AND api_key_hash = ?",
                params![rule_id, api_key_hash],
                |row| row.get::<_, i64>(0),
            )
            .map_err(|e| format!("Failed to check rule existence: {}", e))?;

        if rule_exists == 0 {
            return Ok(false);
        }

        conn.execute(
            "DELETE FROM rule_execution_log WHERE rule_id = ?",
            params![rule_id],
        )
        .map_err(|e| format!("Failed to delete execution logs: {}", e))?;

        let rows_affected = conn
            .execute(
                "DELETE FROM automation_rules WHERE id = ? AND api_key_hash = ?",
                params![rule_id, api_key_hash],
            )
            .map_err(|e| format!("Failed to delete rule: {}", e))?;

        Ok(rows_affected > 0)
    }

    pub async fn log_execution(&self, log: &ExecutionLog) -> Result<(), String> {
        let conn = self.conn.lock().await;

        let processed_items_json = log.processed_items.as_ref()
            .and_then(|items| serde_json::to_string(items).ok());

        conn.execute(
            "INSERT INTO rule_execution_log (rule_id, rule_name, api_key_hash, execution_type, items_processed, total_items, success, error_message, processed_items, partial)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                log.rule_id,
                log.rule_name,
                log.api_key_hash,
                log.execution_type,
                log.items_processed,
                log.total_items,
                log.success,
                log.error_message,
                processed_items_json,
                log.partial
            ],
        )
        .map_err(|e| format!("Failed to log execution: {}", e))?;

        Ok(())
    }

    pub async fn cleanup_old_logs(&self, days_to_keep: i64) -> Result<usize, String> {
        let conn = self.conn.lock().await;

        let rows_affected = conn
            .execute(
                "DELETE FROM rule_execution_log WHERE executed_at < datetime('now', '-' || ? || ' days')",
                params![days_to_keep],
            )
            .map_err(|e| format!("Failed to cleanup old logs: {}", e))?;

        Ok(rows_affected)
    }

    pub async fn get_execution_logs(
        &self,
        rule_id: Option<i64>,
        api_key_hash: &str,
        limit: i32,
    ) -> Result<Vec<ExecutionLog>, String> {
        let conn = self.conn.lock().await;

        let max_limit = limit.min(1000);
        let query = if let Some(_id) = rule_id {
            "SELECT id, rule_id, rule_name, api_key_hash, execution_type, items_processed, total_items, success, error_message, processed_items, executed_at, partial
             FROM rule_execution_log WHERE rule_id = ? AND api_key_hash = ? ORDER BY executed_at DESC LIMIT ?"
        } else {
            "SELECT id, rule_id, rule_name, api_key_hash, execution_type, items_processed, total_items, success, error_message, processed_items, executed_at, partial
             FROM rule_execution_log WHERE api_key_hash = ? ORDER BY executed_at DESC LIMIT ?"
        };

        let mut stmt = conn
            .prepare(query)
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let logs: Result<Vec<ExecutionLog>, _> = if let Some(id) = rule_id {
            stmt.query_map(params![id, api_key_hash, max_limit], |row| {
                let processed_items_json: Option<String> = row.get(9)?;
                let processed_items = processed_items_json
                    .and_then(|json| serde_json::from_str(&json).ok());
                
                Ok(ExecutionLog {
                    id: Some(row.get(0)?),
                    rule_id: row.get(1)?,
                    rule_name: row.get(2)?,
                    api_key_hash: row.get(3)?,
                    execution_type: row.get(4)?,
                    items_processed: row.get(5)?,
                    total_items: row.get(6)?,
                    success: row.get(7)?,
                    error_message: row.get(8)?,
                    processed_items,
                    executed_at: row.get(10)?,
                    partial: row.get(11)?,
                })
            })
            .map_err(|e| format!("Failed to query logs: {}", e))?
            .collect()
        } else {
            stmt.query_map(params![api_key_hash, max_limit], |row| {
                let processed_items_json: Option<String> = row.get(9)?;
                let processed_items = processed_items_json
                    .and_then(|json| serde_json::from_str(&json).ok());
                
                Ok(ExecutionLog {
                    id: Some(row.get(0)?),
                    rule_id: row.get(1)?,
                    rule_name: row.get(2)?,
                    api_key_hash: row.get(3)?,
                    execution_type: row.get(4)?,
                    items_processed: row.get(5)?,
                    total_items: row.get(6)?,
                    success: row.get(7)?,
                    error_message: row.get(8)?,
                    processed_items,
                    executed_at: row.get(10)?,
                    partial: row.get(11)?,
                })
            })
            .map_err(|e| format!("Failed to query logs: {}", e))?
            .collect()
        };

        let logs = logs.map_err(|e| format!("Failed to collect logs: {}", e))?;

        Ok(logs)
    }
}
