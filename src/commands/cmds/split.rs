//! Execute the `split` command: split text into lines based on display width.

use crate::config::Context;
use crate::i18n::keys::{ReportKey, TitleKey};
use crate::style::print::*;
use runefix_core::{RuneDisplayWidth, WidthPolicy, split_by_width_with_policy};

/// Split input text into lines by cumulative display width and print each line.
///
/// This function breaks the input string into segments whose display width
/// does not exceed `max_width`, using a given display `policy`. Each line is printed
/// with its corresponding width, aligned for visual clarity.
///
/// # Arguments
/// * `ctx` - Global application context (for i18n and theming).
/// * `text` - Input string to split.
/// * `max_width` - Maximum allowed display width per line.
/// * `policy` - Display width strategy (e.g., terminal, markdown).
/// * `verbose` - Enable verbose output.
pub fn run_split(ctx: &Context, text: &str, max_width: usize, policy: WidthPolicy, verbose: bool) {
    // Optional header (only in verbose mode)
    if verbose {
        // Display localized section title
        print_title(&ctx.t(TitleKey::SplitLines));
    }

    // Perform width-aware splitting
    let lines = split_by_width_with_policy(text, max_width, Some(&policy));

    // Compute max line width for alignment
    let max_display_width = lines.iter().map(|line| line.width()).max().unwrap_or(0);

    // Print each line with padding and width info
    for (i, line) in lines.iter().enumerate() {
        let width = line.width();
        let pad = max_display_width - width;

        println!(
            "{}{:>2}:    [{}{}] (width = {})",
            ctx.t(ReportKey::SplitLine),
            i + 1,
            line,
            " ".repeat(pad),
            width
        );
    }
}
