use std::error::Error;
use std::fmt;

mod commands;

pub struct LumineCli;

#[derive(Debug, PartialEq, Eq)]
pub enum CliError {
    MissingCommand,
    UnknownCommand(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::MissingCommand => write!(f, "no command provided"),
            CliError::UnknownCommand(cmd) => write!(f, "unknown command `{}`", cmd),
        }
    }
}

impl Error for CliError {}

impl LumineCli {
    pub fn run<I>(args: I) -> Result<(), CliError>
    where
        I: IntoIterator<Item = String>,
    {
        let mut args = args.into_iter();
        let _program = args.next();

        match args.next() {
            Some(cmd) => Self::dispatch(cmd, args.collect()),
            None => Err(CliError::MissingCommand),
        }
    }

    fn dispatch(cmd: String, tail: Vec<String>) -> Result<(), CliError> {
        match cmd.as_str() {
            "build" => commands::build(&tail),
            "help" | "--help" | "-h" => {
                println!("{}", Self::help());
                Ok(())
            }
            _ => Err(CliError::UnknownCommand(cmd)),
        }
    }

    pub fn help() -> &'static str {
        "Lumine CLI\n\nUsage:\n    cargo run -- <command>\n\nAvailable commands:\n    build   Run the lightweight Lumine build pipeline\n    help    Display this help text"
    }
}

#[cfg(test)]
mod tests {
    use super::{CliError, LumineCli};

    #[test]
    fn returns_error_when_missing_command() {
        let result = LumineCli::run(["lumine".to_string()]);
        assert_eq!(result.unwrap_err(), CliError::MissingCommand);
    }

    #[test]
    fn runs_build_command() {
        let args = ["lumine", "build"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert!(LumineCli::run(args).is_ok());
    }
}
