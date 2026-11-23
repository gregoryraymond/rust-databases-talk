Rust database libraries compared: Diesel (sync and diesel-async), sqlx, SeaORM, rusqlite and Turso.
Open the dev container; it has Postgres with PostGIS and every prerequisite.

- `just serve`: the slides on http://localhost:1948. Script in `docs/script.md`.
- `just bench [rows]`: each library creates 1M schools in one transaction. Switch `let backend` in a `main.rs` to `Backend::Postgres` for Postgres.
- `cargo run --release -p <crate> --bin concurrent -- [rows]` (diesel-demo, sqlx-demo, seaorm-demo): the Postgres benchmark with 8 writers at once. Diesel's uses diesel-async.
- `just clean`: deletes the SQLite files and resets the Postgres databases.

Each crate has `create`, `retrieve`, `update` and `delete` for one school, and its own migrations folder.
`cargo run -p <crate>` walks one school through them, then runs the benchmark.
`cargo run -p diesel-demo --bin geo` (also sqlx-demo, seaorm-demo) finds which suburb contains a point with PostGIS `ST_Contains`.
`cargo run -p school-api` serves the same CRUD over HTTP at `/{library}/schools` with Dropshot; `-- openapi` prints the spec.

1M creates, one transaction per writer, release build:

| Library | SQLite | Postgres, 1 writer | Postgres, 8 writers |
|---|---|---|---|
| rusqlite | 2.0 s | | |
| diesel | 8.6 s | 123.7 s | 47.2 s (diesel-async) |
| turso | 13.3 s | | |
| sqlx | 51.8 s | 136.1 s | 25.5 s |
| seaorm | 65.6 s | 144.5 s | 28.6 s |
