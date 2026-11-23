use bench_common::school::{School, SchoolInput};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::school;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = school)]
pub struct SchoolRow {
    pub id: Vec<u8>,
    pub name: String,
    pub lon: f64,
    pub lat: f64,
    pub created_at: NaiveDateTime,
}

#[derive(AsChangeset)]
#[diesel(table_name = school)]
pub struct SchoolChanges {
    pub name: String,
    pub lon: f64,
    pub lat: f64,
}

impl From<&School> for SchoolRow {
    fn from(s: &School) -> Self {
        Self {
            id: s.id.as_bytes().to_vec(),
            name: s.name.clone(),
            lon: s.lon,
            lat: s.lat,
            created_at: s.created_at.naive_utc(),
        }
    }
}

impl From<&SchoolInput> for SchoolChanges {
    fn from(i: &SchoolInput) -> Self {
        Self {
            name: i.name.clone(),
            lon: i.lon,
            lat: i.lat,
        }
    }
}

impl From<SchoolRow> for School {
    fn from(r: SchoolRow) -> Self {
        Self {
            id: Uuid::from_slice(&r.id).unwrap_or_default(),
            name: r.name,
            lon: r.lon,
            lat: r.lat,
            created_at: r.created_at.and_utc(),
        }
    }
}
