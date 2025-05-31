//! Terminal utilities for checking stream types and styling output.

/// Returns `true` if stdout is a terminal (TTY).
///
/// Useful for deciding whether to print ANSI styling or fallback to plain output.
pub fn is_stdout_terminal() -> bool {
    atty::is(atty::Stream::Stdout)
}

/// Returns `true` if stdin is a terminal (TTY).
///
/// Useful for detecting whether input is piped or entered interactively.
pub fn is_stdin_terminal() -> bool {
    atty::is(atty::Stream::Stdin)
}

/// Wraps the given string in ANSI escape codes for bold styling.
///
/// Returns the styled string, only meaningful if printed to a TTY.
pub fn bold(s: &str) -> String {
    format!("\x1b[1m{s}\x1b[0m")
}
