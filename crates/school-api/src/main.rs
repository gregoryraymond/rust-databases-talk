//! A Dropshot CRUD API for schools. `/{library}/schools` runs through Diesel, diesel-async,
//! sqlx, SeaORM, rusqlite or Turso, so the same request can be compared across them.
//!
//! With `openapi` as the argument, prints the OpenAPI document built from the API trait instead.

mod api;
mod database;
mod handlers;
mod settings;

use std::{env, io};

use anyhow::{Context as _, Result, anyhow};
use dropshot::{ConfigDropshot, ConfigLogging, ConfigLoggingLevel, ServerBuilder};

use crate::{
    api::school_api_mod,
    database::Database,
    handlers::SchoolApiImpl,
    settings::{API_TITLE, BIND_ADDRESS, OPENAPI_COMMAND},
};

#[tokio::main]
async fn main() -> Result<()> {
    if env::args().nth(1).as_deref() == Some(OPENAPI_COMMAND) {
        school_api_mod::stub_api_description()
            .map_err(|e| anyhow!("{e}"))?
            .openapi(API_TITLE, semver::Version::new(0, 1, 0))
            .write(&mut io::stdout())
            .context("writing the OpenAPI document")?;
        return Ok(());
    }

    let log = ConfigLogging::StderrTerminal {
        level: ConfigLoggingLevel::Info,
    }
    .to_logger("school-api")
    .context("starting the logger")?;
    let api = school_api_mod::api_description::<SchoolApiImpl>().map_err(|e| anyhow!("{e}"))?;
    let config = ConfigDropshot {
        bind_address: BIND_ADDRESS.parse().context("parsing the bind address")?,
        ..ConfigDropshot::default()
    };
    let database = Database::connect().await?;
    ServerBuilder::new(api, database, log)
        .config(config)
        .start()
        .map_err(|e| anyhow!("{e}"))?
        .await
        .map_err(|e| anyhow!(e))
}
