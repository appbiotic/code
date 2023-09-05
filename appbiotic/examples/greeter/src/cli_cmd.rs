use clap::{Args, Subcommand};

use crate::get_greeting;

/// Generate friendly greetings
#[derive(Subcommand)]
pub enum CliCmd {
    /// Prints greeting
    GetGreeting(GetGreetingArgs),
}

impl CliCmd {
    pub fn execute(&self) {
        match &self {
            Self::GetGreeting(args) => println!("{}", get_greeting(args.name.as_deref())),
        }
    }
}

#[derive(Args)]
pub struct GetGreetingArgs {
    /// The greeting recipient name
    name: Option<String>,
}
