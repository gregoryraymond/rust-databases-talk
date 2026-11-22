//! One pool or connection per library, and the CRUD calls that pick between them.
//!
//! Blocking libraries run on Tokio's blocking threads, so they never stall the async workers.

use std::{
    env,
    path::PathBuf,
    sync::{Arc, Mutex, MutexGuard},
};

use anyhow::{Context as _, Result, anyhow};
use bench_common::{
    database::postgres_url,
    postgis,
    school::{SchoolInput, SchoolRecord},
};
use diesel_demo::{crud::PgPool as DieselPool, crud_async::AsyncPgPool};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::{
    api::Library,
    settings::{POOL_SIZE, RUSQLITE_DATABASE, TURSO_DATABASE},
};

/// Every library's handle on its database.
pub struct Database {
    /// Diesel's blocking pool.
    diesel: DieselPool,
    /// diesel-async's pool.
    diesel_async: AsyncPgPool,
    /// sqlx's pool.
    sqlx: sqlx::PgPool,
    /// SeaORM's connection, itself a sqlx pool.
    seaorm: DatabaseConnection,
    /// rusqlite's single connection: SQLite has one writer anyway.
    rusqlite: Arc<Mutex<rusqlite::Connection>>,
    /// Turso's database, which hands out a connection per request.
    turso: turso::Database,
}

/// Runs blocking work on Tokio's blocking threads.
async fn blocking<T: Send + 'static>(
    work: impl FnOnce() -> Result<T> + Send + 'static,
) -> Result<T> {
    tokio::task::spawn_blocking(work)
        .await
        .context("the blocking task panicked")?
}

/// Takes the rusqlite connection's lock.
fn lock(conn: &Mutex<rusqlite::Connection>) -> Result<MutexGuard<'_, rusqlite::Connection>> {
    conn.lock()
        .map_err(|_| anyhow!("the SQLite connection lock is poisoned"))
}

/// Where a SQLite database file named `name` lives: kept between runs, unlike the benchmarks'.
fn sqlite_path(name: &str) -> PathBuf {
    env::temp_dir().join(format!("rust-databases-talk-{name}.db"))
}

impl Database {
    /// Connects every library, creating the school tables if they are missing.
    ///
    /// # Errors
    ///
    /// Fails if any database cannot be reached or set up.
    pub async fn connect() -> Result<Self> {
        let url = postgres_url();
        let sqlx = sqlx_demo::crud::pool(&url, POOL_SIZE)
            .await
            .context("connecting sqlx")?;
        for sql in postgis::CREATE_IF_MISSING {
            sqlx::query(*sql)
                .execute(&sqlx)
                .await
                .with_context(|| format!("running {sql}"))?;
        }
        let diesel_url = url.clone();
        let diesel = blocking(move || Ok(diesel_demo::crud::pool(&diesel_url)?)).await?;
        let diesel_async =
            diesel_demo::crud_async::pool(&url).context("building diesel-async's pool")?;
        let seaorm = seaorm_demo::crud::connect(&url, POOL_SIZE)
            .await
            .context("connecting SeaORM")?;
        let rusqlite =
            blocking(|| Ok(rusqlite_demo::crud::open(&sqlite_path(RUSQLITE_DATABASE))?)).await?;
        let turso_path = sqlite_path(TURSO_DATABASE);
        let turso_path = turso_path.to_str().context("temp path is not UTF-8")?;
        let turso = turso_demo::crud::open(turso_path).await?;
        Ok(Self {
            diesel,
            diesel_async,
            sqlx,
            seaorm,
            rusqlite: Arc::new(Mutex::new(rusqlite)),
            turso,
        })
    }

    /// Creates a school through `library`.
    ///
    /// # Errors
    ///
    /// Fails if the library does.
    pub async fn create(&self, library: Library, input: SchoolInput) -> Result<SchoolRecord> {
        Ok(match library {
            Library::Diesel => {
                let pool = self.diesel.clone();
                blocking(move || Ok(diesel_demo::crud::create(&mut *pool.get()?, &input)?)).await?
            }
            Library::DieselAsync => {
                let mut conn = self.diesel_async.get().await?;
                diesel_demo::crud_async::create(&mut conn, &input).await?
            }
            Library::Sqlx => sqlx_demo::crud::create(&self.sqlx, &input).await?,
            Library::Seaorm => seaorm_demo::crud::create(&self.seaorm, &input).await?,
            Library::Rusqlite => {
                let conn = Arc::clone(&self.rusqlite);
                blocking(move || Ok(rusqlite_demo::crud::create(&*lock(&conn)?, &input)?)).await?
            }
            Library::Turso => turso_demo::crud::create(&self.turso.connect()?, &input).await?,
        })
    }

