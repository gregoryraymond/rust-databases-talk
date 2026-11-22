use super::*;

#[test]
fn removes_an_earlier_runs_file() {
    let path = fresh_database("bench-common-test").unwrap();
    fs::write(&path, b"stale").unwrap();
    let again = fresh_database("bench-common-test").unwrap();
    assert_eq!(path, again);
    assert!(!again.exists());
}

#[test]
fn postgres_url_is_never_empty() {
    assert!(postgres_url().starts_with("postgres://"));
}
