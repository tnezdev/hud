//! Fixed film-night inspection fixture, not a general export or command runner.
use hud::{
    command::{CommandResult, CommandStatus},
    config::HudConfig,
    panel::{DashboardState, PanelState},
    ui,
};
use ratatui::{Terminal, backend::TestBackend};
use std::{env, process::ExitCode};

const PAYLOADS: [(&str, &str); 5] = [
    (
        "cat examples/film-night/organizer-lead.txt",
        include_str!("film-night/organizer-lead.txt"),
    ),
    (
        "cat examples/film-night/organizer-explain.txt",
        include_str!("film-night/organizer-explain.txt"),
    ),
    (
        "cat examples/film-night/alternatives.json",
        include_str!("film-night/alternatives.json"),
    ),
    (
        "cat examples/film-night/guests-lead.txt",
        include_str!("film-night/guests-lead.txt"),
    ),
    (
        "cat examples/film-night/guests-direct.txt",
        include_str!("film-night/guests-direct.txt"),
    ),
];

fn story(audience: &str) -> Result<DashboardState, String> {
    let source = match audience {
        "organizer" => include_str!("film-night-organizer.toml"),
        "guests" => include_str!("film-night-guests.toml"),
        _ => return Err("audience must be organizer or guests".into()),
    };
    let config = HudConfig::from_toml(source).map_err(|error| error.to_string())?;
    let mut state = DashboardState::from_config(&config);
    for (index, panel) in config.panels.iter().enumerate() {
        let (_, stdout) = PAYLOADS
            .iter()
            .find(|(command, _)| *command == panel.command)
            .ok_or_else(|| format!("unrecognized fixture command for {}", panel.id))?;
        state.apply_result(
            index,
            CommandResult {
                stdout: (*stdout).into(),
                stderr: String::new(),
                status: CommandStatus::Exited(0),
            },
        );
        if !matches!(state.panels[index].state, PanelState::Ready { .. }) {
            return Err(format!("invalid fixture output for {}", panel.id));
        }
    }
    Ok(state)
}

fn preview(
    audience: &str,
    width: u16,
    height: u16,
    detail: Option<&str>,
) -> Result<String, String> {
    if !(20..=240).contains(&width) || !(10..=100).contains(&height) {
        return Err("fixture dimensions must be 20..240 columns and 10..100 rows".into());
    }
    let mut state = story(audience)?;
    if let Some(id) = detail {
        state.focused = state
            .panels
            .iter()
            .position(|panel| panel.id == id)
            .ok_or_else(|| format!("unknown detail panel: {id}"))?;
        state.enter_panel_detail();
    }
    let mut terminal =
        Terminal::new(TestBackend::new(width, height)).map_err(|error| error.to_string())?;
    terminal
        .draw(|frame| ui::draw(frame, &state))
        .map_err(|error| error.to_string())?;
    let mut output = String::new();
    for row in terminal.backend().buffer().content().chunks(width as usize) {
        let line: String = row.iter().map(|cell| cell.symbol()).collect();
        output.push_str(line.trim_end());
        output.push('\n');
    }
    Ok(output)
}

fn run(args: &[String]) -> Result<String, String> {
    if !(3..=4).contains(&args.len()) {
        return Err("usage: story_preview organizer|guests WIDTH HEIGHT [PANEL_ID]".into());
    }
    let width = args[1].parse().map_err(|_| "invalid width")?;
    let height = args[2].parse().map_err(|_| "invalid height")?;
    preview(&args[0], width, height, args.get(3).map(String::as_str))
}

fn main() -> ExitCode {
    match run(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok(output) => {
            print!("{output}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixtures_parse_the_same_payloads_used_by_the_terminal_configs() {
        assert_eq!(story("organizer").unwrap().panels.len(), 3);
        assert_eq!(story("guests").unwrap().panels.len(), 2);
    }

    #[test]
    fn both_audiences_render_at_roomy_and_constrained_sizes() {
        for (width, height) in [(120, 40), (80, 24)] {
            let organizer = preview("organizer", width, height, None).unwrap();
            assert_eq!(organizer.lines().count(), height as usize);
            assert!(organizer.contains("Move tonight's"));
            assert!(organizer.contains("Courtyard"));
            assert!(organizer.contains("Hall"));
            assert!(organizer.contains("Rain cover"));
            assert!(organizer.contains("19:00"));
            assert!(organizer.contains("Forecast: supplied, not live."));
            assert!(organizer.contains("Fictional planning exercise."));

            let guests = preview("guests", width, height, None).unwrap();
            assert_eq!(guests.lines().count(), height as usize);
            assert!(guests.contains("Film night is in the hall."));
            assert!(guests.contains("main entrance"));
            assert!(guests.contains("Same 19:00 start."));
            assert!(!guests.contains("The alternatives"));
            assert!(!guests.contains("Rain cover"));
        }
    }

    #[test]
    fn comparison_detail_uses_the_production_detail_view() {
        let detail = preview("organizer", 80, 24, Some("compare")).unwrap();
        assert!(detail.contains("DETAIL / THE ALTERNATIVES"));
        for value in ["Place", "Rain cover", "Start", "Courtyard", "Hall", "19:00"] {
            assert!(detail.contains(value), "missing {value}");
        }
        assert!(!detail.contains("Why move?"));
    }

    #[test]
    fn preview_rejects_unknown_fixtures_panels_and_invalid_dimensions() {
        assert!(story("untrusted-path").is_err());
        assert!(preview("guests", 80, 24, Some("compare")).is_err());
        assert!(preview("guests", 0, 24, None).is_err());
        assert!(preview("guests", 80, 101, None).is_err());
        assert!(run(&[]).is_err());
        assert!(run(&["guests".into(), "wide".into(), "24".into()]).is_err());
    }
}
