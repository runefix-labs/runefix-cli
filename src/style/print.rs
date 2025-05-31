//! Output helpers for styled CLI formatting.
//!
//! This module provides reusable utilities to print titles,
//! apply styling, and maintain visual consistency across commands.

use crate::style::{consts::WIDTH_LINE, term::is_stdout_terminal};

/// Prints a styled section title with a horizontal divider.
///
/// Example output (when stdout is a terminal):
/// ```
/// 📏 Display Width
/// ─────────────────────────────────────────
/// ```
///
/// - Applies bold styling if output is a terminal
/// - Falls back to plain text otherwise
pub fn print_title(label: &str) {
    if is_stdout_terminal() {
        // Bold output for TTY environments
        println!("\n\x1b[1m{label}\x1b[0m");
    } else {
        // Plain output for non-TTY environments
        println!("{label}");
    }

    // Print horizontal divider line
    println!("{}", "─".repeat(WIDTH_LINE));
}
