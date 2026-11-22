//! The workloads shared by every database demo: the insert benchmark, so each library
//! inserts the same rows into the same table, and the geospatial demo's data, SQL and report.

pub mod backend;
pub mod connection;
pub mod database;
pub mod geo_workload;
pub mod places;
pub mod postgis;
pub mod report;
pub mod rows;
pub mod sample_data;
pub mod school;
pub mod spatial;
pub mod sqlite_geo;
pub mod stats;
pub mod workload;

pub use backend::Backend;
pub use stats::InsertStats;
