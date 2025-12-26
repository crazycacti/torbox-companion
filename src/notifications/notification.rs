use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NotificationType {
    Success,
    Error,
    Warning,
    Info,
}

impl NotificationType {
    pub fn icon(&self) -> &'static str {
        match self {
            NotificationType::Success => "✓",
            NotificationType::Error => "✕",
            NotificationType::Warning => "⚠",
            NotificationType::Info => "ℹ",
        }
    }

    pub fn bg_color(&self) -> &'static str {
        match self {
            NotificationType::Success => "var(--bg-card)",
            NotificationType::Error => "var(--bg-card)",
            NotificationType::Warning => "var(--bg-card)",
            NotificationType::Info => "var(--bg-card)",
        }
    }

    pub fn border_color(&self) -> &'static str {
        match self {
            NotificationType::Success => "rgba(34, 197, 94, 0.3)",
            NotificationType::Error => "rgba(239, 68, 68, 0.3)",
            NotificationType::Warning => "rgba(234, 179, 8, 0.3)",
            NotificationType::Info => "rgba(59, 130, 246, 0.3)",
        }
    }

    pub fn icon_color(&self) -> &'static str {
        match self {
            NotificationType::Success => "#22c55e",
            NotificationType::Error => "#ef4444",
            NotificationType::Warning => "#eab308",
            NotificationType::Info => "#3b82f6",
        }
    }

    pub fn text_color(&self) -> &'static str {
        match self {
            NotificationType::Success => "var(--text-primary)",
            NotificationType::Error => "var(--text-primary)",
            NotificationType::Warning => "var(--text-primary)",
            NotificationType::Info => "var(--text-primary)",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Notification {
    pub id: usize,
    pub message: String,
    pub notification_type: NotificationType,
    pub duration: Option<u64>,
}

impl Notification {
    pub fn new(message: String, notification_type: NotificationType) -> Self {
        Self {
            id: 0,
            message,
            notification_type,
            duration: Some(5000),
        }
    }

    pub fn success(message: String) -> Self {
        Self::new(message, NotificationType::Success)
    }

    pub fn error(message: String) -> Self {
        Self::new(message, NotificationType::Error)
    }

    pub fn warning(message: String) -> Self {
        Self::new(message, NotificationType::Warning)
    }

    pub fn info(message: String) -> Self {
        Self::new(message, NotificationType::Info)
    }

    pub fn with_duration(mut self, duration_ms: u64) -> Self {
        self.duration = Some(duration_ms);
        self
    }

    pub fn persistent(mut self) -> Self {
        self.duration = None;
        self
    }
}

