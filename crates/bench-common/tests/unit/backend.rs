use super::*;

#[test]
fn displays_its_name() {
    assert_eq!(Backend::Sqlite.to_string(), "sqlite");
    assert_eq!(Backend::Postgres.to_string(), "postgres");
}
