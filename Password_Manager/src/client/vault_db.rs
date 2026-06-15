use Password_Manager::db::models::VaultEntry;
use chrono::Utc;
use futures::TryStreamExt;
use mongodb::{bson::doc, Client, Collection};

const DEFAULT_MONGO_URI: &str =
    "mongodb://pm_admin:admin123@localhost:27017/?authSource=admin";

const PLACEHOLDER_TITLE: &str = "Vault Entry";
const PLACEHOLDER_USERNAME: &str = "-";
const PLACEHOLDER_URL: &str = "-";
const PLACEHOLDER_CATEGORY: &str = "General";
const PLACEHOLDER_TAGS: &str = "vault";

fn mongo_uri() -> String {
    std::env::var("MONGO_URI").unwrap_or_else(|_| DEFAULT_MONGO_URI.to_string())
}

fn parse_tags(tags_csv: &str) -> Vec<String> {
    tags_csv
        .split(',')
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
        .collect()
}

pub fn vault_entry_from_password(password: &str) -> Result<VaultEntry, String> {
    vault_entry_from_form(
        PLACEHOLDER_TITLE,
        PLACEHOLDER_USERNAME,
        password,
        PLACEHOLDER_URL,
        PLACEHOLDER_CATEGORY,
        PLACEHOLDER_TAGS,
    )
}

pub fn vault_entry_from_form(
    title: &str,
    username: &str,
    password_encrypted: &str,
    url: &str,
    category: &str,
    tags_csv: &str,
) -> Result<VaultEntry, String> {
    if title.trim().is_empty() {
        return Err("Title cannot be empty".to_string());
    }
    if username.trim().is_empty() {
        return Err("Username cannot be empty".to_string());
    }
    if password_encrypted.trim().is_empty() {
        return Err("Password cannot be empty".to_string());
    }
    let tags = parse_tags(tags_csv);
    Ok(VaultEntry::new(
        title,
        username,
        password_encrypted,
        url,
        category,
        &tags.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
        Utc::now().timestamp(),
    ))
}

async fn get_collection() -> Result<Collection<VaultEntry>, String> {
    let client = Client::with_uri_str(&mongo_uri())
        .await
        .map_err(|e| format!("Failed to connect to MongoDB: {e}"))?;
    let db = client.database("password_manager");
    Ok(db.collection::<VaultEntry>("vault_entries"))
}

async fn load_all_entries_async() -> Result<Vec<VaultEntry>, String> {
    let collection = get_collection().await?;
    let cursor = collection
        .find(doc! {})
        .await
        .map_err(|e| format!("Failed to query MongoDB: {e}"))?;
    cursor
        .try_collect()
        .await
        .map_err(|e| format!("Failed to read MongoDB cursor: {e}"))
}

async fn save_entry_async(entry: VaultEntry) -> Result<(), String> {
    let collection = get_collection().await?;
    collection
        .insert_one(entry)
        .await
        .map(|_| ())
        .map_err(|e| format!("Failed to save vault entry: {e}"))
}

pub fn display_password(entry: &VaultEntry) -> String {
    let value = entry.password_encrypted.trim();
    if value.starts_with("ENC[") && value.ends_with(']') {
        value[4..value.len() - 1].to_string()
    } else {
        value.to_string()
    }
}

pub async fn load_vault_entries_for_ui() -> Result<Vec<VaultEntry>, String> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| format!("Failed to start Tokio runtime: {e}"))?
        .block_on(load_all_entries_async())
}

pub async fn save_vault_entry_for_ui(entry: VaultEntry) -> Result<(), String> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| format!("Failed to start Tokio runtime: {e}"))?
        .block_on(save_entry_async(entry))
}
