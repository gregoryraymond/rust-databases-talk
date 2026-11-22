use super::*;

const SQUARE: Suburb = Suburb {
    name: "Square",
    state: "VIC",
    outline: &[(1.0, 2.0), (3.0, 2.0), (3.0, 4.0), (1.0, 2.0)],
};

#[test]
fn writes_outline_as_wkt() {
    assert_eq!(SQUARE.outline_wkt(), "POLYGON((1 2, 3 2, 3 4, 1 2))");
}

#[test]
fn bounds_cover_the_outline() {
    assert_eq!(
        SQUARE.bounds(),
        Bounds {
            min_lon: 1.0,
            min_lat: 2.0,
            max_lon: 3.0,
            max_lat: 4.0
        }
    );
}

#[test]
fn box_around_a_point_reaches_the_radius() {
    let b = Bounds::around(145.0, -37.85, 1_000.0);
    assert!((b.max_lat - b.min_lat - 2_000.0 / METRES_PER_DEGREE).abs() < 1e-9);
    assert!(b.max_lon - b.min_lon > b.max_lat - b.min_lat);
}
