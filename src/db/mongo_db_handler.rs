use anyhow::Result;
use bson::doc;
use mongodb::{options::ClientOptions, Client, Collection, Database};
use tracing::info;

use crate::model::performance_entry::PerformanceEntryMongoDb;

pub struct MongoDbHandler {
    pub performance_entries_collection: Collection<PerformanceEntryMongoDb>,
    pub db: Database,
}

impl MongoDbHandler {
    pub async fn new(
        db_user: &str,
        db_password: &str,
        db_name: &str,
        db_host: &str,
    ) -> Result<Self> {
        let uri =
            format!("mongodb://{db_user}:{db_password}@{db_host}/{db_name}?authSource={db_name}");
        let client_options = ClientOptions::parse(uri).await?;
        let client = Client::with_options(client_options)?;

        let connect_info = format!("database '{db_name}' as user '{db_user}' on host '{db_host}'");
        info!("Trying to connect to {connect_info}");

        let db = client.database(db_name);
        db.run_command(doc! {"ping": 1}).await?;
        info!("Connected to {connect_info}");
        let performance_entries_collection = db.collection("performance_entries");

        let db_handler = MongoDbHandler {
            performance_entries_collection,
            db,
        };

        Ok(db_handler)
    }
}
