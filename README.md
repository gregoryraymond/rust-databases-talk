Rust database libraries compared: Diesel (sync and diesel-async), sqlx, SeaORM, rusqlite and Turso.
Open the dev container; it has Postgres with PostGIS and every prerequisite.

- `just serve`: the slides on http://localhost:1948. Script in `docs/script.md`.
- `just bench [rows]`: each library creates 1M schools in one transaction. Switch `let backend` in a `main.rs` to `Backend::Postgres` for Postgres.
- `just clean`: deletes the SQLite files and resets the Postgres databases.

Each crate has `create`, `retrieve`, `update` and `delete` for one school, and its own migrations folder.
`cargo run -p <crate>` walks one school through them, then runs the benchmark.
`cargo run -p diesel-demo --bin geo` (also sqlx-demo, seaorm-demo) finds which suburb contains a point with PostGIS `ST_Contains`.
`cargo run -p school-api` serves the same CRUD over HTTP at `/{library}/schools` with Dropshot; `-- openapi` prints the spec.

3M creates, SQLite, one transaction, release build:

| Library | Time | µs/row |
|---|---|---|
| rusqlite | 9.4 s | 3.1 |
| diesel | 27.7 s | 9.2 |
| turso | 51.9 s | 17.3 |
| sqlx | 169.6 s | 56.5 |
| seaorm | 206.6 s | 68.9 |
