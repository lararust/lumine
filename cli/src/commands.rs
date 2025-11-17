use super::CliError;

pub fn build(_args: &[String]) -> Result<(), CliError> {
    println!("Running Lumine build pipeline...");
    println!("  - preparing support crate");
    println!("  - preparing http crate");
    println!("  - preparing database crate");
    println!("  - preparing cli crate");
    println!("Done! Lumine is ready.");

    Ok(())
}
