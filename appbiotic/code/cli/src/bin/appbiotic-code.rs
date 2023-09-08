//! # appbiotic-code-cli
//!
//! Appbiotic Code CLI is a tool to manage your coding projects.

use appbiotic_examples::CliCmd;
use clap::Parser;
use tracing::{event, Level};

/// A tool to manage your coding projects.
#[derive(Parser)]
struct Cli {
    #[cfg(feature = "with-examples")]
    #[command(subcommand)]
    command: CliCmd,
}

pub fn main() {
    appbiotic_code_runtime::init_telemetry();
    event!(Level::TRACE, "appbiotic_code_runtime telemetry initialized");

    let cli = Cli::parse();
    match &cli.command {
        #[cfg(feature = "with-examples-greeter")]
        CliCmd::Greeter(cmd) => cmd.execute(),
    }
}
