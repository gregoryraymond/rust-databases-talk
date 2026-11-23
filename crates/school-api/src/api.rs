use bench_common::school::{School, SchoolInput};
use dropshot::{
    HttpError, HttpResponseCreated, HttpResponseDeleted, HttpResponseOk, Path, RequestContext,
    TypedBody,
};
use schemars::JsonSchema;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum Library {
    Diesel,
    DieselAsync,
    Sqlx,
    Seaorm,
    Rusqlite,
    Turso,
}

#[derive(Deserialize, JsonSchema)]
pub struct LibraryPath {
    pub library: Library,
}

#[derive(Deserialize, JsonSchema)]
pub struct SchoolPath {
    pub library: Library,
    pub id: Uuid,
}

#[dropshot::api_description]
pub trait SchoolApi {
    type Context;

    #[endpoint { method = POST, path = "/{library}/schools" }]
    async fn create_school(
        rqctx: RequestContext<Self::Context>,
        path: Path<LibraryPath>,
        body: TypedBody<SchoolInput>,
    ) -> Result<HttpResponseCreated<School>, HttpError>;

    #[endpoint { method = GET, path = "/{library}/schools/{id}" }]
    async fn retrieve_school(
        rqctx: RequestContext<Self::Context>,
        path: Path<SchoolPath>,
    ) -> Result<HttpResponseOk<School>, HttpError>;

    #[endpoint { method = PUT, path = "/{library}/schools/{id}" }]
    async fn update_school(
        rqctx: RequestContext<Self::Context>,
        path: Path<SchoolPath>,
        body: TypedBody<SchoolInput>,
    ) -> Result<HttpResponseOk<School>, HttpError>;

    #[endpoint { method = DELETE, path = "/{library}/schools/{id}" }]
    async fn delete_school(
        rqctx: RequestContext<Self::Context>,
        path: Path<SchoolPath>,
    ) -> Result<HttpResponseDeleted, HttpError>;
}
