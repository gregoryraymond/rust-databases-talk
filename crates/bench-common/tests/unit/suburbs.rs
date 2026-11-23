use super::*;

#[test]
fn writes_outline_as_wkt() {
    let square = Suburb {
        name: "Square",
        outline: &[(1.0, 2.0), (3.0, 2.0), (3.0, 4.0), (1.0, 2.0)],
    };
    assert_eq!(square.outline_wkt(), "POLYGON((1 2, 3 2, 3 4, 1 2))");
}
