use hud::{
    app::{HudApp, run_terminal_app},
    command::ShellCommandRunner,
    config::{HudConfig, demo_config},
};
use std::{env, path::PathBuf, process};

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let cli = Cli::parse(env::args().skip(1))?;

    if cli.help {
        print_help();
        return Ok(());
    }

    if cli.version {
        println!("hud {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let config = if cli.demo {
        demo_config()
    } else if let Some(path) = cli.config {
        HudConfig::load_from_path(path).map_err(|error| error.to_string())?
    } else {
        HudConfig::load_default().map_err(|error| error.to_string())?
    };

    if cli.check_config {
        println!("hud config OK: {} panels", config.panels.len());
        return Ok(());
    }

    let app = HudApp::new(config, ShellCommandRunner);
    run_terminal_app(app).map_err(|error| format!("terminal error: {error}"))
}

#[derive(Debug, Default)]
struct Cli {
    config: Option<PathBuf>,
    check_config: bool,
    demo: bool,
    help: bool,
    version: bool,
}

impl Cli {
    fn parse(args: impl IntoIterator<Item = String>) -> Result<Self, String> {
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

fn print_help() {
    println!(
        "hud {}\n\nUsage:\n  hud [--config <path>] [--check-config]\n  hud --demo\n  hud --version\n\nKeys:\n  q/Esc/Ctrl-C quit\n  h/j/k/l or arrows move focus\n  r refresh focused panel\n  R refresh all panels\n  panel action keys are shown in the footer",
        env!("CARGO_PKG_VERSION")
    );
}
