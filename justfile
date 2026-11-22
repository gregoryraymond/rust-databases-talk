set shell := ["bash", "-cu"]
set windows-shell := ["bash", "-cu"]

compose := "docker compose -f .devcontainer/docker-compose.yml"
demos := "diesel-demo seaorm-demo sqlx-demo rusqlite-demo turso-demo"
libraries := "diesel diesel-async sqlx seaorm rusqlite turso"

default:
    @just --list

build:
    cargo build --workspace --all-targets

run crate rows="3000000":
    cargo run --release -p {{crate}} -- {{rows}}

run-all rows="3000000":
    for c in {{demos}}; do cargo run --release -q -p $c -- {{rows}}; done

geo crate:
    cargo run -q -p {{crate}} -- geo

geo-all:
    for c in {{demos}}; do cargo run -q -p $c -- geo; echo; done

api:
    cargo run -p school-api

openapi:
    cargo run -q -p school-api -- openapi > crates/school-api/openapi.json

crud library url="http://localhost:8080":
    #!/usr/bin/env bash
    set -euo pipefail
    base="{{url}}/{{library}}/schools"
    create='{"name":"Prahran High School","sector":"Government","school_type":"Secondary","lon":144.9922,"lat":-37.8540}'
    update='{"name":"Prahran High School","sector":"Government","school_type":"Pri/Sec","lon":144.9935,"lat":-37.8695}'
    echo "POST $base"
    created=$(curl -sf -X POST -H 'content-type: application/json' -d "$create" "$base")
    echo "$created"
    id=$(echo "$created" | sed -E 's/.*"id":"([^"]+)".*/\1/')
    echo "GET $base/$id"
    curl -sf "$base/$id"; echo
    echo "PUT $base/$id"
    curl -sf -X PUT -H 'content-type: application/json' -d "$update" "$base/$id"; echo
    echo "GET $base"
    curl -sf "$base"; echo
    echo "DELETE $base/$id"
    curl -s -o /dev/null -w '%{http_code}\n' -X DELETE "$base/$id"
    echo "GET $base/$id"
    curl -s -o /dev/null -w '%{http_code}\n' "$base/$id"

crud-all url="http://localhost:8080":
    for l in {{libraries}}; do just crud $l {{url}}; echo; done

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

check-slides:
    python3 slides/check.py

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
