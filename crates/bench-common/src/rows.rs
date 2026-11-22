//! The rows the benchmark inserts.

/// The `name` and `value` of row `i`: distinct, cheap to build, and the same for every library.
#[must_use]
pub fn row(i: u32) -> (String, i64) {
    (format!("item-{i}"), i64::from(i) * 7)
}

#[cfg(test)]
mod tests {
    include!("../tests/unit/rows.rs");
}
