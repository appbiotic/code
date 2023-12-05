//! # appbiotic-code-examples
//!
//! A collection of example services

/// Example commands
#[cfg(feature = "appbiotic-code-examples-greeter")]
pub mod greeter;

#[cfg(feature = "commands")]
pub mod commands {
    use clap::Subcommand;

    #[derive(Debug, Subcommand)]
    pub enum CliCmd {
        #[cfg(feature = "appbiotic-code-examples-greeter")]
        #[command(subcommand)]
        Greeter(crate::greeter::commands::CliCmd),
    }

    impl CliCmd {
        pub fn execute(&self) -> anyhow::Result<()> {
            match self {
                #[cfg(feature = "appbiotic-code-examples-greeter")]
                Self::Greeter(cmd) => cmd.execute(),
            }
        }
    }
}
