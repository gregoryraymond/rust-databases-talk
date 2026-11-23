mod api;
mod database;
mod handlers;

use std::{env, io};

use anyhow::{Result, anyhow};
use dropshot::{ConfigDropshot, ConfigLogging, ConfigLoggingLevel, ServerBuilder};

use crate::{api::school_api_mod, database::Database, handlers::SchoolApiImpl};

/// Every interface, so the dev container can forward the port.
const BIND_ADDRESS: &str = "0.0.0.0:8080";

#[tokio::main]
async fn main() -> Result<()> {
    if env::args().nth(1).as_deref() == Some("openapi") {
        school_api_mod::stub_api_description()
            .map_err(|e| anyhow!("{e}"))?
            .openapi("Schools", semver::Version::new(0, 1, 0))
            .write(&mut io::stdout())?;
        return Ok(());
    }

    let log = ConfigLogging::StderrTerminal {
        level: ConfigLoggingLevel::Info,
    }
    .to_logger("school-api")?;
    let api = school_api_mod::api_description::<SchoolApiImpl>().map_err(|e| anyhow!("{e}"))?;
    let config = ConfigDropshot {
        bind_address: BIND_ADDRESS.parse()?,
        ..ConfigDropshot::default()
    };
    ServerBuilder::new(api, Database::connect().await?, log)
        .config(config)
        .start()
        .map_err(|e| anyhow!("{e}"))?
        .await
        .map_err(|e| anyhow!(e))
}
