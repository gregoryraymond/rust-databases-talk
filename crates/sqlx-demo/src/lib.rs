use bench_common::school::School;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub mod postgres;
pub mod sqlite;

type SchoolRow = (Uuid, String, f64, f64, DateTime<Utc>);

fn school((id, name, lon, lat, created_at): SchoolRow) -> School {
    School {
        id,
        name,
        lon,
        lat,
        created_at,
    }
}
