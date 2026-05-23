use std::{
    env, fs,
    fs::File,
    path::PathBuf,
    process::{Command, Stdio},
    sync::atomic::{AtomicU64, Ordering},
    thread,
    time::{Duration, Instant},
};

static CAPTURE_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandRequest {
    pub command: String,
    pub timeout: Duration,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionRequest {
    pub command: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandResult {
    pub stdout: String,
    pub stderr: String,
    pub status: CommandStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandStatus {
    Exited(i32),
    TimedOut,
    LaunchFailed(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandLaunchError {
    pub message: String,
}

pub trait CommandRunner: Clone + Send + Sync + 'static {
    fn run(&self, request: CommandRequest) -> CommandResult;
    fn launch(&self, request: ActionRequest) -> Result<(), CommandLaunchError>;
}

#[derive(Debug, Clone, Default)]
pub struct ShellCommandRunner;

impl CommandRunner for ShellCommandRunner {
    fn run(&self, request: CommandRequest) -> CommandResult {
        let capture = match CaptureFiles::new() {
            Ok(capture) => capture,
            Err(error) => {
                return CommandResult {
                    stdout: String::new(),
                    stderr: String::new(),
                    status: CommandStatus::LaunchFailed(format!(
                        "failed to create capture files: {error}"
                    )),
                };
            }
        };

        let stdout = match File::create(&capture.stdout_path) {
            Ok(file) => file,
            Err(error) => {
                return CommandResult {
                    stdout: String::new(),
                    stderr: String::new(),
                    status: CommandStatus::LaunchFailed(format!(
                        "failed to capture stdout: {error}"
                    )),
                };
            }
        };
        let stderr = match File::create(&capture.stderr_path) {
            Ok(file) => file,
            Err(error) => {
                return CommandResult {
                    stdout: String::new(),
                    stderr: String::new(),
                    status: CommandStatus::LaunchFailed(format!(
                        "failed to capture stderr: {error}"
                    )),
                };
            }
        };

        let mut child = match shell_command(&request.command)
            .stdin(Stdio::null())
            .stdout(Stdio::from(stdout))
            .stderr(Stdio::from(stderr))
            .spawn()
        {
            Ok(child) => child,
            Err(error) => {
                return CommandResult {
                    stdout: String::new(),
                    stderr: String::new(),
                    status: CommandStatus::LaunchFailed(format!(
                        "failed to launch command: {error}"
                    )),
                };
            }
        };

        let started = Instant::now();
        let status = loop {
            match child.try_wait() {
                Ok(Some(status)) => break CommandStatus::Exited(status.code().unwrap_or(-1)),
                Ok(None) if started.elapsed() >= request.timeout => {
                    let _ = child.kill();
                    let _ = child.wait();
                    break CommandStatus::TimedOut;
                }
                Ok(None) => thread::sleep(Duration::from_millis(20)),
                Err(error) => {
                    let _ = child.kill();
                    let _ = child.wait();
                    break CommandStatus::LaunchFailed(format!(
                        "failed to monitor command: {error}"
                    ));
                }
            }
        };

        CommandResult {
            stdout: capture.read_stdout(),
            stderr: capture.read_stderr(),
            status,
        }
    }

    fn launch(&self, request: ActionRequest) -> Result<(), CommandLaunchError> {
        let mut child = shell_command(&request.command)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|error| CommandLaunchError {
                message: format!("failed to launch action: {error}"),
            })?;

        thread::spawn(move || {
            let _ = child.wait();
        });

        Ok(())
    }
}

fn shell_command(command: &str) -> Command {
    let shell = env::var_os("SHELL").unwrap_or_else(|| "/bin/sh".into());
    let mut process = Command::new(shell);
    process.arg("-lc").arg(command);
    process
}

#[derive(Debug)]
struct CaptureFiles {
    stdout_path: PathBuf,
    stderr_path: PathBuf,
}

impl CaptureFiles {
    fn new() -> std::io::Result<Self> {
        let id = CAPTURE_COUNTER.fetch_add(1, Ordering::Relaxed);
        let base = env::temp_dir().join(format!("hud-{}-{id}", std::process::id()));
        Ok(Self {
            stdout_path: base.with_extension("stdout"),
            stderr_path: base.with_extension("stderr"),
        })
    }

    fn read_stdout(&self) -> String {
        read_capture_file(&self.stdout_path)
    }

    fn read_stderr(&self) -> String {
        read_capture_file(&self.stderr_path)
    }
}

impl Drop for CaptureFiles {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.stdout_path);
        let _ = fs::remove_file(&self.stderr_path);
    }
}

fn read_capture_file(path: &PathBuf) -> String {
    fs::read_to_string(path).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct FakeRunner {
        result: CommandResult,
    }

    impl CommandRunner for FakeRunner {
        fn run(&self, _request: CommandRequest) -> CommandResult {
            self.result.clone()
        }

        fn launch(&self, _request: ActionRequest) -> Result<(), CommandLaunchError> {
            Ok(())
        }
    }

    #[test]
    fn fake_runner_can_stand_in_for_command_execution() {
        let runner = FakeRunner {
            result: CommandResult {
                stdout: "ok".into(),
                stderr: String::new(),
                status: CommandStatus::Exited(0),
            },
        };

        let result = runner.run(CommandRequest {
            command: "ignored".into(),
            timeout: Duration::from_secs(1),
        });

        assert_eq!(result.stdout, "ok");
        assert_eq!(result.status, CommandStatus::Exited(0));
    }
}
