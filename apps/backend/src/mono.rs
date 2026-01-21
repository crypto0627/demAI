use actix_web::{App, HttpServer, rt::System};
use clap::Parser;
use snafu::ResultExt;

use crate::{
    cli::SharedParams,
    error::{HttpServerHostSnafu, HttpServerIOSnafu, Result},
};

#[derive(Debug, Parser)]
pub struct Opts {
    /// Host string in "${HOST}:${PORT}" format.
    #[clap(long, default_value = "127.0.0.1:3000", env = "ORACLE_HOST")]
    host: String,
}

pub fn run(_shared: SharedParams, opts: Opts) -> Result<()> {
    let system = System::new();

    system.block_on(async {
        HttpServer::new(move || App::new())
            .bind(opts.host.clone())
            .context(HttpServerHostSnafu)?
            .run()
            .await
            .context(HttpServerIOSnafu)
    })
}
