use crate::panel::{Action, DashboardState};

pub fn resolve_action(state: &DashboardState, key: char) -> Option<Action> {
    state
        .focused_panel()
        .and_then(|panel| panel.actions.iter().find(|action| action.key == key))
        .cloned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::HudConfig;

    #[test]
    fn resolves_action_for_focused_panel_only() {
        let config = HudConfig::from_toml(
            r#"
            title = "Test"

            [[panels]]
            id = "one"
            title = "One"
            command = "one"

            [[panels.actions]]
            key = "o"
            label = "Open one"
            command = "open-one"

            [[panels]]
            id = "two"
            title = "Two"
            command = "two"

            [[panels.actions]]
            key = "t"
            label = "Open two"
            command = "open-two"
            "#,
        )
        .expect("valid config");
        let mut state = DashboardState::from_config(&config);

        assert_eq!(
            resolve_action(&state, 'o').map(|action| action.command),
            Some("open-one".into())
        );
        assert_eq!(resolve_action(&state, 't'), None);

        state.focus_next();

        assert_eq!(resolve_action(&state, 'o'), None);
        assert_eq!(
            resolve_action(&state, 't').map(|action| action.command),
            Some("open-two".into())
        );
    }
}
