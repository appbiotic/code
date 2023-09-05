//! # appbiotic-examples-cli-cmd
//!
//! A collection of example greetings functionality.

#[cfg(feature = "with-cli-cmd")]
pub mod cli_cmd;

pub fn get_greeting<N: AsRef<str>>(name: Option<N>) -> String {
    match name {
        Some(name) => format!("Hello, {}!", name.as_ref()),
        None => "Hello, stranger.".to_string(),
    }
}

#[cfg(test)]
mod test {
    use crate::get_greeting;

    #[test]
    fn it_works() {
        let greeting = get_greeting(Some("Kris"));
        assert_eq!(&greeting, "Hello, Kris!");
    }
}
