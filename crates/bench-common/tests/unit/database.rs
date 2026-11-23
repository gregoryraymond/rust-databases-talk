use super::*;

#[test]
fn fresh_sqlite_path_removes_an_earlier_file() {
    let path = fresh_sqlite_path("bench-common-test").unwrap();
    fs::write(&path, b"stale").unwrap();
    assert!(!fresh_sqlite_path("bench-common-test").unwrap().exists());
}

#[test]
fn postgres_url_names_the_database() {
    assert!(postgres_url("talk_sqlx").ends_with("/talk_sqlx"));
}
