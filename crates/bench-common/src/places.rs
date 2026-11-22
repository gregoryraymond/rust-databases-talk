//! The suburbs and schools the geospatial demos store, as plain longitude/latitude data.

use std::fmt::Write as _;

/// Metres in one degree of latitude, near enough everywhere for a search box.
const METRES_PER_DEGREE: f64 = 111_320.0;

/// A suburb boundary: a closed ring of `(longitude, latitude)` points in WGS 84.
#[derive(Debug, Clone, Copy)]
pub struct Suburb {
    /// The locality name.
    pub name: &'static str,
    /// The state it is in.
    pub state: &'static str,
    /// The outline, first point repeated last.
    pub outline: &'static [(f64, f64)],
}

impl Suburb {
    /// The outline as well-known text, the form `ST_GeomFromText` and SpatiaLite take.
    #[must_use]
    pub fn outline_wkt(&self) -> String {
        let mut wkt = String::from("POLYGON((");
        for (i, (lon, lat)) in self.outline.iter().enumerate() {
            if i > 0 {
                wkt.push_str(", ");
            }
            let _ = write!(wkt, "{lon} {lat}");
        }
        wkt.push_str("))");
        wkt
    }

    /// The smallest box holding the outline.
    #[must_use]
    pub fn bounds(&self) -> Bounds {
        self.outline.iter().fold(
            Bounds {
                min_lon: f64::INFINITY,
                min_lat: f64::INFINITY,
                max_lon: f64::NEG_INFINITY,
                max_lat: f64::NEG_INFINITY,
            },
            |b, &(lon, lat)| Bounds {
                min_lon: b.min_lon.min(lon),
                min_lat: b.min_lat.min(lat),
                max_lon: b.max_lon.max(lon),
                max_lat: b.max_lat.max(lat),
            },
        )
    }
}

/// A school's location and kind.
#[derive(Debug, Clone, Copy)]
pub struct School {
    /// The school's name.
    pub name: &'static str,
    /// Government, Catholic or Independent.
    pub sector: &'static str,
    /// Primary, Secondary, Pri/Sec and so on.
    pub school_type: &'static str,
    /// Longitude in WGS 84.
    pub lon: f64,
    /// Latitude in WGS 84.
    pub lat: f64,
}

/// A longitude/latitude box, for the index-friendly first pass of a spatial query.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds {
    /// Western edge.
    pub min_lon: f64,
    /// Southern edge.
    pub min_lat: f64,
    /// Eastern edge.
    pub max_lon: f64,
    /// Northern edge.
    pub max_lat: f64,
}

impl Bounds {
    /// A box reaching at least `metres` from `(lon, lat)` in every direction.
    #[must_use]
    pub fn around(lon: f64, lat: f64, metres: f64) -> Self {
        let dlat = metres / METRES_PER_DEGREE;
        let dlon = metres / (METRES_PER_DEGREE * lat.to_radians().cos());
        Self {
            min_lon: lon - dlon,
            min_lat: lat - dlat,
            max_lon: lon + dlon,
            max_lat: lat + dlat,
        }
    }
}

#[cfg(test)]
mod tests {
    include!("../tests/unit/places.rs");
}
