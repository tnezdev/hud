use crate::{
    action::resolve_action,
    command::{ActionRequest, CommandRequest, CommandResult, CommandRunner},
    config::HudConfig,
    panel::{DashboardState, FocusMovement, View},
    ui,
};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{
    io,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

pub struct HudApp<R: CommandRunner> {
    pub state: DashboardState,
    runner: R,
    refresh_tx: Sender<AppMessage>,
    refresh_rx: Receiver<AppMessage>,
}

enum AppMessage {
    PanelRefresh {
        panel_index: usize,
        result: CommandResult,
    },
    RowDetail(CommandResult),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Control {
    Continue,
    Quit,
}

impl<R: CommandRunner> HudApp<R> {
    pub fn new(config: HudConfig, runner: R) -> Self {
        let (refresh_tx, refresh_rx) = mpsc::channel();

        Self {
            state: DashboardState::from_config(&config),
            runner,
            refresh_tx,
            refresh_rx,
        }
    }

    pub fn refresh_all(&mut self) {
        for index in 0..self.state.panels.len() {
            self.spawn_refresh(index);
        }
    }

    pub fn refresh_focused(&mut self) {
        self.spawn_refresh(self.state.focused);
    }

    pub fn drain_refreshes(&mut self) {
        while let Ok(message) = self.refresh_rx.try_recv() {
            match message {
                AppMessage::PanelRefresh {
                    panel_index,
                    result,
                } => self.state.apply_result(panel_index, result),
                AppMessage::RowDetail(result) => self.state.apply_row_detail_result(result),
            }
        }
    }

    fn spawn_refresh(&mut self, panel_index: usize) {
        let Some(panel) = self.state.panels.get(panel_index).cloned() else {
            return;
        };

        self.state.mark_loading(panel_index);

        let runner = self.runner.clone();
        let tx = self.refresh_tx.clone();
        thread::spawn(move || {
            let result = runner.run(CommandRequest {
                command: panel.command,
                timeout: panel.timeout,
            });

            let _ = tx.send(AppMessage::PanelRefresh {
                panel_index,
                result,
            });
        });
    }

    fn enter_selected_row_detail(&mut self) {
        let Some(request) = self.state.enter_selected_row_detail() else {
            return;
        };

        let runner = self.runner.clone();
        let tx = self.refresh_tx.clone();
        thread::spawn(move || {
            let result = runner.run(CommandRequest {
                command: request.command,
                timeout: request.timeout,
            });

            let _ = tx.send(AppMessage::RowDetail(result));
        });
    }

    fn handle_key(&mut self, key: KeyEvent) -> Control {
        if key.kind != KeyEventKind::Press {
            return Control::Continue;
        }

        match key.code {
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => Control::Quit,
            KeyCode::Char('?') => {
                self.state.toggle_help();
                Control::Continue
            }
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('x') if self.state.help_open => {
                self.state.close_help();
                Control::Continue
            }
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('x')
                if self.state.active_view() != View::Dashboard =>
            {
                self.state.pop_view();
                Control::Continue
            }
            KeyCode::Esc | KeyCode::Char('q') => Control::Quit,
            KeyCode::Enter if self.state.active_view() == View::Dashboard => {
                self.state.enter_panel_detail();
                Control::Continue
            }
            KeyCode::Enter if self.state.active_view() == View::PanelDetail => {
                self.enter_selected_row_detail();
                Control::Continue
            }
            KeyCode::Tab => {
                self.state.move_focus(FocusMovement::Next);
                Control::Continue
            }
            KeyCode::BackTab => {
                self.state.move_focus(FocusMovement::Previous);
                Control::Continue
            }
            KeyCode::Left | KeyCode::Char('h') if self.state.active_view() == View::Dashboard => {
                self.state.move_focus(FocusMovement::Left);
                Control::Continue
            }
            KeyCode::Right | KeyCode::Char('l') if self.state.active_view() == View::Dashboard => {
                self.state.move_focus(FocusMovement::Right);
                Control::Continue
            }
            KeyCode::Up | KeyCode::Char('k') if self.state.active_view() == View::PanelDetail => {
                self.state.select_focused_row_up();
                Control::Continue
            }
            KeyCode::Down | KeyCode::Char('j') if self.state.active_view() == View::PanelDetail => {
                self.state.select_focused_row_down();
                Control::Continue
            }
            KeyCode::Up | KeyCode::Char('k') if self.state.active_view() == View::RowDetail => {
                self.state.scroll_row_detail_up();
                Control::Continue
            }
            KeyCode::Down | KeyCode::Char('j') if self.state.active_view() == View::RowDetail => {
                self.state.scroll_row_detail_down();
                Control::Continue
            }
            KeyCode::Up | KeyCode::Char('k') if self.state.active_view() == View::Dashboard => {
                self.state.move_focus(FocusMovement::Up);
                Control::Continue
            }
            KeyCode::Down | KeyCode::Char('j') if self.state.active_view() == View::Dashboard => {
                self.state.move_focus(FocusMovement::Down);
                Control::Continue
            }
            KeyCode::Char('r') => {
                self.refresh_focused();
                Control::Continue
            }
            KeyCode::Char('R') => {
                self.refresh_all();
                Control::Continue
            }
            KeyCode::Char(key) => {
                self.launch_action(key);
                Control::Continue
            }
            _ => Control::Continue,
        }
    }

    fn launch_action(&mut self, key: char) {
        let Some(action) = resolve_action(&self.state, key) else {
            return;
        };

        match self.runner.launch(ActionRequest {
            command: action.command.clone(),
        }) {
            Ok(()) => self.state.set_notice(format!(
                "launched action '{}': {}",
                action.key, action.label
            )),
            Err(error) => self.state.set_notice(error.message),
        }
    }
}

pub fn run_terminal_app<R: CommandRunner>(mut app: HudApp<R>) -> io::Result<()> {
    let _session = TerminalSession::enter()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    app.refresh_all();

    loop {
        app.drain_refreshes();
        terminal.draw(|frame| ui::draw(frame, &app.state))?;

        if event::poll(Duration::from_millis(50))? {
            let Event::Key(key) = event::read()? else {
                continue;
            };

            if app.handle_key(key) == Control::Quit {
                break;
            }
        }
    }

    Ok(())
}

struct TerminalSession;

impl TerminalSession {
    fn enter() -> io::Result<Self> {
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen)?;
        Ok(Self)
    }
}

impl Drop for TerminalSession {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::{CommandLaunchError, CommandStatus};
    use std::sync::{Arc, Mutex};

    #[derive(Clone)]
    struct FakeRunner {
        launched: Arc<Mutex<Vec<String>>>,
    }

    impl CommandRunner for FakeRunner {
        fn run(&self, _request: CommandRequest) -> CommandResult {
            CommandResult {
                stdout: "ok".into(),
                stderr: String::new(),
                status: CommandStatus::Exited(0),
            }
        }

        fn launch(&self, request: ActionRequest) -> Result<(), CommandLaunchError> {
            self.launched
                .lock()
                .expect("lock launched")
                .push(request.command);
            Ok(())
        }
    }

    #[test]
    fn action_launch_uses_injectable_runner() {
        let config = HudConfig::from_toml(
            r#"
            title = "Test"

            [[panels]]
            id = "one"
            title = "One"
            command = "one"

            [[panels.actions]]
            key = "o"
            label = "Open"
            command = "open"
            "#,
        )
        .expect("valid config");
        let launched = Arc::new(Mutex::new(Vec::new()));
        let runner = FakeRunner {
            launched: launched.clone(),
        };
        let mut app = HudApp::new(config, runner);

        app.launch_action('o');

        assert_eq!(launched.lock().expect("lock launched").as_slice(), ["open"]);
        assert!(
            app.state
                .notice
                .as_deref()
                .expect("notice")
                .contains("launched action")
        );
    }
}
