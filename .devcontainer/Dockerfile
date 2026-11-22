FROM mcr.microsoft.com/devcontainers/rust:1-bookworm

RUN apt-get update && apt-get install -y --no-install-recommends \
        ca-certificates curl gnupg \
    && install -d /usr/share/postgresql-common/pgdg \
    && curl -fsSL https://www.postgresql.org/media/keys/ACCC4CF8.asc \
        | gpg --dearmor -o /usr/share/postgresql-common/pgdg/apt.gpg \
    && echo "deb [signed-by=/usr/share/postgresql-common/pgdg/apt.gpg] https://apt.postgresql.org/pub/repos/apt bookworm-pgdg main" \
        > /etc/apt/sources.list.d/pgdg.list \
    && apt-get update && apt-get install -y --no-install-recommends \
        build-essential \
        pkg-config \
        clang \
        libpq-dev \
        postgresql-client-17 \
        libsqlite3-dev \
        sqlite3 \
        python3 \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install diesel_cli --no-default-features --features sqlite,postgres \
    && cargo install sqlx-cli --no-default-features --features sqlite,postgres,rustls \
    && cargo install sea-orm-cli just cargo-deny cargo-audit \
    && rm -rf /usr/local/cargo/registry
