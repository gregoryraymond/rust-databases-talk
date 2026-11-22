set shell := ["bash", "-cu"]
set windows-shell := ["bash", "-cu"]

compose := "docker compose -f .devcontainer/docker-compose.yml"
demos := "diesel-demo seaorm-demo sqlx-demo rusqlite-demo turso-demo"

default:
    @just --list

build:
    cargo build --workspace --all-targets

run crate rows="3000000":
    cargo run --release -p {{crate}} -- {{rows}}

run-all rows="3000000":
    for c in {{demos}}; do cargo run --release -q -p $c -- {{rows}}; done

test:
    cargo test --workspace

fmt:
    cargo fmt --all

lint:
    cargo clippy --workspace --all-targets -- -D warnings

doc:
    cargo doc --no-deps --document-private-items --workspace

deny:
    cargo deny check

audit:
    cargo audit

check:
    cargo fmt --all -- --check
    just lint
    just test
    just doc
    just deny
    just audit

serve:
    ./slides/serve.sh

db-up:
    {{compose}} up -d --wait postgres

db-down:
    {{compose}} stop postgres

db-reset:
    {{compose}} down -v postgres

psql:
    {{compose}} exec postgres psql -U talk talk

clean-dbs:
    rm -f "${TMPDIR:-${TEMP:-/tmp}}"/rust-databases-talk-*
