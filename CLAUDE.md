# Rust Databases Talk Workspace

A talk comparing Rust database libraries, each demo on the same SQLite table.
The conventions in `../CLAUDE.md` (lints, tests in `tests/unit/`, constants
modules, no `unwrap`) apply here.

## Project Structure

- `crates/bench-common/` - shared insert workload (`workload`), rows, temp DB files, `InsertStats`
- `crates/diesel-demo/` - Diesel (sync, typed query DSL, `table!`)
- `crates/seaorm-demo/` - SeaORM (async ORM on sqlx, entities/ActiveModels)
- `crates/sqlx-demo/` - sqlx (async, raw SQL, compile-time checked macros)
- `crates/rusqlite-demo/` - rusqlite (sync, thin wrapper over the C library)
- `crates/turso-demo/` - Turso (async, SQLite-compatible engine in pure Rust; no libsqlite3-sys)
- `slides/` - reveal.js presentation (outline only, not filled yet)
- `.devcontainer/` - dev container plus Postgres 17 via `docker-compose.yml`
- `justfile` - every command; `just` lists them

Each demo crate has the same modules: `connection` (constants), `version`
(reads `sqlite_version()`), `inserts` (the benchmark).

## Toolchain

`rust-toolchain.toml` pins stable: sqlx 0.9 and sea-orm 2.0 need Rust 1.94+.

## SQLite linking

Cargo allows only one crate with `links = "sqlite3"` per workspace, so every
library must resolve to the same `libsqlite3-sys` (currently 0.40, built
`bundled`). Upgrading one library may force upgrading the others. Turso is a
separate engine and is not part of this constraint.

## Build & Run

```bash
just run sqlx-demo          # version + 3M inserts
just run sqlx-demo 100000   # custom row count
just run-all                # every demo
just test                   # quick tests (1000 inserts each)
just bench                  # 3M-insert benchmark for every library, release
just bench-one turso-demo   # one library
just check                  # fmt, clippy, test, doc, deny, audit
just serve                  # http://localhost:1948
```

The benchmark inserts one row per statement inside a single transaction, into
a fresh file database in the temp directory, and times the inserts plus commit.

## Postgres

The devcontainer runs Postgres 17 as the `postgres` compose service and sets
`POSTGRES_URL=postgres://talk:talk@postgres:5432/talk` in the dev container.
From the host: `just db-up` (port 5432, override with `POSTGRES_PORT`), then
`postgres://talk:talk@localhost:5432/talk`; `just psql`, `just db-down`,
`just db-reset`. The demos still target SQLite; none reads `POSTGRES_URL` yet.

## Key References

- Diesel: https://diesel.rs/guides/
- SeaORM: https://www.sea-ql.org/SeaORM/docs/
- sqlx: https://docs.rs/sqlx/
- rusqlite: https://docs.rs/rusqlite/
- Turso: https://docs.rs/turso/
- SQLite: https://www.sqlite.org/docs.html
