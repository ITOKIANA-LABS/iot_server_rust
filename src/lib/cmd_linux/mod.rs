use std::process::{Command, Output, Stdio};
use std::io::{self, Error};

pub fn execute_command(command: &str, args: Option<&[&str]>) -> Result<Output, Error> {
    let mut command = Command::new(command);
    
    if let Some(args) = args {
        command.args(args);
    }

    let output = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    output
}

pub fn function(command: &str, args: Option<&[&str]>) -> io::Result<String> {
    let output = execute_command(command, args)?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, String::from_utf8_lossy(&output.stderr).to_string()))
    }
}