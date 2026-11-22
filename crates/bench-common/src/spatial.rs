//! The exact spatial tests SQLite cannot do without SpatiaLite, done in Rust with `geo`.

use geo::{Contains as _, Distance as _, Haversine, Point, Polygon};
use wkt::TryFromWkt as _;

use crate::report::Nearby;

/// Why a spatial test could not run.
#[derive(Debug, thiserror::Error)]
pub enum SpatialError {
    /// A stored outline did not parse as a WKT polygon.
    #[error("outline is not a WKT polygon: {0}")]
    Wkt(String),
}

/// Whether the WKT polygon `outline` contains the point `(lon, lat)`.
///
/// # Errors
///
/// Fails if `outline` is not a WKT polygon.
pub fn outline_contains(outline: &str, lon: f64, lat: f64) -> Result<bool, SpatialError> {
    let polygon =
        Polygon::<f64>::try_from_wkt_str(outline).map_err(|e| SpatialError::Wkt(e.to_string()))?;
    Ok(polygon.contains(&Point::new(lon, lat)))
}

/// Great-circle distance in metres between two `(lon, lat)` points.
#[must_use]
pub fn distance_metres(from: (f64, f64), to: (f64, f64)) -> f64 {
    Haversine.distance(Point::from(from), Point::from(to))
}

/// The `(name, lon, lat)` candidates within `metres` of `from`, nearest first.
#[must_use]
pub fn nearest_within(
    from: (f64, f64),
    metres: f64,
    candidates: impl IntoIterator<Item = (String, f64, f64)>,
) -> Vec<Nearby> {
    let mut nearby: Vec<Nearby> = candidates
        .into_iter()
        .map(|(name, lon, lat)| Nearby {
            name,
            metres: distance_metres(from, (lon, lat)),
        })
        .filter(|n| n.metres <= metres)
        .collect();
    nearby.sort_by(|a, b| a.metres.total_cmp(&b.metres));
    nearby
}

#[cfg(test)]
mod tests {
    include!("../tests/unit/spatial.rs");
}
