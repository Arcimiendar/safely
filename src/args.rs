use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use clap::Parser;
use log::warn;


#[derive(Debug, Clone)]
pub struct Config {
    pub content: String,
    pub root_dir: PathBuf,
}

impl Config {
    pub fn new(content: String, root_dir: PathBuf) -> Self {
        Self { content, root_dir }
    }
}

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, default_value = ".safelyignore", value_parser = read_config)]
    pub config: Config,

    #[arg(trailing_var_arg = true)]
    pub command: Vec<String>,

    #[arg(short, long, env, default_value = "/bin/sh")]
    pub shell: String
}


fn read_config(filename: &str) -> Result<Config, String> {
    if Path::new(filename).parent().is_some_and(|p| !p.as_os_str().is_empty()) {
        return Err(format!(
            "Relative paths are not supported yet. \
            Pass a filename only; the file will be autodiscovered (got {filename})"
        ));
    }

    let start = env::current_dir()
        .map_err(|e| format!("Could not read current dir: {e}"))?;
    let mut dir = start.as_path();
    loop {
        let candidate = dir.join(filename);
        if candidate.is_file() {
            let content = fs::read_to_string(&candidate)
                .map_err(|e| format!("Could not read {}: {e}", candidate.display()))?;
            return Ok(Config::new(content, dir.to_path_buf()));
        }
        match dir.parent() {
            Some(parent) => dir = parent,
            None => return Ok(Config::new(String::new(),PathBuf::from("."))),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const MISSING: &str = ".safely-missing-test-config";

    #[test]
    fn config_short_flag() {
        let args = Args::parse_from(["safely", "-c", MISSING]);
        assert!(args.command.is_empty());
    }

    #[test]
    fn relative_path_errors() {
        let result = Args::try_parse_from(["safely", "-c", "sub/config"]);
        assert!(result.is_err());
    }

    #[test]
    fn missing_file_returns_empty_config() {
        let args = Args::parse_from(["safely", "-c", MISSING]);
        assert!(args.config.content.is_empty());
        assert_eq!(args.config.root_dir, PathBuf::from("."));
    }

    #[test]
    fn collects_trailing_args() {
        let args = Args::parse_from([
            "safely", "-c", MISSING, "echo", "hello", "world",
        ]);
        assert_eq!(args.command, vec!["echo", "hello", "world"]);
    }

    #[test]
    fn trailing_args_preserve_unknown_flags() {
        let args = Args::parse_from([
            "safely", "-c", MISSING, "cmd", "--unknown-flag", "-x",
        ]);
        assert_eq!(args.command, vec!["cmd", "--unknown-flag", "-x"]);
    }

    #[test]
    fn shell_flag() {
        let args = Args::parse_from([
            "safely", "-c", MISSING, "--shell", "fish",
        ]);
        assert_eq!(args.shell, "fish");
    }
}

