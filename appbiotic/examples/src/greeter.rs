//! # appbiotic-examples-cli-cmd
//!
//! A collection of example greetings functionality.

#[cfg(feature = "with-cli-cmd")]
pub mod cli_cmd {

    /// Generate friendly greetings
    #[derive(clap::Subcommand)]
    pub enum CliCmd {
        /// Prints greeting
        GetGreeting(GetGreetingArgs),
    }

    #[cfg(feature = "with-cli-cmd")]
    impl CliCmd {
        pub fn execute(&self) {
            match &self {
                Self::GetGreeting(args) => {
                    println!("{}", crate::greeter::get_greeting(args.name.as_deref()))
                }
            }
        }
    }

    #[cfg(feature = "with-cli-cmd")]
    #[derive(clap::Args)]
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
