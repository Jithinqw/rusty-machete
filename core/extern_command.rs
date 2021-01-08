use std::process::Command;
use regex::Regex;
use std::io::Error;

fn main() -> Result<(), Error> {
    let command = Command::new("uname").arg("-a").output()?;
    if !command.status.success() {
        panic!("Some thing went wrong");
    }
    Ok(())
}