    /// One school through `library`.
    ///
    /// # Errors
    ///
    /// Fails if the library does.
    pub async fn get(&self, library: Library, id: Uuid) -> Result<Option<SchoolRecord>> {
        Ok(match library {
            Library::Diesel => {
                let pool = self.diesel.clone();
                blocking(move || Ok(diesel_demo::crud::get(&mut *pool.get()?, id)?)).await?
            }
            Library::DieselAsync => {
                let mut conn = self.diesel_async.get().await?;
                diesel_demo::crud_async::get(&mut conn, id).await?
            }
            Library::Sqlx => sqlx_demo::crud::get(&self.sqlx, id).await?,
            Library::Seaorm => seaorm_demo::crud::get(&self.seaorm, id).await?,
            Library::Rusqlite => {
                let conn = Arc::clone(&self.rusqlite);
                blocking(move || Ok(rusqlite_demo::crud::get(&*lock(&conn)?, id)?)).await?
            }
            Library::Turso => turso_demo::crud::get(&self.turso.connect()?, id).await?,
        })
    }

    /// Every school through `library`.
    ///
    /// # Errors
    ///
    /// Fails if the library does.
    pub async fn list(&self, library: Library) -> Result<Vec<SchoolRecord>> {
        Ok(match library {
            Library::Diesel => {
                let pool = self.diesel.clone();
                blocking(move || Ok(diesel_demo::crud::list(&mut *pool.get()?)?)).await?
            }
            Library::DieselAsync => {
                let mut conn = self.diesel_async.get().await?;
                diesel_demo::crud_async::list(&mut conn).await?
            }
            Library::Sqlx => sqlx_demo::crud::list(&self.sqlx).await?,
            Library::Seaorm => seaorm_demo::crud::list(&self.seaorm).await?,
            Library::Rusqlite => {
                let conn = Arc::clone(&self.rusqlite);
                blocking(move || Ok(rusqlite_demo::crud::list(&*lock(&conn)?)?)).await?
            }
            Library::Turso => turso_demo::crud::list(&self.turso.connect()?).await?,
        })
    }

    /// Replaces a school's fields through `library`.
    ///
    /// # Errors
    ///
    /// Fails if the library does.
    pub async fn update(
        &self,
        library: Library,
        id: Uuid,
        input: SchoolInput,
    ) -> Result<Option<SchoolRecord>> {
        Ok(match library {
            Library::Diesel => {
                let pool = self.diesel.clone();
                blocking(move || Ok(diesel_demo::crud::update(&mut *pool.get()?, id, &input)?))
                    .await?
            }
            Library::DieselAsync => {
                let mut conn = self.diesel_async.get().await?;
                diesel_demo::crud_async::update(&mut conn, id, &input).await?
            }
            Library::Sqlx => sqlx_demo::crud::update(&self.sqlx, id, &input).await?,
            Library::Seaorm => seaorm_demo::crud::update(&self.seaorm, id, &input).await?,
            Library::Rusqlite => {
                let conn = Arc::clone(&self.rusqlite);
                blocking(move || Ok(rusqlite_demo::crud::update(&*lock(&conn)?, id, &input)?))
                    .await?
            }
            Library::Turso => turso_demo::crud::update(&self.turso.connect()?, id, &input).await?,
        })
    }

    /// Deletes a school through `library`, returning whether there was one.
    ///
    /// # Errors
    ///
    /// Fails if the library does.
    pub async fn delete(&self, library: Library, id: Uuid) -> Result<bool> {
        Ok(match library {
            Library::Diesel => {
                let pool = self.diesel.clone();
                blocking(move || Ok(diesel_demo::crud::delete(&mut *pool.get()?, id)?)).await?
            }
            Library::DieselAsync => {
                let mut conn = self.diesel_async.get().await?;
                diesel_demo::crud_async::delete(&mut conn, id).await?
            }
            Library::Sqlx => sqlx_demo::crud::delete(&self.sqlx, id).await?,
            Library::Seaorm => seaorm_demo::crud::delete(&self.seaorm, id).await?,
            Library::Rusqlite => {
                let conn = Arc::clone(&self.rusqlite);
                blocking(move || Ok(rusqlite_demo::crud::delete(&*lock(&conn)?, id)?)).await?
            }
            Library::Turso => turso_demo::crud::delete(&self.turso.connect()?, id).await?,
        })
    }
}
