use crate::{
    command::{CommandResult, CommandStatus},
    config::HudConfig,
};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DashboardState {
    pub title: String,
    pub panels: Vec<Panel>,
    pub focused: usize,
    pub notice: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Panel {
    pub id: String,
    pub title: String,
    pub command: String,
    pub timeout: Duration,
    pub actions: Vec<Action>,
    pub state: PanelState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Action {
    pub key: char,
    pub label: String,
    pub command: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PanelState {
    Idle,
    Loading,
    Ready { output: String },
    Error(PanelError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PanelError {
    pub message: String,
    pub detail: Option<String>,
    pub kind: PanelErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PanelErrorKind {
    ExitStatus(i32),
    TimedOut,
    LaunchFailed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusMovement {
    Next,
    Previous,
}

impl DashboardState {
    pub fn from_config(config: &HudConfig) -> Self {
        Self {
            title: config.title.clone(),
            panels: config
                .panels
                .iter()
                .map(|panel| Panel {
                    id: panel.id.clone(),
                    title: panel.title.clone(),
                    command: panel.command.clone(),
                    timeout: panel.timeout,
                    actions: panel
                        .actions
                        .iter()
                        .map(|action| Action {
                            key: action.key,
                            label: action.label.clone(),
                            command: action.command.clone(),
                        })
                        .collect(),
                    state: PanelState::Idle,
                })
                .collect(),
            focused: 0,
            notice: None,
        }
    }

    pub fn focused_panel(&self) -> Option<&Panel> {
        self.panels.get(self.focused)
    }

    pub fn focus_next(&mut self) {
        self.move_focus(FocusMovement::Next);
    }

    pub fn focus_previous(&mut self) {
        self.move_focus(FocusMovement::Previous);
    }

    pub fn move_focus(&mut self, movement: FocusMovement) {
        if self.panels.is_empty() {
            self.focused = 0;
            return;
        }

        self.focused = match movement {
            FocusMovement::Next => (self.focused + 1) % self.panels.len(),
            FocusMovement::Previous if self.focused == 0 => self.panels.len() - 1,
            FocusMovement::Previous => self.focused - 1,
        };
    }

    pub fn mark_loading(&mut self, panel_index: usize) {
        if let Some(panel) = self.panels.get_mut(panel_index) {
            panel.state = PanelState::Loading;
        }
    }

    pub fn apply_result(&mut self, panel_index: usize, result: CommandResult) {
        if let Some(panel) = self.panels.get_mut(panel_index) {
            panel.state = PanelState::from_command_result(result);
        }
    }

    pub fn set_notice(&mut self, notice: impl Into<String>) {
        self.notice = Some(notice.into());
    }

    pub fn clear_notice(&mut self) {
        self.notice = None;
    }
}

impl PanelState {
    pub fn from_command_result(result: CommandResult) -> Self {
        match result.status {
            CommandStatus::Exited(0) => PanelState::Ready {
                output: result.stdout,
            },
            CommandStatus::Exited(status) => PanelState::Error(PanelError {
                message: format!("command exited with status {status}"),
                detail: non_empty_detail(result.stderr, result.stdout),
                kind: PanelErrorKind::ExitStatus(status),
            }),
            CommandStatus::TimedOut => PanelState::Error(PanelError {
                message: "command timed out".into(),
                detail: non_empty_detail(result.stderr, result.stdout),
                kind: PanelErrorKind::TimedOut,
            }),
            CommandStatus::LaunchFailed(message) => PanelState::Error(PanelError {
                message,
                detail: non_empty_detail(result.stderr, result.stdout),
                kind: PanelErrorKind::LaunchFailed,
            }),
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            PanelState::Idle => "idle",
            PanelState::Loading => "loading",
            PanelState::Ready { .. } => "ready",
            PanelState::Error(_) => "error",
        }
    }
}

fn non_empty_detail(primary: String, fallback: String) -> Option<String> {
    let primary = primary.trim().to_string();
    if !primary.is_empty() {
        return Some(primary);
    }

    let fallback = fallback.trim().to_string();
    if !fallback.is_empty() {
        Some(fallback)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::HudConfig;

    fn sample_state() -> DashboardState {
        let config = HudConfig::from_toml(
            r#"
            title = "Test"

            [[panels]]
            id = "one"
            title = "One"
            command = "one"

            [[panels]]
            id = "two"
            title = "Two"
            command = "two"
            "#,
        )
        .expect("valid config");

        DashboardState::from_config(&config)
    }

    #[test]
    fn focus_movement_wraps_deterministically() {
        let mut state = sample_state();

        assert_eq!(state.focused, 0);
        state.focus_next();
        assert_eq!(state.focused, 1);
        state.focus_next();
        assert_eq!(state.focused, 0);
        state.focus_previous();
        assert_eq!(state.focused, 1);
    }

    #[test]
    fn panel_state_tracks_loading_ready_and_error() {
        let mut state = sample_state();

        assert_eq!(state.panels[0].state, PanelState::Idle);
        state.mark_loading(0);
        assert_eq!(state.panels[0].state, PanelState::Loading);

        state.apply_result(
            0,
            CommandResult {
                stdout: "done".into(),
                stderr: String::new(),
                status: CommandStatus::Exited(0),
            },
        );
        assert_eq!(
            state.panels[0].state,
            PanelState::Ready {
                output: "done".into()
            }
        );

        state.apply_result(
            0,
            CommandResult {
                stdout: String::new(),
                stderr: "nope".into(),
                status: CommandStatus::Exited(2),
            },
        );
        assert!(matches!(
            state.panels[0].state,
            PanelState::Error(PanelError {
                kind: PanelErrorKind::ExitStatus(2),
                ..
            })
        ));
    }
}
