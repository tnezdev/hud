//! Output boundary shared only by the data-only examples, not the HUD runtime.
use std::{
    io::{self, Write},
    process::ExitCode,
};

/// Combine live-operation and non-printing restoration results before encoding.
pub fn after_restore(operation: io::Result<()>, restoration: io::Result<()>) -> Result<(), String> {
    match (operation, restoration) {
        (Ok(()), Ok(())) => Ok(()),
        (Err(error), Ok(())) => Err(error.to_string()),
        (Ok(()), Err(error)) => Err(format!("Failed to restore terminal: {error}")),
        (Err(error), Err(restore_error)) => Err(format!(
            "{error}; Failed to restore terminal: {restore_error}"
        )),
    }
}

/// Encode handled errors once, immediately before stderr. Parsing stays unchanged.
pub fn finish(result: Result<(), String>, stderr: &mut impl Write) -> ExitCode {
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            let mut escaped = String::with_capacity(error.len());
            for c in error.chars() {
                if c.is_control()
                    || matches!(
                        c,
                        '\u{061c}'
                            | '\u{200e}'
                            | '\u{200f}'
                            | '\u{2028}'..='\u{202e}'
                            | '\u{2066}'..='\u{2069}'
                    )
                {
                    escaped.extend(c.escape_default());
                } else {
                    escaped.push(c);
                }
            }
            // A broken stderr must still fail, without an unescaped fallback/panic.
            let _ = writeln!(stderr, "{escaped}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn restoration_failure_turns_success_into_an_encoded_failure() {
        let result = after_restore(Ok(()), Err(io::Error::other("\u{1b}[2J\n")));
        let mut stderr = Vec::new();
        assert_eq!(finish(result, &mut stderr), ExitCode::FAILURE);
        assert_eq!(stderr, b"Failed to restore terminal: \\u{1b}[2J\\n\n");
    }

    #[test]
    fn operation_and_restoration_errors_survive_and_are_encoded_together() {
        let result = after_restore(
            Err(io::Error::other("draw\u{9b}0m")),
            Err(io::Error::other("restore\u{202e}\n")),
        );
        let mut stderr = Vec::new();
        assert_eq!(finish(result, &mut stderr), ExitCode::FAILURE);
        assert_eq!(
            stderr,
            b"draw\\u{9b}0m; Failed to restore terminal: restore\\u{202e}\\n\n"
        );
    }

    #[test]
    fn successful_restoration_preserves_the_operation_result() {
        assert_eq!(after_restore(Ok(()), Ok(())), Ok(()));
        assert_eq!(
            after_restore(Err(io::Error::other("original failure")), Ok(())),
            Err("original failure".into())
        );
    }

    #[test]
    fn controls_bidi_and_line_separators_are_visible_escapes() {
        let controls = ('\0'..='\u{1f}')
            .chain('\u{7f}'..='\u{9f}')
            .chain(['\u{061c}', '\u{200e}', '\u{200f}'])
            .chain('\u{2028}'..='\u{202e}')
            .chain('\u{2066}'..='\u{2069}');
        for c in controls {
            let mut stderr = Vec::new();
            let status = finish(Err(format!("before{c}after")), &mut stderr);
            assert_eq!(status, ExitCode::FAILURE);
            assert_eq!(
                stderr,
                format!("before{}after\n", c.escape_default()).as_bytes()
            );
        }
    }

    #[test]
    fn ordinary_diagnostics_keep_printable_unicode_and_punctuation() {
        for error in [
            "",
            "expected an unsigned integer",
            "C:\\notes\\café — '旅' → failed",
        ] {
            let mut stderr = Vec::new();
            assert_eq!(finish(Err(error.into()), &mut stderr), ExitCode::FAILURE);
            assert_eq!(stderr, format!("{error}\n").as_bytes());
        }
    }

    #[test]
    fn path_and_io_error_text_cannot_add_terminal_commands_or_lines() {
        let path = "notes/\u{1b}[2J\u{202e}file.json";
        let cause = io::Error::new(io::ErrorKind::NotFound, "missing\r\n\t\u{9b}0m");
        let mut stderr = Vec::new();
        assert_eq!(
            finish(Err(format!("{path}: {cause}")), &mut stderr),
            ExitCode::FAILURE
        );
        assert_eq!(
            stderr,
            b"notes/\\u{1b}[2J\\u{202e}file.json: missing\\r\\n\\t\\u{9b}0m\n"
        );
    }

    #[test]
    fn success_does_not_write_a_diagnostic() {
        let mut stderr = Vec::new();
        assert_eq!(finish(Ok(()), &mut stderr), ExitCode::SUCCESS);
        assert!(stderr.is_empty());
    }

    #[test]
    fn broken_stderr_still_returns_failure_without_panicking() {
        struct BrokenWriter;
        impl Write for BrokenWriter {
            fn write(&mut self, _: &[u8]) -> io::Result<usize> {
                Err(io::Error::from(io::ErrorKind::BrokenPipe))
            }

            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }
        assert_eq!(
            finish(Err("\u{1b}[2Jfailure".into()), &mut BrokenWriter),
            ExitCode::FAILURE
        );
    }
}
