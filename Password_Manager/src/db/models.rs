use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultEntry {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub username: String,
    pub password_encrypted: String,
    pub url: String,
    pub category: String,
    pub tags: Vec<String>,
    pub created_at: i64,
}

impl VaultEntry {
    pub fn new(
        title: &str,
        username: &str,
        password_encrypted: &str,
        url: &str,
        category: &str,
        tags: &[&str],
        created_at: i64,
    ) -> Self {
        Self {
            id: None,
            title: title.to_string(),
            username: username.to_string(),
            password_encrypted: password_encrypted.to_string(),
            url: url.to_string(),
            category: category.to_string(),
            tags: tags.iter().map(|t| t.to_string()).collect(),
            created_at,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuditEvent {
    pub user_id: String,
    pub event_time_ms: i64,
    pub event_type: String,
    pub entry_title: String,
    pub success: bool,
}
