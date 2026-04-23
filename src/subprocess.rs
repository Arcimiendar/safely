use std::process::Command;
use shlex::try_join;
use anyhow::Result;

pub fn run_subprocess(shell: &str, command: &[String]) -> Result<()> {
    let command = try_join(command.iter().map(|s| s.as_str()))?;
    Command::new(shell)
        .args(["-c", &command])
        .spawn()?
        .wait()?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_echo_in_sh() {
        let result = run_subprocess(
            "/bin/sh", &["echo".to_string(), "1".to_string()]
        );
        assert!(result.is_ok());
    }
}