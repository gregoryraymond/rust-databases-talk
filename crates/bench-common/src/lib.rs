pub mod database;
pub mod school;
pub mod suburbs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    Sqlite,
    Postgres,
}
