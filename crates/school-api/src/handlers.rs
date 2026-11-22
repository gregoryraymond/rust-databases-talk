//! The API trait implemented: each handler unpacks the request and hands it to [`Database`].

use bench_common::school::{SchoolInput, SchoolRecord};
use dropshot::{
    HttpError, HttpResponseCreated, HttpResponseDeleted, HttpResponseOk, Path, RequestContext,
    TypedBody,
};
use uuid::Uuid;

use crate::{
    api::{LibraryPath, SchoolApi, SchoolPath},
    database::Database,
};

/// The implementation the server runs.
pub enum SchoolApiImpl {}

/// A library failure as a 500.
#[expect(
    clippy::needless_pass_by_value,
    reason = "used as a map_err function, which passes by value"
)]
fn internal(e: anyhow::Error) -> HttpError {
    HttpError::for_internal_error(format!("{e:#}"))
}

/// A missing school as a 404.
fn not_found(id: Uuid) -> HttpError {
    HttpError::for_not_found(None, format!("no school {id}"))
}

impl SchoolApi for SchoolApiImpl {
    type Context = Database;

    async fn list_schools(
        rqctx: RequestContext<Database>,
        path: Path<LibraryPath>,
    ) -> Result<HttpResponseOk<Vec<SchoolRecord>>, HttpError> {
        let library = path.into_inner().library;
        let schools = rqctx.context().list(library).await.map_err(internal)?;
        Ok(HttpResponseOk(schools))
    }

    async fn create_school(
        rqctx: RequestContext<Database>,
        path: Path<LibraryPath>,
        body: TypedBody<SchoolInput>,
    ) -> Result<HttpResponseCreated<SchoolRecord>, HttpError> {
        let library = path.into_inner().library;
        let school = rqctx
            .context()
            .create(library, body.into_inner())
            .await
            .map_err(internal)?;
        Ok(HttpResponseCreated(school))
    }

    async fn get_school(
        rqctx: RequestContext<Database>,
        path: Path<SchoolPath>,
    ) -> Result<HttpResponseOk<SchoolRecord>, HttpError> {
        let SchoolPath { library, id } = path.into_inner();
        let school = rqctx.context().get(library, id).await.map_err(internal)?;
        school.map(HttpResponseOk).ok_or_else(|| not_found(id))
    }

    async fn update_school(
        rqctx: RequestContext<Database>,
        path: Path<SchoolPath>,
        body: TypedBody<SchoolInput>,
    ) -> Result<HttpResponseOk<SchoolRecord>, HttpError> {
        let SchoolPath { library, id } = path.into_inner();
        let school = rqctx
            .context()
            .update(library, id, body.into_inner())
            .await
            .map_err(internal)?;
        school.map(HttpResponseOk).ok_or_else(|| not_found(id))
    }

    async fn delete_school(
        rqctx: RequestContext<Database>,
        path: Path<SchoolPath>,
    ) -> Result<HttpResponseDeleted, HttpError> {
        let SchoolPath { library, id } = path.into_inner();
        if rqctx
            .context()
            .delete(library, id)
            .await
            .map_err(internal)?
        {
            Ok(HttpResponseDeleted())
        } else {
            Err(not_found(id))
        }
    }
}
