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

    #[test]
    fn shell_runner_captures_stdout() {
        let runner = ShellCommandRunner;
        let result = runner.run(CommandRequest {
            command: "printf 'hello world'".into(),
            timeout: Duration::from_secs(5),
        });
        assert_eq!(result.stdout.trim(), "hello world");
        assert_eq!(result.status, CommandStatus::Exited(0));
    }

    #[test]
    fn shell_runner_captures_stderr() {
        let runner = ShellCommandRunner;
        let result = runner.run(CommandRequest {
            command: "printf 'err' >&2".into(),
            timeout: Duration::from_secs(5),
        });
        assert_eq!(result.stderr.trim(), "err");
        assert_eq!(result.status, CommandStatus::Exited(0));
    }

    #[test]
    fn shell_runner_preserves_nonzero_exit() {
        let runner = ShellCommandRunner;
        let result = runner.run(CommandRequest {
            command: "exit 42".into(),
            timeout: Duration::from_secs(5),
        });
        assert_eq!(result.status, CommandStatus::Exited(42));
    }

    #[test]
    fn shell_runner_reports_timeout() {
        let runner = ShellCommandRunner;
        let result = runner.run(CommandRequest {
            command: "sleep 10".into(),
            timeout: Duration::from_millis(100),
        });
        assert_eq!(result.status, CommandStatus::TimedOut);
    }

    #[test]
    fn shell_runner_reports_launch_failure() {
        let runner = ShellCommandRunner;
        let result = runner.run(CommandRequest {
            command: "this_command_does_not_exist_abc123".into(),
            timeout: Duration::from_secs(5),
        });
        // The shell itself exits, so we get a non-zero exit rather than LaunchFailed
        assert!(matches!(result.status, CommandStatus::Exited(_)));
    }
}

#[cfg(test)]
mod shell_runner_edge_tests {
    use super::*;
    use std::thread;

    /// Captures both stdout and stderr from a single command execution.
    #[test]
    fn run_captures_both_stdout_and_stderr_simultaneously() {
        let runner = ShellCommandRunner;
        let result = runner.run(CommandRequest {
            command: "printf 'out'; printf 'err' >&2".into(),
            timeout: Duration::from_secs(5),
        });

        assert_eq!(result.status, CommandStatus::Exited(0));
        assert!(
            result.stdout.contains("out"),
            "stdout was: {:?}",
            result.stdout
        );
        assert!(
            result.stderr.contains("err"),
            "stderr was: {:?}",
            result.stderr
        );
    }

    /// Captures stderr from a failing command (non-zero exit + stderr together).
    #[test]
    fn run_captures_stderr_alongside_nonzero_exit() {
        let runner = ShellCommandRunner;
        let result = runner.run(CommandRequest {
            command: "printf 'oops' >&2; exit 1".into(),
            timeout: Duration::from_secs(5),
        });

        assert_eq!(result.status, CommandStatus::Exited(1));
        assert!(
            result.stderr.contains("oops"),
            "stderr was: {:?}",
            result.stderr
        );
    }

    /// A silent successful command yields empty stdout and stderr.
    #[test]
    fn run_silent_command_produces_empty_output() {
        let runner = ShellCommandRunner;
        let result = runner.run(CommandRequest {
            command: "true".into(),
            timeout: Duration::from_secs(5),
        });

        assert_eq!(result.status, CommandStatus::Exited(0));
        assert_eq!(result.stdout, "");
        assert_eq!(result.stderr, "");
    }

    /// Timed-out commands produce TimedOut status. Partial stdout
    /// availability depends on OS file buffering and is not guaranteed,
    /// so we verify only the status result here.
    #[test]
    fn run_timeout_status_is_timed_out() {
        let runner = ShellCommandRunner;
        let result = runner.run(CommandRequest {
            command: "sleep 10".into(),
            timeout: Duration::from_millis(100),
        });

        assert_eq!(result.status, CommandStatus::TimedOut);
        // stdout/stderr may be empty or partial depending on OS buffering;
        // the important invariant is the status, not the output presence.
    }

    /// Concurrent run() calls isolate their temp-file capture paths —
    /// each thread reads only its own stdout.
    #[test]
    fn run_concurrent_calls_isolate_temp_files() {
        let runner = ShellCommandRunner;
        let handles: Vec<_> = (0..4)
            .map(|i| {
                let runner = runner.clone();
                thread::spawn(move || {
                    let result = runner.run(CommandRequest {
                        command: format!("printf 'thread{i}'"),
                        timeout: Duration::from_secs(5),
                    });
                    assert_eq!(result.status, CommandStatus::Exited(0));
                    assert!(
                        result.stdout.contains(&format!("thread{i}")),
                        "stdout was: {:?}",
                        result.stdout
                    );
                })
            })
            .collect();

        for handle in handles {
            handle.join().expect("thread panicked");
        }
    }

    /// launch() returns Ok immediately without waiting for the process to exit.
    #[test]
    fn launch_returns_immediately_without_blocking() {
        let runner = ShellCommandRunner;
        let start = Instant::now();
        let result = runner.launch(ActionRequest {
            command: "sleep 5".into(),
        });

        assert!(result.is_ok());
        assert!(
            start.elapsed() < Duration::from_secs(2),
            "launch took {:?}, expected near-instant return",
            start.elapsed()
        );
    }

    // Launch failure coverage note:
    //
    // ShellCommandRunner::run can produce LaunchFailed in four code paths:
    //   1. CaptureFiles::new() fails (temp dir unavailable)
    //   2. File::create fails for stdout/stderr capture files
    //   3. Command::spawn fails (shell binary missing/unexecutable)
    //   4. child.try_wait returns an error during polling
    //
    // Path 3 is the most interesting integration case, but testing it requires
    // overriding $SHELL to a nonexistent binary. env::set_var is unsafe in
    // Rust 2024+ and unsound in multi-threaded tests, and shell_command() is
    // a private free function with no dependency injection point for the shell
    // path. The existing shell_runner_reports_launch_failure test in the parent
    // module verifies that a nonexistent *command* (not shell) produces
    // Exited(nonzero) rather than LaunchFailed, which documents the actual
    // behavior: $SHELL -lc <bad-command> exits with a shell error code.
    //
    // Full LaunchFailed integration coverage would require either injecting
    // the shell path or serializing env mutation — out of scope for V1.
}
