use futures::TryStreamExt;
use mongodb::{
    bson::{doc, Document},
    Client, Collection,
};

use super::models::VaultEntry;

pub struct MongoVault {
    entries: Collection<VaultEntry>,
    raw: Collection<Document>,
}

impl MongoVault {
    pub async fn connect(uri: &str) -> mongodb::error::Result<Self> {
        let client = Client::with_uri_str(uri).await?;
        let db = client.database("password_manager");
        Ok(Self {
            entries: db.collection::<VaultEntry>("vault_entries"),
            raw: db.collection::<Document>("vault_entries"),
        })
    }

    pub async fn seed_if_empty(&self, entries: Vec<VaultEntry>) -> mongodb::error::Result<u64> {
        let existing = self.entries.count_documents(doc! {}).await?;
        if existing > 0 {
            return Ok(0);
        }
        let result = self.entries.insert_many(entries).await?;
        Ok(result.inserted_ids.len() as u64)
    }

    pub async fn select_by_category(
        &self,
        category: &str,
    ) -> mongodb::error::Result<Vec<VaultEntry>> {
        let cursor = self.entries.find(doc! { "category": category }).await?;
        cursor.try_collect().await
    }

    pub async fn select_recent_with_tag(
        &self,
        created_after: i64,
        tag: &str,
    ) -> mongodb::error::Result<Vec<VaultEntry>> {
        let filter = doc! {
            "created_at": { "$gte": created_after },
            "tags": tag,
        };
        let cursor = self.entries.find(filter).await?;
        cursor.try_collect().await
    }

    pub async fn project_title_username(&self) -> mongodb::error::Result<Vec<Document>> {
        let cursor = self
            .raw
            .find(doc! {})
            .projection(doc! { "title": 1, "username": 1, "_id": 0 })
            .await?;
        cursor.try_collect().await
    }

    pub async fn project_urls_of_category(
        &self,
        category: &str,
    ) -> mongodb::error::Result<Vec<Document>> {
        let cursor = self
            .raw
            .find(doc! { "category": category })
            .projection(doc! { "title": 1, "url": 1, "_id": 0 })
            .await?;
        cursor.try_collect().await
    }

    pub async fn aggregate_count_per_category(&self) -> mongodb::error::Result<Vec<Document>> {
        let pipeline = vec![
            doc! { "$group": { "_id": "$category", "anzahl": { "$sum": 1 } } },
            doc! { "$sort": { "anzahl": -1 } },
        ];
        let cursor = self.raw.aggregate(pipeline).await?;
        cursor.try_collect().await
    }

    pub async fn aggregate_avg_tags_per_category(&self) -> mongodb::error::Result<Vec<Document>> {
        let pipeline = vec![
            doc! { "$project": { "category": 1, "tag_count": { "$size": "$tags" } } },
            doc! { "$group": { "_id": "$category", "avg_tags": { "$avg": "$tag_count" } } },
        ];
        let cursor = self.raw.aggregate(pipeline).await?;
        cursor.try_collect().await
    }
}
