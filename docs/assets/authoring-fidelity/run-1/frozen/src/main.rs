use hud::{
    app::{HudApp, run_terminal_app},
    cli::Cli,
    command::ShellCommandRunner,
    config::{HudConfig, demo_config},
};
use std::{env, process};

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let cli = Cli::parse(env::args().skip(1))?;

    if cli.help {
        hud::cli::print_help();
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
