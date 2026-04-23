use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, default_value = ".safely")]
    pub config: PathBuf,

    #[arg(trailing_var_arg = true)]
    pub command: Vec<String>,

    #[arg(short, long, env, default_value = "/bin/sh")]
    pub shell: String
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_when_not_specified() {
        let args = Args::parse_from(["safely"]);
        assert_eq!(args.config, PathBuf::from(".safely"));
        assert!(args.command.is_empty());
    }

    #[test]
    fn config_long_flag() {
        let args = Args::parse_from(["safely", "--config", "/etc/safely.toml"]);
        assert_eq!(args.config, PathBuf::from("/etc/safely.toml"));
    }

    #[test]
    fn config_short_flag() {
        let args = Args::parse_from(["safely", "-c", "custom/path"]);
        assert_eq!(args.config, PathBuf::from("custom/path"));
    }

    #[test]
    fn collects_trailing_args() {
        let args = Args::parse_from(["safely", "echo", "hello", "world"]);
        assert_eq!(args.command, vec!["echo", "hello", "world"]);
        assert_eq!(args.config, PathBuf::from(".safely"));
    }

    #[test]
    fn config_flag_with_trailing_args() {
        let args = Args::parse_from(["safely", "-c", "cfg", "run", "--force"]);
        assert_eq!(args.config, PathBuf::from("cfg"));
        assert_eq!(args.command, vec!["run", "--force"]);
    }

    #[test]
    fn trailing_args_preserve_unknown_flags() {
        let args = Args::parse_from(["safely", "cmd", "--unknown-flag", "-x"]);
        assert_eq!(args.command, vec!["cmd", "--unknown-flag", "-x"]);
    }

    #[test]
    fn shell_flag() {
        let args = Args::parse_from(["safely", "--shell", "fish"]);
        assert_eq!(args.shell, "fish");
    }
}

