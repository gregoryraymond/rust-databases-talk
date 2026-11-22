//! How long a benchmark run took.

use std::{fmt, time::Duration};

/// The result of inserting `rows` rows with one library.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InsertStats {
    /// The library that did the inserts.
    pub library: &'static str,
    /// How many rows were inserted.
    pub rows: u32,
    /// Wall-clock time from the first insert to the commit.
    pub elapsed: Duration,
}

impl InsertStats {
    /// Rows inserted per second.
    #[must_use]
    pub fn rows_per_second(&self) -> f64 {
        f64::from(self.rows) / self.elapsed.as_secs_f64()
    }

    /// Average time per row, in microseconds.
    #[must_use]
    pub fn micros_per_row(&self) -> f64 {
        self.elapsed.as_secs_f64() * 1e6 / f64::from(self.rows)
    }
}

impl fmt::Display for InsertStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {} inserts in {:.2?} ({:.0} rows/s, {:.2} us/row)",
            self.library,
            self.rows,
            self.elapsed,
            self.rows_per_second(),
            self.micros_per_row(),
        )
    }
}

#[cfg(test)]
mod tests {
    include!("../tests/unit/stats.rs");
}
