//! # appbiotic-code-cli
//!
//! Appbiotic Code CLI is a tool to manage your coding projects.

use clap::Parser;

/// A tool to manage your coding projects.
#[derive(Parser)]
struct Cli {
    #[cfg(feature = "with-examples")]
    #[command(subcommand)]
    command: appbiotic_examples_cli_cmd::CliCmd,
}

pub fn main() {
    let cli = Cli::parse();
    match &cli.command {
        #[cfg(feature = "with-examples-greeter")]
        appbiotic_examples_cli_cmd::CliCmd::Greeter(cmd) => cmd.execute(),
    }
}
