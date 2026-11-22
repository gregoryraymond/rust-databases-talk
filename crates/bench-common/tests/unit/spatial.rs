use super::*;

const SQUARE: &str = "POLYGON((0 0, 1 0, 1 1, 0 1, 0 0))";

#[test]
fn point_inside_outline_is_contained() {
    assert!(outline_contains(SQUARE, 0.5, 0.5).unwrap());
    assert!(!outline_contains(SQUARE, 1.5, 0.5).unwrap());
}

#[test]
fn bad_wkt_is_an_error() {
    assert!(outline_contains("POINT(1 2)", 0.0, 0.0).is_err());
}

#[test]
fn one_degree_of_latitude_is_about_111_km() {
    let metres = distance_metres((145.0, -37.0), (145.0, -38.0));
    assert!((metres - 111_195.0).abs() < 100.0, "{metres}");
}

#[test]
fn keeps_only_nearby_sorted_by_distance() {
    let found = nearest_within(
        (145.0, -37.85),
        1_000.0,
        [
            ("far".to_owned(), 145.1, -37.85),
            ("second".to_owned(), 145.005, -37.85),
            ("first".to_owned(), 145.001, -37.85),
        ],
    );
    let names: Vec<_> = found.iter().map(|n| n.name.as_str()).collect();
    assert_eq!(names, ["first", "second"]);
}
