use clap::Parser;
use crate::config::Config;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, default_value = ".safely.yml", value_parser = read_config)]
    pub config: Config,

    #[arg(trailing_var_arg = true)]
    pub command: Vec<String>,

    #[arg(short, long, env, default_value = "/bin/sh")]
    pub shell: String
}


fn read_config(path: &str) -> Result<Config, String> {
    Config::from_file(path).map_err(|_err| "invalid config file provided".into())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_config() -> tempfile::NamedTempFile {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        let default_config = Config::default();
        serde_yaml_ng::to_writer(&file, &default_config).unwrap();
        file
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

