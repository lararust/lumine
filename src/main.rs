use lumine_cli::{CliError, LumineCli};

fn main() {
    if let Err(err) = LumineCli::run(std::env::args()) {
        eprintln!("{err}");

        if matches!(err, CliError::MissingCommand) {
            eprintln!("\n{}", LumineCli::help());
        }

        std::process::exit(1);
    }
}
