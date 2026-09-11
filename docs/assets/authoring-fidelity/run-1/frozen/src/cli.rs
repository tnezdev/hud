use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct Cli {
    pub config: Option<PathBuf>,
    pub check_config: bool,
    pub demo: bool,
    pub help: bool,
    pub version: bool,
}

impl Cli {
    pub fn parse(args: impl IntoIterator<Item = String>) -> Result<Self, String> {
        let mut cli = Cli::default();
        let mut args = args.into_iter();

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--config" => {
                    let Some(path) = args.next() else {
                        return Err("--config requires a path".into());
                    };
                    cli.config = Some(path.into());
                }
                "--check-config" => cli.check_config = true,
                "--demo" => cli.demo = true,
                "--help" | "-h" => cli.help = true,
                "--version" | "-V" => cli.version = true,
                unknown => return Err(format!("unknown argument: {unknown}")),
            }
        }

        Ok(cli)
    }
}

pub fn print_help() {
    println!(
        "hud {}\n\nUsage:\n  hud [--config <path>] [--check-config]\n  hud --demo\n  hud --version\n\nKeys:\n  q/Esc/Ctrl-C quit\n  h/j/k/l or arrows move focus\n  r refresh focused panel\n  R refresh all panels\n  panel action keys are shown in the footer",
        env!("CARGO_PKG_VERSION")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_args() {
        let cli = Cli::parse([] as [String; 0]).unwrap();
        assert!(cli.config.is_none());
        assert!(!cli.check_config);
        assert!(!cli.demo);
        assert!(!cli.help);
        assert!(!cli.version);
    }

    #[test]
    fn parse_config_with_path() {
        let cli = Cli::parse(["--config".into(), "my.toml".into()]).unwrap();
        assert_eq!(cli.config.unwrap().to_str().unwrap(), "my.toml");
    }

    #[test]
    fn parse_config_without_path_returns_error() {
        let err = Cli::parse(["--config".into()]).unwrap_err();
        assert!(err.contains("--config requires a path"));
    }

    #[test]
    fn parse_unknown_argument_returns_error() {
        let err = Cli::parse(["--bogus".into()]).unwrap_err();
        assert!(err.contains("unknown argument"));
    }

    #[test]
    fn parse_flags() {
        let cli = Cli::parse([
            "--demo".into(),
            "--check-config".into(),
            "--help".into(),
            "--version".into(),
        ])
        .unwrap();
        assert!(cli.demo);
        assert!(cli.check_config);
        assert!(cli.help);
        assert!(cli.version);
    }
}
