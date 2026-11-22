//! Where the geospatial demos search and move things.

/// The corner of Chapel Street and Commercial Road, Prahran: the point every demo searches from.
pub const SEARCH_POINT: (f64, f64) = (144.9930, -37.8490);

/// How far from [`SEARCH_POINT`] counts as nearby: about a 20 minute walk.
pub const SEARCH_RADIUS_METRES: f64 = 1_500.0;

/// Where the update step moves the first school to: Balaclava station.
pub const MOVE_TO: (f64, f64) = (144.9935, -37.8695);

/// The spatial reference the demos store coordinates in: WGS 84 longitude/latitude, as GPS reports them.
pub const WGS84_SRID: u32 = 4326;

/// The first argument that runs a demo's geospatial example instead of its insert benchmark.
pub const GEO_COMMAND: &str = "geo";
