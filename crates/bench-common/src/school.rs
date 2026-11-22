//! The school resource every library's CRUD module reads and writes, and the API serves.

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The fields a client sends to create or replace a school.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SchoolInput {
    /// The school's name.
    pub name: String,
    /// Government, Catholic or Independent.
    pub sector: String,
    /// Primary, Secondary, Pri/Sec and so on.
    pub school_type: String,
    /// Longitude in WGS 84.
    pub lon: f64,
    /// Latitude in WGS 84.
    pub lat: f64,
}

/// A stored school.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SchoolRecord {
    /// A time-ordered id.
    pub id: Uuid,
    /// The school's name.
    pub name: String,
    /// Government, Catholic or Independent.
    pub sector: String,
    /// Primary, Secondary, Pri/Sec and so on.
    pub school_type: String,
    /// Longitude in WGS 84.
    pub lon: f64,
    /// Latitude in WGS 84.
    pub lat: f64,
    /// When the row was made.
    pub created_at: DateTime<Utc>,
    /// When the row last changed.
    pub updated_at: DateTime<Utc>,
}

impl SchoolRecord {
    /// A new record for `input`, with a fresh UUIDv7 and both timestamps set to now.
    #[must_use]
    pub fn new(input: &SchoolInput) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::now_v7(),
            name: input.name.clone(),
            sector: input.sector.clone(),
            school_type: input.school_type.clone(),
            lon: input.lon,
            lat: input.lat,
            created_at: now,
            updated_at: now,
        }
    }
}
