use bench_common::school::{School, SchoolInput};
use dropshot::{
    HttpError, HttpResponseCreated, HttpResponseDeleted, HttpResponseOk, Path, RequestContext,
    TypedBody,
};
use uuid::Uuid;

use crate::{
    api::{LibraryPath, SchoolApi, SchoolPath},
    database::Database,
};

pub enum SchoolApiImpl {}

#[expect(
    clippy::needless_pass_by_value,
    reason = "used with map_err, which passes by value"
)]
fn internal(e: anyhow::Error) -> HttpError {
    HttpError::for_internal_error(format!("{e:#}"))
}

fn not_found(id: Uuid) -> HttpError {
    HttpError::for_not_found(None, format!("no school {id}"))
}

impl SchoolApi for SchoolApiImpl {
    type Context = Database;

    async fn create_school(
        rqctx: RequestContext<Database>,
        path: Path<LibraryPath>,
        body: TypedBody<SchoolInput>,
    ) -> Result<HttpResponseCreated<School>, HttpError> {
        let library = path.into_inner().library;
        let school = rqctx
            .context()
            .create(library, body.into_inner())
            .await
            .map_err(internal)?;
        Ok(HttpResponseCreated(school))
    }

    async fn retrieve_school(
        rqctx: RequestContext<Database>,
        path: Path<SchoolPath>,
    ) -> Result<HttpResponseOk<School>, HttpError> {
        let SchoolPath { library, id } = path.into_inner();
        let school = rqctx
            .context()
            .retrieve(library, id)
            .await
            .map_err(internal)?;
        school.map(HttpResponseOk).ok_or_else(|| not_found(id))
    }

    async fn update_school(
        rqctx: RequestContext<Database>,
        path: Path<SchoolPath>,
        body: TypedBody<SchoolInput>,
    ) -> Result<HttpResponseOk<School>, HttpError> {
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
