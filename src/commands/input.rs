//! Input resolution logic for CLI commands.

use crate::config::Context;
use crate::i18n::keys::ErrorKey;
use crate::style::term::is_stdin_terminal;
use std::io::{self, Read};
use std::process::exit;

/// Resolves the input text from either CLI argument or stdin.
pub fn resolve_input(ctx: &Context, text: Option<String>) -> String {
    // Use CLI argument if available
    if let Some(t) = text {
        return t;
    }

    let mut buf = String::new();

    // If no input and stdin is a terminal, show error and exit
    if is_stdin_terminal() {
        eprintln!("{}", ctx.t(ErrorKey::InputTextNoProvided));
        exit(1);
    }

    // Read entire stdin into a string buffer
    io::stdin()
        .read_to_string(&mut buf)
        .expect("Failed to read from stdin");

    // Remove trailing newline for cleaner output
    buf.trim_end_matches('\n').to_string()
}
