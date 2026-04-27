use std::fs::File;
use std::io::Read;
use clap::Parser;
use log::warn;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, default_value = ".safelyignore", value_parser = read_config)]
    pub config: String,

    #[arg(trailing_var_arg = true)]
    pub command: Vec<String>,

    #[arg(short, long, env, default_value = "/bin/sh")]
    pub shell: String
}


fn read_config(path: &str) -> Result<String, String> {
    let mut file = File::open(path)
        .map_err(|_| format!("Could not open file: {}", path))?;
    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|_err| ".safelyignore file read failed".to_string())?;
    if content.is_empty() {
        warn!("Config is empty");
    }
    Ok(content)

}


#[cfg(test)]
mod tests {
    use super::*;

    fn write_config() -> tempfile::NamedTempFile {
        tempfile::NamedTempFile::new().unwrap()
    }

    #[test]
    fn config_short_flag() {
        let file = write_config();
        let args = Args::parse_from(["safely", "-c", file.path().to_str().unwrap()]);
        assert!(args.command.is_empty());
    }

    #[test]
    fn invalid_config_path_errors() {
        let result = Args::try_parse_from(["safely", "-c", "/does/not/exist.yml"]);
        assert!(result.is_err());
    }

    #[test]
    fn collects_trailing_args() {
        let file = write_config();
        let args = Args::parse_from([
            "safely",
            "-c",
            file.path().to_str().unwrap(),
            "echo",
            "hello",
            "world",
        ]);
        assert_eq!(args.command, vec!["echo", "hello", "world"]);
    }

    #[test]
    fn trailing_args_preserve_unknown_flags() {
        let file = write_config();
        let args = Args::parse_from([
            "safely",
            "-c",
            file.path().to_str().unwrap(),
            "cmd",
            "--unknown-flag",
            "-x",
        ]);
        assert_eq!(args.command, vec!["cmd", "--unknown-flag", "-x"]);
    }

    #[test]
    fn shell_flag() {
        let file = write_config();
        let args = Args::parse_from([
            "safely",
            "-c",
            file.path().to_str().unwrap(),
            "--shell",
            "fish",
        ]);
        assert_eq!(args.shell, "fish");
    }
}

