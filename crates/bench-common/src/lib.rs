//! The workload and stats shared by every database demo's insert benchmark, so
//! each library inserts the same rows into the same table.

pub mod backend;
pub mod connection;
pub mod database;
pub mod rows;
pub mod stats;
pub mod workload;

pub use backend::Backend;
pub use stats::InsertStats;
