CREATE EXTENSION IF NOT EXISTS postgis;

CREATE TABLE suburb (
    name TEXT PRIMARY KEY,
    outline geometry(Polygon, 4326) NOT NULL
);
