use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize, Serializer};

#[derive(Serialize, Deserialize)]
pub enum PerformanceUnit {
    Seconds,
    Minutes,
}

#[derive(Serialize, Deserialize)]
pub struct PerformanceEntryMongoDb {
    pub _id: ObjectId,
    pub discipline: String, // e.g. Running
    pub metric: String,     // e.g. 10 km
    pub value: u64,         // 55 * 60 + 16 -> 55min and 16 seconds
    pub unit: PerformanceUnit,
    pub measured_at: DateTime,
    pub created_at: DateTime,
    pub modified_at: DateTime,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformanceEntry {
    pub id: String,
    pub discipline: String,
    pub metric: String,
    pub value: u64,
    pub unit: PerformanceUnit,
    #[serde(serialize_with = "serialize_datetime")]
    pub measured_at: DateTime,
    #[serde(serialize_with = "serialize_datetime")]
    pub created_at: DateTime,
    #[serde(serialize_with = "serialize_datetime")]
    pub modified_at: DateTime,
}

fn serialize_datetime<S>(date: &DateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&date.to_string())
}

impl From<PerformanceEntryMongoDb> for PerformanceEntry {
    fn from(performance_entry_mongo_db: PerformanceEntryMongoDb) -> Self {
        Self {
            id: performance_entry_mongo_db._id.to_hex(),
            discipline: performance_entry_mongo_db.discipline,
            metric: performance_entry_mongo_db.metric,
            value: performance_entry_mongo_db.value,
            unit: performance_entry_mongo_db.unit,
            measured_at: performance_entry_mongo_db.measured_at,
            created_at: performance_entry_mongo_db.created_at,
            modified_at: performance_entry_mongo_db.modified_at,
        }
    }
}
