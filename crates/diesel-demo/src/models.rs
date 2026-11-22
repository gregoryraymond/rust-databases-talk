//! Diesel's structs for the school table: one per shape a query reads or writes.

use bench_common::{
    geo_workload::WGS84_SRID,
    school::{SchoolInput, SchoolRecord},
};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use postgis_diesel::types::Point;
use uuid::Uuid;

use crate::schema::school;

/// A new school row.
#[derive(Insertable)]
#[diesel(table_name = school)]
pub struct NewSchool<'a> {
    /// A time-ordered id.
    pub id: Uuid,
    /// The school's name.
    pub name: &'a str,
    /// Government, Catholic or Independent.
    pub sector: &'a str,
    /// Primary, Secondary and so on.
    pub school_type: &'a str,
    /// Where it is.
    pub location: Point,
    /// When the row was made.
    pub created_at: DateTime<Utc>,
    /// When the row last changed.
    pub updated_at: DateTime<Utc>,
}

/// A school as read back.
#[derive(Queryable, Selectable)]
#[diesel(table_name = school, check_for_backend(diesel::pg::Pg))]
pub struct SchoolRow {
    /// The id.
    pub id: Uuid,
    /// The school's name.
    pub name: String,
    /// Government, Catholic or Independent.
    pub sector: String,
    /// Primary, Secondary and so on.
    pub school_type: String,
    /// Where it is.
    pub location: Point,
    /// When the row was made.
    pub created_at: DateTime<Utc>,
    /// When the row last changed.
    pub updated_at: DateTime<Utc>,
}

/// The columns an update replaces: everything but the id and creation time.
#[derive(AsChangeset)]
#[diesel(table_name = school)]
pub struct SchoolChanges<'a> {
    /// The school's name.
    pub name: &'a str,
    /// Government, Catholic or Independent.
    pub sector: &'a str,
    /// Primary, Secondary and so on.
    pub school_type: &'a str,
    /// Where it is.
    pub location: Point,
    /// When it changed.
    pub updated_at: DateTime<Utc>,
}

/// A WGS 84 point.
#[must_use]
pub fn point((lon, lat): (f64, f64)) -> Point {
    Point::new(lon, lat, Some(WGS84_SRID))
}

impl<'a> From<&'a SchoolRecord> for NewSchool<'a> {
    fn from(r: &'a SchoolRecord) -> Self {
        Self {
            id: r.id,
            name: &r.name,
            sector: &r.sector,
            school_type: &r.school_type,
            location: point((r.lon, r.lat)),
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

impl<'a> From<&'a SchoolInput> for SchoolChanges<'a> {
    fn from(i: &'a SchoolInput) -> Self {
        Self {
            name: &i.name,
            sector: &i.sector,
            school_type: &i.school_type,
            location: point((i.lon, i.lat)),
            updated_at: Utc::now(),
        }
    }
}

impl From<SchoolRow> for SchoolRecord {
    fn from(r: SchoolRow) -> Self {
        Self {
            id: r.id,
            name: r.name,
            sector: r.sector,
            school_type: r.school_type,
            lon: r.location.x,
            lat: r.location.y,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}
