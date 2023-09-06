//! # appbiotic-examples
//!
//! A collection of example services

#[cfg(feature = "with-greeter")]
pub mod greeter;

/// Example commands
#[cfg(feature = "with-cli-cmd")]
#[derive(clap::Subcommand)]
pub enum CliCmd {
    #[cfg(feature = "with-greeter")]
    #[command(subcommand)]
    Greeter(crate::greeter::cli_cmd::CliCmd),
}
