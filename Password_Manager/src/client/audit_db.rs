use Password_Manager::db::cassandra::CassandraAudit;

const DEFAULT_CASSANDRA_NODE: &str = "127.0.0.1:9042";
const DEFAULT_CASSANDRA_USER: &str = "cassandra";
const DEFAULT_CASSANDRA_PASSWORD: &str = "cassandra";

fn cassandra_node() -> String {
    std::env::var("CASSANDRA_NODE").unwrap_or_else(|_| DEFAULT_CASSANDRA_NODE.to_string())
}

fn cassandra_user() -> String {
    std::env::var("CASSANDRA_USER").unwrap_or_else(|_| DEFAULT_CASSANDRA_USER.to_string())
}

fn cassandra_password() -> String {
    std::env::var("CASSANDRA_PASSWORD").unwrap_or_else(|_| DEFAULT_CASSANDRA_PASSWORD.to_string())
}

#[derive(Debug, Clone)]
pub struct AuditEventDisplay {
    pub user_id: String,
    pub event_type: String,
    pub event_time_ms: i64,
    pub entry_title: String,
    pub success: bool,
}

async fn connect_audit() -> Result<CassandraAudit, String> {
    CassandraAudit::connect(
        &cassandra_node(),
        &cassandra_user(),
        &cassandra_password(),
    )
    .await
    .map_err(|e| format!("Cassandra connection failed: {e}"))
}

async fn load_user_events_async(
    audit: &CassandraAudit,
    user_id: &str,
) -> Result<Vec<AuditEventDisplay>, String> {
    let details = audit
        .select_events_of_user(user_id)
        .await
        .map_err(|e| format!("Failed to load audit events: {e}"))?;
    let times = audit
        .project_type_and_time(user_id)
        .await
        .map_err(|e| format!("Failed to load event timestamps: {e}"))?;
    let titles = audit
        .project_recent_titles(user_id, details.len() as i32)
        .await
        .map_err(|e| format!("Failed to load event titles: {e}"))?;

    Ok(details
        .into_iter()
        .zip(times.into_iter())
        .zip(titles.into_iter())
        .map(|(((_, event_type, success), (_, timestamp)), (entry_title,))| AuditEventDisplay {
            user_id: user_id.to_string(),
            event_type,
            event_time_ms: timestamp.0,
            entry_title,
            success,
        })
        .collect())
}

async fn load_all_events_async() -> Result<Vec<AuditEventDisplay>, String> {
    let audit = connect_audit().await?;
    let users = audit
        .aggregate_events_per_user()
        .await
        .map_err(|e| format!("Failed to load users: {e}"))?;

    let mut events = Vec::new();
    for (user_id, _) in users {
        let mut user_events = load_user_events_async(&audit, &user_id).await?;
        events.append(&mut user_events);
    }

    events.sort_by(|a, b| b.event_time_ms.cmp(&a.event_time_ms));
    Ok(events)
}

pub async fn load_all_audit_events_for_ui() -> Result<Vec<AuditEventDisplay>, String> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| format!("Failed to start Tokio runtime: {e}"))?
        .block_on(load_all_events_async())
}
