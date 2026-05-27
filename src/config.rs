use serde::Deserialize;
use std::{
    collections::HashSet,
    env, fmt, fs,
    path::{Path, PathBuf},
    time::Duration,
};

pub const DEFAULT_TIMEOUT_SECS: u64 = 120;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HudConfig {
    pub title: String,
    pub default_timeout: Duration,
    pub panels: Vec<PanelConfig>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PanelConfig {
    pub id: String,
    pub title: String,
    pub command: String,
    pub output_format: OutputFormat,
    pub timeout: Duration,
    pub actions: Vec<ActionConfig>,
    pub row_detail: Option<RowDetailConfig>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowDetailConfig {
    pub title: String,
    pub command: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Text,
    TableJson,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionConfig {
    pub key: char,
    pub label: String,
    pub command: String,
}

#[derive(Debug)]
pub enum ConfigError {
    MissingPathContext,
    Missing {
        path: PathBuf,
    },
    Read {
        path: PathBuf,
        message: String,
    },
    Parse {
        path: Option<PathBuf>,
        message: String,
    },
    Validation {
        errors: Vec<String>,
    },
}

impl HudConfig {
    pub fn load_default() -> Result<Self, ConfigError> {
        Self::load_from_path(default_config_path()?)
    }

    pub fn load_from_path(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        if !path.exists() {
            return Err(ConfigError::Missing {
                path: path.to_path_buf(),
            });
        }

        let contents = fs::read_to_string(path).map_err(|error| ConfigError::Read {
            path: path.to_path_buf(),
            message: error.to_string(),
        })?;

        Self::from_toml_with_path(&contents, Some(path.to_path_buf()))
    }

    pub fn from_toml(input: &str) -> Result<Self, ConfigError> {
        Self::from_toml_with_path(input, None)
    }

    fn from_toml_with_path(input: &str, path: Option<PathBuf>) -> Result<Self, ConfigError> {
        let raw: RawHudConfig = toml::from_str(input).map_err(|error| ConfigError::Parse {
            path,
            message: error.to_string(),
        })?;

        raw.validate()
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::MissingPathContext => {
                write!(
                    formatter,
                    "could not determine config path because neither XDG_CONFIG_HOME nor HOME is set"
                )
            }
            ConfigError::Missing { path } => write!(
                formatter,
                "missing hud config at {}. Create it or pass --config <path>.",
                path.display()
            ),
            ConfigError::Read { path, message } => {
                write!(
                    formatter,
                    "could not read config at {}: {message}",
                    path.display()
                )
            }
            ConfigError::Parse { path, message } => {
                if let Some(path) = path {
                    write!(formatter, "invalid TOML in {}: {message}", path.display())
                } else {
                    write!(formatter, "invalid TOML: {message}")
                }
            }
            ConfigError::Validation { errors } => {
                write!(formatter, "invalid hud config: {}", errors.join("; "))
            }
        }
    }
}

impl std::error::Error for ConfigError {}

pub fn default_config_path() -> Result<PathBuf, ConfigError> {
    if let Some(config_home) = env::var_os("XDG_CONFIG_HOME").filter(|value| !value.is_empty()) {
        return Ok(PathBuf::from(config_home).join(".hud").join("config.toml"));
    }

    if let Some(home) = env::var_os("HOME").filter(|value| !value.is_empty()) {
        return Ok(PathBuf::from(home)
            .join(".config")
            .join(".hud")
            .join("config.toml"));
    }

    Err(ConfigError::MissingPathContext)
}

pub fn demo_config() -> HudConfig {
    HudConfig::from_toml(
        r#"
        title = "hud demo"
        default_timeout_secs = 10

        [[panels]]
        id = "orientation"
        title = "Orientation"
        command = "printf 'hud demo\n\nr refresh panel\nR refresh all\nq quit\n'"

        [[panels.actions]]
        key = "e"
        label = "Print environment"
        command = "env | sort | head -20"

        [[panels]]
        id = "calendar"
        title = "Now"
        command = "date"
        "#,
    )
    .expect("built-in demo config is valid")
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawHudConfig {
    title: Option<String>,
    default_timeout_secs: Option<u64>,
    #[serde(default)]
    panels: Vec<RawPanelConfig>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawPanelConfig {
    id: Option<String>,
    title: Option<String>,
    command: Option<String>,
    output: Option<RawOutputFormat>,
    timeout_secs: Option<u64>,
    row_detail: Option<RawRowDetailConfig>,
    #[serde(default)]
    actions: Vec<RawActionConfig>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRowDetailConfig {
    title: Option<String>,
    command: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawActionConfig {
    key: Option<String>,
    label: Option<String>,
    command: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum RawOutputFormat {
    Text,
    TableJson,
}

impl RawHudConfig {
    fn validate(self) -> Result<HudConfig, ConfigError> {
        let mut errors = Vec::new();

        let title = required_text("title", self.title, &mut errors);
        let default_timeout_secs = self.default_timeout_secs.unwrap_or(DEFAULT_TIMEOUT_SECS);
        validate_timeout("default_timeout_secs", default_timeout_secs, &mut errors);

        if self.panels.is_empty() {
            errors.push("at least one [[panels]] entry is required".into());
        }

        let mut panel_ids = HashSet::new();
        let mut panels = Vec::new();
        for (index, panel) in self.panels.into_iter().enumerate() {
            let label = format!("panels[{index}]");
            let id = required_text(&format!("{label}.id"), panel.id, &mut errors);
            let panel_title = required_text(&format!("{label}.title"), panel.title, &mut errors);
            let command = required_text(&format!("{label}.command"), panel.command, &mut errors);

            if let Some(id) = &id
                && !panel_ids.insert(id.clone())
            {
                errors.push(format!("duplicate panel id '{id}'"));
            }

            let timeout_secs = panel.timeout_secs.unwrap_or(default_timeout_secs);
            validate_timeout(&format!("{label}.timeout_secs"), timeout_secs, &mut errors);

            let actions = validate_actions(&label, panel.actions, &mut errors);
            let row_detail = validate_row_detail(&label, panel.row_detail, &mut errors);

            if let (Some(id), Some(title), Some(command)) = (id, panel_title, command) {
                panels.push(PanelConfig {
                    id,
                    title,
                    command,
                    output_format: panel
                        .output
                        .map(OutputFormat::from)
                        .unwrap_or(OutputFormat::Text),
                    timeout: Duration::from_secs(timeout_secs),
                    actions,
                    row_detail,
                });
            }
        }

        if !errors.is_empty() {
            return Err(ConfigError::Validation { errors });
        }

        Ok(HudConfig {
            title: title.expect("title is present after validation"),
            default_timeout: Duration::from_secs(default_timeout_secs),
            panels,
        })
    }
}

fn validate_row_detail(
    panel_label: &str,
    row_detail: Option<RawRowDetailConfig>,
    errors: &mut Vec<String>,
) -> Option<RowDetailConfig> {
    let row_detail = row_detail?;
    let label = format!("{panel_label}.row_detail");
    let title = required_text(&format!("{label}.title"), row_detail.title, errors);
    let command = required_text(&format!("{label}.command"), row_detail.command, errors);

    match (title, command) {
        (Some(title), Some(command)) => Some(RowDetailConfig { title, command }),
        _ => None,
    }
}

impl From<RawOutputFormat> for OutputFormat {
    fn from(value: RawOutputFormat) -> Self {
        match value {
            RawOutputFormat::Text => OutputFormat::Text,
            RawOutputFormat::TableJson => OutputFormat::TableJson,
        }
    }
}

fn validate_actions(
    panel_label: &str,
    actions: Vec<RawActionConfig>,
    errors: &mut Vec<String>,
) -> Vec<ActionConfig> {
    let mut action_keys = HashSet::new();
    let mut valid_actions = Vec::new();

    for (index, action) in actions.into_iter().enumerate() {
        let label = format!("{panel_label}.actions[{index}]");
        let key = action.key.and_then(|key| {
            let mut chars = key.chars();
            let first = chars.next();
            if first.is_none() || chars.next().is_some() {
                errors.push(format!("{label}.key must be exactly one character"));
                None
            } else {
                first
            }
        });
        let action_label = required_text(&format!("{label}.label"), action.label, errors);
        let command = required_text(&format!("{label}.command"), action.command, errors);

        if let Some(key) = key
            && !action_keys.insert(key)
        {
            errors.push(format!("duplicate action key '{key}' in {panel_label}"));
        }

        if let (Some(key), Some(label), Some(command)) = (key, action_label, command) {
            valid_actions.push(ActionConfig {
                key,
                label,
                command,
            });
        }
    }

    valid_actions
}

fn required_text(field: &str, value: Option<String>, errors: &mut Vec<String>) -> Option<String> {
    match value.map(|value| value.trim().to_string()) {
        Some(value) if !value.is_empty() => Some(value),
        _ => {
            errors.push(format!("{field} is required"));
            None
        }
    }
}

fn validate_timeout(field: &str, value: u64, errors: &mut Vec<String>) {
    if value == 0 {
        errors.push(format!("{field} must be greater than zero"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_config() {
        let config = HudConfig::from_toml(
            r#"
            title = "Workspace"
            default_timeout_secs = 30

            [[panels]]
            id = "tasks"
            title = "Tasks"
            command = "task mine"
            output = "table-json"
            timeout_secs = 5

            [panels.row_detail]
            title = "Task detail"
            command = "task {{ID}}"

            [[panels.actions]]
            key = "t"
            label = "Open tasks"
            command = "taskwarrior-tui"
            "#,
        )
        .expect("valid config");

        assert_eq!(config.title, "Workspace");
        assert_eq!(config.default_timeout, Duration::from_secs(30));
        assert_eq!(config.panels[0].timeout, Duration::from_secs(5));
        assert_eq!(config.panels[0].output_format, OutputFormat::TableJson);
        assert_eq!(
            config.panels[0].row_detail,
            Some(RowDetailConfig {
                title: "Task detail".into(),
                command: "task {{ID}}".into()
            })
        );
        assert_eq!(config.panels[0].actions[0].key, 't');
    }

    #[test]
    fn reports_missing_required_fields() {
        let error = HudConfig::from_toml(
            r#"
            title = "Broken"

            [[panels]]
            id = "tasks"
            title = "Tasks"
            "#,
        )
        .expect_err("missing command is invalid");

        assert!(error.to_string().contains("panels[0].command is required"));
    }

    #[test]
    fn reports_invalid_shapes() {
        let error = HudConfig::from_toml(
            r#"
            title = "Broken"
            panels = "not a list"
            "#,
        )
        .expect_err("invalid TOML shape");

        assert!(error.to_string().contains("invalid TOML"));
    }

    #[test]
    fn rejects_invalid_action_key() {
        let error = HudConfig::from_toml(
            r#"
            title = "Broken"

            [[panels]]
            id = "tasks"
            title = "Tasks"
            command = "task mine"

            [[panels.actions]]
            key = "go"
            label = "Go"
            command = "echo go"
            "#,
        )
        .expect_err("multi-character action key is invalid");

        assert!(
            error
                .to_string()
                .contains("key must be exactly one character")
        );
    }

    #[test]
    fn missing_config_path_is_clear() {
        let path = env::temp_dir().join("hud-test-missing-config.toml");
        let error = HudConfig::load_from_path(&path).expect_err("missing file is invalid");

        assert!(error.to_string().contains("missing hud config"));
        assert!(error.to_string().contains(path.to_string_lossy().as_ref()));
    }
}
