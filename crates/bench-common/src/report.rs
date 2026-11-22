//! What each geospatial demo found, printed the same way for every library.

use std::fmt;

use chrono::{DateTime, SecondsFormat, Utc};
use uuid::Uuid;

/// A school found near the search point.
#[derive(Debug, Clone, PartialEq)]
pub struct Nearby {
    /// The school's name.
    pub name: String,
    /// How far it is from the search point.
    pub metres: f64,
}

/// The results of one geospatial demo run.
#[derive(Debug, Clone)]
pub struct GeoReport {
    /// The library that ran it.
    pub library: &'static str,
    /// The spatial engine: PostGIS, or plain SQLite plus Rust.
    pub engine: &'static str,
    /// Suburbs inserted.
    pub suburbs: usize,
    /// Schools inserted.
    pub schools: usize,
    /// The first school as read back, from [`describe_school`].
    pub read_back: String,
    /// Suburbs containing the search point.
    pub suburbs_at_search_point: Vec<String>,
    /// Schools within the search radius, nearest first.
    pub nearby: Vec<Nearby>,
    /// The suburb the first school is in after it moved.
    pub suburb_after_move: Option<String>,
    /// Schools left after deleting the first.
    pub schools_after_delete: i64,
}

/// One line describing a school read back from the database.
#[must_use]
pub fn describe_school(
    id: Uuid,
    name: &str,
    lon: f64,
    lat: f64,
    created_at: DateTime<Utc>,
) -> String {
    format!(
        "{name} at ({lon:.5}, {lat:.5}), id {id}, created {}",
        created_at.to_rfc3339_opts(SecondsFormat::Secs, true)
    )
}

impl fmt::Display for GeoReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}/{}", self.library, self.engine)?;
        writeln!(
            f,
            "  inserted {} suburbs and {} schools",
            self.suburbs, self.schools
        )?;
        writeln!(f, "  read back: {}", self.read_back)?;
        writeln!(
            f,
            "  search point is in: {}",
            self.suburbs_at_search_point.join(", ")
        )?;
        writeln!(f, "  schools nearby:")?;
        for n in &self.nearby {
            writeln!(f, "    {:>6.0} m  {}", n.metres, n.name)?;
        }
        writeln!(
            f,
            "  after moving it, it is in: {}",
            self.suburb_after_move.as_deref().unwrap_or("no suburb")
        )?;
        write!(
            f,
            "  schools after deleting it: {}",
            self.schools_after_delete
        )
    }
}
