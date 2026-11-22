use super::*;

#[tokio::test]
async fn reports_a_sqlite_3_version() {
    let version = library_version().await.unwrap();
    assert!(version.starts_with("3."), "unexpected version {version}");
}
