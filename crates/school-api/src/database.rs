use std::{
    env,
    sync::{Arc, Mutex, MutexGuard},
};

use anyhow::{Context as _, Result, anyhow};
use bench_common::{
    database::postgres_url,
    school::{School, SchoolInput},
};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel_async::{
    AsyncPgConnection,
    pooled_connection::{AsyncDieselConnectionManager, deadpool},
};
use diesel_demo::crud::AnyConnection;
use uuid::Uuid;

use crate::api::Library;

pub struct Database {
    diesel: Pool<ConnectionManager<AnyConnection>>,
    diesel_async: deadpool::Pool<AsyncPgConnection>,
    sqlx: sqlx::PgPool,
    seaorm: sea_orm::DatabaseConnection,
    rusqlite: Arc<Mutex<rusqlite::Connection>>,
    turso: turso::Database,
}

async fn blocking<T: Send + 'static>(
    work: impl FnOnce() -> Result<T> + Send + 'static,
) -> Result<T> {
    tokio::task::spawn_blocking(work).await?
}

fn lock(conn: &Mutex<rusqlite::Connection>) -> Result<MutexGuard<'_, rusqlite::Connection>> {
    conn.lock()
        .map_err(|_| anyhow!("the rusqlite lock is poisoned"))
}

impl Database {
    pub async fn connect() -> Result<Self> {
        let diesel_url = postgres_url("talk_diesel");
        diesel_demo::crud::connect(&diesel_url)?;
        let turso_path = env::temp_dir().join("api-turso.db");
        Ok(Self {
            diesel: Pool::new(ConnectionManager::new(&diesel_url))?,
            diesel_async: deadpool::Pool::builder(AsyncDieselConnectionManager::new(&diesel_url))
                .build()?,
            sqlx: sqlx_demo::postgres::connect(&postgres_url("talk_sqlx")).await?,
            seaorm: seaorm_demo::crud::connect(&postgres_url("talk_seaorm")).await?,
            rusqlite: Arc::new(Mutex::new(rusqlite_demo::open(
                &env::temp_dir().join("api-rusqlite.db"),
            )?)),
            turso: turso_demo::open(turso_path.to_str().context("path is not UTF-8")?).await?,
        })
    }

    pub async fn create(&self, library: Library, input: SchoolInput) -> Result<School> {
        Ok(match library {
            Library::Diesel => {
                let pool = self.diesel.clone();
                blocking(move || Ok(diesel_demo::crud::create(&mut *pool.get()?, &input)?)).await?
            }
            Library::DieselAsync => {
                let mut conn = self.diesel_async.get().await?;
                diesel_demo::crud_async::create(&mut conn, &input).await?
            }
            Library::Sqlx => sqlx_demo::postgres::create(&self.sqlx, &input).await?,
            Library::Seaorm => seaorm_demo::crud::create(&self.seaorm, &input).await?,
            Library::Rusqlite => {
                let conn = Arc::clone(&self.rusqlite);
                blocking(move || Ok(rusqlite_demo::create(&*lock(&conn)?, &input)?)).await?
            }
            Library::Turso => turso_demo::create(&self.turso.connect()?, &input).await?,
        })
    }

    pub async fn retrieve(&self, library: Library, id: Uuid) -> Result<Option<School>> {
        Ok(match library {
            Library::Diesel => {
                let pool = self.diesel.clone();
                blocking(move || Ok(diesel_demo::crud::retrieve(&mut *pool.get()?, id)?)).await?
            }
            Library::DieselAsync => {
                let mut conn = self.diesel_async.get().await?;
                diesel_demo::crud_async::retrieve(&mut conn, id).await?
            }
            Library::Sqlx => sqlx_demo::postgres::retrieve(&self.sqlx, id).await?,
            Library::Seaorm => seaorm_demo::crud::retrieve(&self.seaorm, id).await?,
            Library::Rusqlite => {
                let conn = Arc::clone(&self.rusqlite);
                blocking(move || Ok(rusqlite_demo::retrieve(&*lock(&conn)?, id)?)).await?
            }
            Library::Turso => turso_demo::retrieve(&self.turso.connect()?, id).await?,
        })
    }

    pub async fn update(
        &self,
        library: Library,
        id: Uuid,
        input: SchoolInput,
    ) -> Result<Option<School>> {
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
            Library::Sqlx => sqlx_demo::postgres::update(&self.sqlx, id, &input).await?,
            Library::Seaorm => seaorm_demo::crud::update(&self.seaorm, id, &input).await?,
            Library::Rusqlite => {
                let conn = Arc::clone(&self.rusqlite);
                blocking(move || Ok(rusqlite_demo::update(&*lock(&conn)?, id, &input)?)).await?
            }
            Library::Turso => turso_demo::update(&self.turso.connect()?, id, &input).await?,
        })
    }

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
            Library::Sqlx => sqlx_demo::postgres::delete(&self.sqlx, id).await?,
            Library::Seaorm => seaorm_demo::crud::delete(&self.seaorm, id).await?,
            Library::Rusqlite => {
                let conn = Arc::clone(&self.rusqlite);
                blocking(move || Ok(rusqlite_demo::delete(&*lock(&conn)?, id)?)).await?
            }
            Library::Turso => turso_demo::delete(&self.turso.connect()?, id).await?,
        })
    }
}
