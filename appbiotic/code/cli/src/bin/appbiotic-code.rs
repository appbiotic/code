//! # appbiotic-code-cli
//!
//! Appbiotic Code CLI is a tool to manage your coding projects.

use clap::Parser;
use tracing::{event, Level};

/// A tool to manage your coding projects.
#[derive(Parser)]
struct Cli {
    #[cfg(feature = "appbiotic-examples")]
    #[command(subcommand)]
    command: appbiotic_examples::commands::CliCmd,
}

pub fn main() -> anyhow::Result<()> {
    appbiotic_code_runtime::init_telemetry(true);
    event!(Level::TRACE, "appbiotic_code_runtime telemetry initialized");
    let cli = Cli::parse();
    cli.command.execute()
}
