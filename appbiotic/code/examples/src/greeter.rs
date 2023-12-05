//! # appbiotic-code-examples-greeter
//!
//! A collection of example greetings functionality.

#[cfg(feature = "commands")]
pub mod commands {
    use clap::{Args, Subcommand};

    /// Generate friendly greetings
    #[derive(Debug, Subcommand)]
    pub enum CliCmd {
        /// Prints greeting
        GetGreeting(GetGreetingArgs),
    }

    impl CliCmd {
        pub fn execute(&self) -> anyhow::Result<()> {
            match &self {
                Self::GetGreeting(args) => {
                    println!("{}", crate::greeter::get_greeting(args.name.as_deref()));
                    Ok(())
                }
            }
        }
    }

    #[derive(Debug, Args)]
    pub struct GetGreetingArgs {
        /// The greeting recipient name
        name: Option<String>,
    }
}

pub fn get_greeting<N: AsRef<str>>(name: Option<N>) -> String {
    match name {
        Some(name) => format!("Hello, {}!", name.as_ref()),
        None => "Hello, stranger.".to_string(),
    }
}

#[cfg(test)]
mod test {
    use crate::greeter::get_greeting;

    #[test]
    fn it_works() {
        let greeting = get_greeting(Some("Kris"));
        assert_eq!(&greeting, "Hello, Kris!");
    }
}
