set shell := ["bash", "-cu"]

default:
    @just --list

serve:
    python3 -m http.server 1948 --directory slides

bench rows="1000000":
    for c in diesel-demo seaorm-demo sqlx-demo rusqlite-demo turso-demo; do cargo run --release -q -p $c -- {{rows}} | tail -1; done

clean:
    rm -f /tmp/*.db
    for db in talk_diesel talk_sqlx talk_seaorm; do psql -q -c "DROP DATABASE IF EXISTS $db WITH (FORCE)" -c "CREATE DATABASE $db"; done
