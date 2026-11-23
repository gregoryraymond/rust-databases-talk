use std::{env, fs, io, path::PathBuf};

pub fn postgres_url(database: &str) -> String {
    format!("postgres://talk:talk@postgres:5432/{database}")
}

pub fn fresh_sqlite_path(name: &str) -> io::Result<PathBuf> {
    let path = env::temp_dir().join(format!("{name}.db"));
    match fs::remove_file(&path) {
        Err(e) if e.kind() != io::ErrorKind::NotFound => Err(e),
        _ => Ok(path),
    }
}

#[cfg(test)]
mod tests {
    include!("../tests/unit/database.rs");
}
