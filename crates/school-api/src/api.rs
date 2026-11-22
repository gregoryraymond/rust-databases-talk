//! The API as a Dropshot trait: the OpenAPI document comes from this alone, before any implementation.

use bench_common::school::{SchoolInput, SchoolRecord};
use dropshot::{
    HttpError, HttpResponseCreated, HttpResponseDeleted, HttpResponseOk, Path, RequestContext,
    TypedBody,
};
use schemars::JsonSchema;
use serde::Deserialize;
use uuid::Uuid;

/// The library a request goes through.
#[derive(Debug, Clone, Copy, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum Library {
    /// Synchronous Diesel over an r2d2 pool, on a blocking thread.
    Diesel,
    /// diesel-async over a deadpool pool.
    DieselAsync,
    /// sqlx over a `PgPool`.
    Sqlx,
    /// SeaORM over its sqlx pool.
    Seaorm,
    /// rusqlite on one locked connection, on a blocking thread.
    Rusqlite,
    /// Turso, a connection per request.
    Turso,
}

/// A path naming only the library.
#[derive(Deserialize, JsonSchema)]
pub struct LibraryPath {
    /// The library to go through.
    pub library: Library,
}

/// A path naming the library and a school.
#[derive(Deserialize, JsonSchema)]
pub struct SchoolPath {
    /// The library to go through.
    pub library: Library,
    /// The school's id.
    pub id: Uuid,
}

/// CRUD for schools, through whichever library the path names.
#[dropshot::api_description]
pub trait SchoolApi {
    /// The server state handlers share.
    type Context;

    /// Lists every school, oldest first.
    #[endpoint { method = GET, path = "/{library}/schools" }]
    async fn list_schools(
        rqctx: RequestContext<Self::Context>,
        path: Path<LibraryPath>,
    ) -> Result<HttpResponseOk<Vec<SchoolRecord>>, HttpError>;

    /// Creates a school.
    #[endpoint { method = POST, path = "/{library}/schools" }]
    async fn create_school(
        rqctx: RequestContext<Self::Context>,
        path: Path<LibraryPath>,
        body: TypedBody<SchoolInput>,
    ) -> Result<HttpResponseCreated<SchoolRecord>, HttpError>;

    /// Fetches one school.
    #[endpoint { method = GET, path = "/{library}/schools/{id}" }]
    async fn get_school(
        rqctx: RequestContext<Self::Context>,
        path: Path<SchoolPath>,
    ) -> Result<HttpResponseOk<SchoolRecord>, HttpError>;

    /// Replaces a school's fields.
    #[endpoint { method = PUT, path = "/{library}/schools/{id}" }]
    async fn update_school(
        rqctx: RequestContext<Self::Context>,
        path: Path<SchoolPath>,
        body: TypedBody<SchoolInput>,
    ) -> Result<HttpResponseOk<SchoolRecord>, HttpError>;

    /// Deletes a school.
    #[endpoint { method = DELETE, path = "/{library}/schools/{id}" }]
    async fn delete_school(
        rqctx: RequestContext<Self::Context>,
        path: Path<SchoolPath>,
    ) -> Result<HttpResponseDeleted, HttpError>;
}
