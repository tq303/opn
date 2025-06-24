use std::process::Command;

pub fn run_command(cmd: &str, context: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut command = Command::new("sh");
    command.arg("-c").arg(cmd);

    // Set the matched text as environment variable
    command.env("OPN", context);

    let status = command.status()?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("Command exited with status {}", status).into())
    }
}
