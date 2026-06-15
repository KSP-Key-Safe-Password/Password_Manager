use scylla::{Session, SessionBuilder};

use super::models::AuditEvent;

pub struct CassandraAudit {
    session: Session,
}

impl CassandraAudit {
    pub async fn connect(
        node: &str,
        user: &str,
        password: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let session = SessionBuilder::new()
            .known_node(node)
            .user(user, password)
            .build()
            .await?;

        session
            .query(
                "CREATE KEYSPACE IF NOT EXISTS pm_audit \
                 WITH replication = {'class': 'SimpleStrategy', 'replication_factor': 1}",
                &[],
            )
            .await?;

        session
            .query(
                "CREATE TABLE IF NOT EXISTS pm_audit.audit_log ( \
                    user_id text, \
                    event_time timestamp, \
                    event_type text, \
                    entry_title text, \
                    success boolean, \
                    PRIMARY KEY ((user_id), event_time) \
                 ) WITH CLUSTERING ORDER BY (event_time DESC)",
                &[],
            )
            .await?;

        Ok(Self { session })
    }

    pub async fn insert_event(&self, ev: &AuditEvent) -> Result<(), Box<dyn std::error::Error>> {
        self.session
            .query(
                "INSERT INTO pm_audit.audit_log \
                 (user_id, event_time, event_type, entry_title, success) \
                 VALUES (?, ?, ?, ?, ?)",
                (
                    &ev.user_id,
                    scylla::frame::value::CqlTimestamp(ev.event_time_ms),
                    &ev.event_type,
                    &ev.entry_title,
                    ev.success,
                ),
            )
            .await?;
        Ok(())
    }

    pub async fn select_events_of_user(
        &self,
        user_id: &str,
    ) -> Result<Vec<(String, String, bool)>, Box<dyn std::error::Error>> {
        let result = self
            .session
            .query(
                "SELECT user_id, event_type, success \
                 FROM pm_audit.audit_log WHERE user_id = ?",
                (user_id,),
            )
            .await?;

        let mut rows = Vec::new();
        for row in result.rows_typed::<(String, String, bool)>()? {
            rows.push(row?);
        }
        Ok(rows)
    }

    pub async fn select_failed_events_of_user(
        &self,
        user_id: &str,
    ) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
        let result = self
            .session
            .query(
                "SELECT user_id, event_type \
                 FROM pm_audit.audit_log \
                 WHERE user_id = ? AND success = false ALLOW FILTERING",
                (user_id,),
            )
            .await?;

        let mut rows = Vec::new();
        for row in result.rows_typed::<(String, String)>()? {
            rows.push(row?);
        }
        Ok(rows)
    }

    pub async fn project_type_and_time(
        &self,
        user_id: &str,
    ) -> Result<Vec<(String, scylla::frame::value::CqlTimestamp)>, Box<dyn std::error::Error>>
    {
        let result = self
            .session
            .query(
                "SELECT event_type, event_time \
                 FROM pm_audit.audit_log WHERE user_id = ?",
                (user_id,),
            )
            .await?;

        let mut rows = Vec::new();
        for row in result.rows_typed::<(String, scylla::frame::value::CqlTimestamp)>()? {
            rows.push(row?);
        }
        Ok(rows)
    }

    pub async fn project_recent_titles(
        &self,
        user_id: &str,
        limit: i32,
    ) -> Result<Vec<(String,)>, Box<dyn std::error::Error>> {
        let result = self
            .session
            .query(
                "SELECT entry_title \
                 FROM pm_audit.audit_log WHERE user_id = ? LIMIT ?",
                (user_id, limit),
            )
            .await?;

        let mut rows = Vec::new();
        for row in result.rows_typed::<(String,)>()? {
            rows.push(row?);
        }
        Ok(rows)
    }

    pub async fn aggregate_events_per_user(
        &self,
    ) -> Result<Vec<(String, i64)>, Box<dyn std::error::Error>> {
        let result = self
            .session
            .query(
                "SELECT user_id, COUNT(*) \
                 FROM pm_audit.audit_log GROUP BY user_id",
                &[],
            )
            .await?;

        let mut rows = Vec::new();
        for row in result.rows_typed::<(String, i64)>()? {
            rows.push(row?);
        }
        Ok(rows)
    }

    pub async fn aggregate_latest_event_time(
        &self,
        user_id: &str,
    ) -> Result<Option<scylla::frame::value::CqlTimestamp>, Box<dyn std::error::Error>> {
        let result = self
            .session
            .query(
                "SELECT MAX(event_time) \
                 FROM pm_audit.audit_log WHERE user_id = ?",
                (user_id,),
            )
            .await?;

        let mut rows = result.rows_typed::<(Option<scylla::frame::value::CqlTimestamp>,)>()?;
        match rows.next() {
            Some(row) => Ok(row?.0),
            None => Ok(None),
        }
    }
}
