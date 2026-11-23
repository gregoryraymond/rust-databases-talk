use std::env;

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Schools the benchmark creates unless the first argument says otherwise.
pub const ROW_COUNT: u32 = 1_000_000;

/// Connections writing at once in the concurrent Postgres benchmark.
pub const WRITERS: u32 = 8;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SchoolInput {
    pub name: String,
    pub lon: f64,
    pub lat: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct School {
    pub id: Uuid,
    pub name: String,
    pub lon: f64,
    pub lat: f64,
    pub created_at: DateTime<Utc>,
}

impl School {
    pub fn new(input: &SchoolInput) -> Self {
        Self {
            id: Uuid::now_v7(),
            name: input.name.clone(),
            lon: input.lon,
            lat: input.lat,
            created_at: Utc::now(),
        }
    }
}

impl SchoolInput {
    pub fn example() -> Self {
        Self {
            name: "Prahran High School".to_owned(),
            lon: 144.9922,
            lat: -37.8540,
        }
    }

    pub fn numbered(i: u32) -> Self {
        Self {
            name: format!("school-{i}"),
            ..Self::example()
        }
    }
}

pub fn rows_from_args() -> anyhow::Result<u32> {
    Ok(match env::args().nth(1) {
        Some(arg) => arg.parse()?,
        None => ROW_COUNT,
    })
}
