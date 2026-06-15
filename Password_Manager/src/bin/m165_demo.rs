use Password_Manager::db::{
    cassandra::CassandraAudit,
    models::{AuditEvent, VaultEntry},
    mongo::MongoVault,
};

fn env_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

fn test_vault_entries() -> Vec<VaultEntry> {
    let day = 86_400;
    let base = 1_750_000_000;
    vec![
        VaultEntry::new("E-Banking", "noe.b", "ENC[k9f3...]", "https://ebanking.example.ch", "Banking", &["wichtig", "finanzen"], base),
        VaultEntry::new("Kreditkarte", "noe.b", "ENC[a8d1...]", "https://cards.example.ch", "Banking", &["finanzen"], base + day),
        VaultEntry::new("GitHub", "chefff", "ENC[q2m7...]", "https://github.com", "Entwicklung", &["arbeit", "wichtig"], base + 2 * day),
        VaultEntry::new("crates.io", "chefff", "ENC[z5t0...]", "https://crates.io", "Entwicklung", &["arbeit"], base + 3 * day),
        VaultEntry::new("Moodle BBB", "noe.bachofner", "ENC[p4w8...]", "https://moodle.bbbaden.ch", "Schule", &["schule"], base + 4 * day),
        VaultEntry::new("Schul-Mail", "noe.bachofner", "ENC[j6r2...]", "https://mail.bbbaden.ch", "Schule", &["schule", "wichtig"], base + 5 * day),
        VaultEntry::new("Steam", "chefff", "ENC[m1c9...]", "https://steampowered.com", "Gaming", &["privat"], base + 6 * day),
        VaultEntry::new("DCS World", "chefff", "ENC[x7v4...]", "https://digitalcombatsimulator.com", "Gaming", &["privat", "simulation"], base + 7 * day),
        VaultEntry::new("Nextcloud", "admin", "ENC[b3n6...]", "https://cloud.local", "Selfhosting", &["server", "wichtig"], base + 8 * day),
        VaultEntry::new("Vaultwarden", "admin", "ENC[h0s5...]", "https://vault.local", "Selfhosting", &["server"], base + 9 * day),
    ]
}

fn test_audit_events() -> Vec<AuditEvent> {
    let minute = 60_000;
    let base = 1_750_000_000_000_i64;
    let ev = |user: &str, offset: i64, typ: &str, title: &str, ok: bool| AuditEvent {
        user_id: user.to_string(),
        event_time_ms: base + offset * minute,
        event_type: typ.to_string(),
        entry_title: title.to_string(),
        success: ok,
    };
    vec![
        ev("noe", 0, "login", "-", true),
        ev("noe", 1, "read_entry", "E-Banking", true),
        ev("noe", 2, "update_entry", "E-Banking", true),
        ev("noe", 3, "read_entry", "Kreditkarte", true),
        ev("noe", 4, "logout", "-", true),
        ev("alwin", 0, "login", "-", false),
        ev("alwin", 1, "login", "-", true),
        ev("alwin", 2, "create_entry", "GitLab", true),
        ev("alwin", 3, "delete_entry", "Altes Konto", false),
        ev("alwin", 4, "logout", "-", true),
    ]
}

async fn run_mongo_demo() -> mongodb::error::Result<()> {
    let uri = env_or(
        "MONGO_URI",
        "mongodb://pm_admin:admin123@localhost:27017/?authSource=admin",
    );
    println!("=== MongoDB ({uri}) ===\n");

    let vault = MongoVault::connect(&uri).await?;

    let inserted = vault.seed_if_empty(test_vault_entries()).await?;
    println!("[Schreibzugriff] Neu eingefuegte Testdatensaetze: {inserted}\n");

    println!("[Selektion 1] Eintraege der Kategorie 'Banking':");
    for e in vault.select_by_category("Banking").await? {
        println!("  - {} ({})", e.title, e.username);
    }

    println!("\n[Selektion 2] Eintraege mit Tag 'wichtig', erstellt ab 1750259200:");
    for e in vault.select_recent_with_tag(1_750_259_200, "wichtig").await? {
        println!("  - {} (created_at={})", e.title, e.created_at);
    }

    println!("\n[Projektion 1] Nur Titel + Benutzername (Passwort bleibt in der DB):");
    for d in vault.project_title_username().await? {
        println!("  - {d}");
    }

    println!("\n[Projektion 2] Nur Titel + URL der Kategorie 'Entwicklung':");
    for d in vault.project_urls_of_category("Entwicklung").await? {
        println!("  - {d}");
    }

    println!("\n[Aggregation 1] Anzahl Eintraege pro Kategorie:");
    for d in vault.aggregate_count_per_category().await? {
        println!("  - {d}");
    }

    println!("\n[Aggregation 2] Durchschnittliche Anzahl Tags pro Kategorie:");
    for d in vault.aggregate_avg_tags_per_category().await? {
        println!("  - {d}");
    }

    Ok(())
}

async fn run_cassandra_demo() -> Result<(), Box<dyn std::error::Error>> {
    let node = env_or("CASSANDRA_NODE", "127.0.0.1:9042");
    let user = env_or("CASSANDRA_USER", "cassandra");
    let password = env_or("CASSANDRA_PASSWORD", "cassandra");
    println!("\n=== Cassandra ({node}) ===\n");

    let audit = CassandraAudit::connect(&node, &user, &password).await?;

    println!("[Schreibzugriff] Fuege 10 Audit-Events ein (idempotent dank PK):");
    for ev in test_audit_events() {
        audit.insert_event(&ev).await?;
    }
    println!("  ok\n");

    println!("[Selektion 1] Alle Events von 'noe' (Zugriff via Partition Key):");
    for (user, typ, ok) in audit.select_events_of_user("noe").await? {
        println!("  - {user}: {typ} (success={ok})");
    }

    println!("\n[Selektion 2] Fehlgeschlagene Events von 'alwin' (ALLOW FILTERING):");
    for (user, typ) in audit.select_failed_events_of_user("alwin").await? {
        println!("  - {user}: {typ}");
    }

    println!("\n[Projektion 1] Nur event_type + event_time von 'noe':");
    for (typ, ts) in audit.project_type_and_time("noe").await? {
        println!("  - {typ} @ {}ms", ts.0);
    }

    println!("\n[Projektion 2] Nur die Titel der letzten 3 Events von 'alwin':");
    for (title,) in audit.project_recent_titles("alwin", 3).await? {
        println!("  - {title}");
    }

    println!("\n[Aggregation 1] Anzahl Events pro Benutzer (GROUP BY Partition Key):");
    for (user, count) in audit.aggregate_events_per_user().await? {
        println!("  - {user}: {count}");
    }

    println!("\n[Aggregation 2] Neuestes Event von 'noe' (MAX):");
    match audit.aggregate_latest_event_time("noe").await? {
        Some(ts) => println!("  - {}ms", ts.0),
        None => println!("  - keine Events vorhanden"),
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = run_mongo_demo().await {
        eprintln!("MongoDB-Demo fehlgeschlagen: {e}");
    }
    if let Err(e) = run_cassandra_demo().await {
        eprintln!("Cassandra-Demo fehlgeschlagen: {e}");
    }
}
