mod cli;
mod error;
mod mono;

use clap::Parser;
use cli::{Cli, Subcommand::Mono};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    init_tracing();
    let cli = Cli::parse();

    match cli.subcommand {
        Mono(opts) => mono::run(cli.shared_params, opts),
    }
    .expect("main failed, debugging with panic backtrace...");
}

pub fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let fmt_layer = fmt::layer()
        .pretty()
        .with_target(true)
        .with_line_number(true)
        .with_file(true);

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .init();
}
