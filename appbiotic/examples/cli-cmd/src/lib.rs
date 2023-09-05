//! # appbiotic-examples-cli-cmd
//!
//! A collection of example CLI commands.

use clap::Subcommand;

/// Example commands
#[cfg(feature = "with-greeter")]
#[derive(Subcommand)]
pub enum CliCmd {
    #[command(subcommand)]
    Greeter(appbiotic_examples_greeter::cli_cmd::CliCmd),
}
