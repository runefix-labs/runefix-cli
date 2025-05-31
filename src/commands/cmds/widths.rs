//! Execute the `widths` command: compute width of each grapheme cluster using a specified policy.

use crate::config::Context;
use crate::i18n::keys::TitleKey;
use crate::style::print::*;
use runefix_core::{WidthPolicy, display_width_with_policy};
use unicode_segmentation::UnicodeSegmentation;

/// Compute and print the display width of each grapheme in the input string.
///
/// This function uses `UnicodeSegmentation` to divide the input into grapheme clusters,
/// then calculates the display width of each using the provided `WidthPolicy`.
///
/// # Arguments
/// * `ctx` - Global application context (used for i18n, theme, etc.).
/// * `input` - Raw input string to analyze.
/// * `policy` - Display width strategy (e.g., terminal, markdown).
/// * `verbose` - Enable verbose output.
pub fn run_widths(ctx: &Context, input: &str, policy: WidthPolicy, verbose: bool) {
    // Optional header (only in verbose mode)
    if verbose {
        // Display localized section title
        print_title(&ctx.t(TitleKey::WidthPerChar));
    }

    // Split input into Unicode grapheme clusters
    let graphemes = input.graphemes(true).collect::<Vec<&str>>();

    // Iterate through each grapheme and print its display width
    for g in graphemes {
        let width = display_width_with_policy(g, Some(&policy));
        println!("[{}] = {}", g, width);
    }
}
