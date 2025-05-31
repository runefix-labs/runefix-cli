//! Execute the `truncate` command: trim text based on display width constraint.

use crate::config::Context;
use crate::i18n::keys::TitleKey;
use crate::style::print::*;
use runefix_core::{WidthPolicy, truncate_by_width_with_policy};

/// Truncate the input string to a maximum display width using the provided policy.
///
/// This is useful for ensuring strings fit within a fixed-width UI element or terminal display.
///
/// # Arguments
/// * `ctx` - Global application context (for i18n, etc.).
/// * `input` - Text to be truncated.
/// * `max_width` - Maximum allowed display width in columns.
/// * `policy` - Display width strategy (e.g., terminal-aware, markdown-safe).
/// * `verbose` - Enable verbose output.
pub fn run_truncate(
    ctx: &Context,
    input: &str,
    max_width: usize,
    policy: WidthPolicy,
    verbose: bool,
) {
    // Optional header (only in verbose mode)
    if verbose {
        // Display localized section title
        print_title(&ctx.t(TitleKey::TruncatedOutput));
    }

    // Apply truncation based on display width
    let result = truncate_by_width_with_policy(input, max_width, Some(&policy));

    // Print the truncated result
    println!("{result}");
}
